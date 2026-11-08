use crate::{BlockTextures, MATERIAL_COUNT};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::Mesh,
};
use game_core::{AIR, BLOCKS, SECTION_SIZE, is_opaque};

const SIZE: usize = SECTION_SIZE as usize;
const PADDED_SIZE: usize = SIZE + 2;

/// Builds at most one greedy mesh per visible render material in a 16³ section.
///
/// `sample` receives world block positions, including a one-block border, so
/// adjacent sections must be present in the caller's snapshot for seam culling.
/// Air and unknown block IDs produce no geometry. Opaque neighbors hide faces;
/// matching transparent neighbors also hide their shared internal faces.
#[must_use]
pub fn mesh_section(section: [i32; 3], sample: impl Fn([i32; 3]) -> u8) -> Vec<(u8, Mesh)> {
    mesh_section_with_textures(section, sample, &BlockTextures::default())
}

pub fn mesh_section_with_textures(
    section: [i32; 3],
    sample: impl Fn([i32; 3]) -> u8,
    textures: &BlockTextures,
) -> Vec<(u8, Mesh)> {
    // Cache the border once: the six sweeps reuse every block many times, while
    // a world sample can involve a hash lookup and an edited-block lookup.
    let mut blocks = [AIR; PADDED_SIZE * PADDED_SIZE * PADDED_SIZE];
    let origin = section.map(|coordinate| i64::from(coordinate) * i64::from(SECTION_SIZE));
    for y in -1..=SECTION_SIZE {
        for z in -1..=SECTION_SIZE {
            for x in -1..=SECTION_SIZE {
                let local = [x, y, z];
                let world = std::array::from_fn(|axis| {
                    i32::try_from(origin[axis] + i64::from(local[axis])).ok()
                });
                if let [Some(x), Some(y), Some(z)] = world {
                    let id = sample([x, y, z]);
                    if usize::from(id) < BLOCKS.len() {
                        blocks[padded_index(local)] = id;
                    }
                }
            }
        }
    }

    let mut builders: [MeshBuilder; MATERIAL_COUNT] =
        std::array::from_fn(|_| MeshBuilder::default());
    for axis in 0..3 {
        // Vertical faces use texture v for world y; texture grain and grass
        // layers consequently stay upright on both x and z sides.
        let (u_axis, v_axis) = match axis {
            0 => (2, 1),
            1 => (0, 2),
            _ => (0, 1),
        };
        for positive in [false, true] {
            for slice in 0..SECTION_SIZE {
                let mut mask = [FaceCell::default(); SIZE * SIZE];
                for v in 0..SIZE {
                    for u in 0..SIZE {
                        let mut local = [0; 3];
                        local[axis] = slice;
                        local[u_axis] = u as i32;
                        local[v_axis] = v as i32;
                        let id = blocks[padded_index(local)];
                        if id == AIR {
                            continue;
                        }
                        local[axis] += if positive { 1 } else { -1 };
                        let neighbor = blocks[padded_index(local)];
                        if !is_opaque(neighbor) && neighbor != id {
                            mask[u + v * SIZE] = FaceCell {
                                id,
                                ao: corner_ao(&blocks, local, u_axis, v_axis),
                            };
                        }
                    }
                }

                for v in 0..SIZE {
                    let mut u = 0;
                    while u < SIZE {
                        let cell = mask[u + v * SIZE];
                        let id = cell.id;
                        if id == AIR {
                            u += 1;
                            continue;
                        }
                        let mut width = 1;
                        while cell.uniform()
                            && u + width < SIZE
                            && mask[u + width + v * SIZE] == cell
                        {
                            width += 1;
                        }
                        let mut height = 1;
                        while cell.uniform()
                            && v + height < SIZE
                            && (0..width)
                                .all(|offset| mask[u + offset + (v + height) * SIZE] == cell)
                        {
                            height += 1;
                        }
                        let material = textures.layer(id, axis, positive);
                        builders[usize::from(material)].quad(FaceRectangle {
                            axis,
                            positive,
                            u_axis,
                            v_axis,
                            plane: slice + i32::from(positive),
                            u,
                            v,
                            width,
                            height,
                            ao: cell.ao,
                        });
                        for row in v..v + height {
                            mask[u + row * SIZE..u + width + row * SIZE].fill(FaceCell::default());
                        }
                        u += width;
                    }
                }
            }
        }
    }

    builders
        .into_iter()
        .enumerate()
        .filter_map(|(id, builder)| builder.finish().map(|mesh| (id as u8, mesh)))
        .collect()
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct FaceCell {
    id: u8,
    ao: [u8; 4],
}
impl FaceCell {
    fn uniform(self) -> bool {
        self.ao.iter().all(|a| *a == self.ao[0])
    }
}

fn corner_ao(blocks: &[u8], outside: [i32; 3], u: usize, v: usize) -> [u8; 4] {
    [(-1, -1), (1, -1), (1, 1), (-1, 1)].map(|(du, dv)| {
        let mut side_u = outside;
        side_u[u] += du;
        let mut side_v = outside;
        side_v[v] += dv;
        let mut corner = side_u;
        corner[v] += dv;
        let a = u8::from(is_opaque(blocks[padded_index(side_u)]));
        let b = u8::from(is_opaque(blocks[padded_index(side_v)]));
        let c = u8::from(is_opaque(blocks[padded_index(corner)]));
        if a == 1 && b == 1 { 0 } else { 3 - a - b - c }
    })
}

fn padded_index([x, y, z]: [i32; 3]) -> usize {
    (x + 1) as usize + (z + 1) as usize * PADDED_SIZE + (y + 1) as usize * PADDED_SIZE * PADDED_SIZE
}

struct FaceRectangle {
    axis: usize,
    positive: bool,
    u_axis: usize,
    v_axis: usize,
    plane: i32,
    u: usize,
    v: usize,
    width: usize,
    height: usize,
    ao: [u8; 4],
}

#[derive(Default)]
struct MeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
}

impl MeshBuilder {
    fn quad(&mut self, rectangle: FaceRectangle) {
        let FaceRectangle {
            axis,
            positive,
            u_axis,
            v_axis,
            plane,
            u,
            v,
            width,
            height,
            ao,
        } = rectangle;
        let first = self.positions.len() as u32;
        let mut point = [0.0; 3];
        point[axis] = plane as f32;
        point[u_axis] = u as f32;
        point[v_axis] = v as f32;
        self.positions.push(point);
        point[u_axis] += width as f32;
        self.positions.push(point);
        point[v_axis] += height as f32;
        self.positions.push(point);
        point[u_axis] -= width as f32;
        self.positions.push(point);

        let mut normal = [0.0; 3];
        normal[axis] = if positive { 1.0 } else { -1.0 };
        self.normals.extend([normal; 4]);
        self.uvs.extend([
            [0.0, height as f32],
            [width as f32, height as f32],
            [width as f32, 0.0],
            [0.0, 0.0],
        ]);
        self.colors.extend(ao.map(|a| {
            let light = 0.76 + f32::from(a) * 0.08;
            [light, light, light, 1.]
        }));
        // cross(u,v) is -X, -Y, +Z for the bases above. Reverse triangle
        // winding exactly when that basis points into the block.
        let mut indices =
            if u16::from(ao[0]) + u16::from(ao[2]) > u16::from(ao[1]) + u16::from(ao[3]) {
                [0, 1, 3, 1, 2, 3]
            } else {
                [0, 1, 2, 0, 2, 3]
            };
        if positive != (axis == 2) {
            indices.swap(1, 2);
            indices.swap(4, 5);
        }
        self.indices.extend(indices.map(|index| first + index));
    }

    fn finish(self) -> Option<Mesh> {
        if self.positions.is_empty() {
            return None;
        }
        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, self.colors)
        .with_inserted_indices(Indices::U32(self.indices));
        Some(mesh)
    }
}
