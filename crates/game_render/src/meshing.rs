use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::Mesh,
};
use game_core::{AIR, BLOCKS, GRASS, SECTION_SIZE, is_opaque};

const SIZE: usize = SECTION_SIZE as usize;
const PADDED_SIZE: usize = SIZE + 2;

/// Builds at most one greedy mesh per visible block material in a 16³ section.
///
/// `sample` receives world block positions, including a one-block border, so
/// adjacent sections must be present in the caller's snapshot for seam culling.
/// Air and unknown block IDs produce no geometry. Opaque neighbors hide faces;
/// matching transparent neighbors also hide their shared internal faces.
#[must_use]
pub fn mesh_section(section: [i32; 3], sample: impl Fn([i32; 3]) -> u8) -> Vec<(u8, Mesh)> {
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

    let mut builders: [MeshBuilder; BLOCKS.len()] = std::array::from_fn(|_| MeshBuilder::default());
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
                let mut mask = [AIR; SIZE * SIZE];
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
                            mask[u + v * SIZE] = id;
                        }
                    }
                }

                for v in 0..SIZE {
                    let mut u = 0;
                    while u < SIZE {
                        let id = mask[u + v * SIZE];
                        if id == AIR {
                            u += 1;
                            continue;
                        }
                        let mut width = 1;
                        while u + width < SIZE && mask[u + width + v * SIZE] == id {
                            width += 1;
                        }
                        let mut height = 1;
                        while v + height < SIZE
                            && (0..width).all(|offset| mask[u + offset + (v + height) * SIZE] == id)
                        {
                            height += 1;
                        }
                        builders[usize::from(id)].quad(FaceRectangle {
                            axis,
                            positive,
                            u_axis,
                            v_axis,
                            plane: slice + i32::from(positive),
                            u,
                            v,
                            width,
                            height,
                        });
                        for row in v..v + height {
                            mask[u + row * SIZE..u + width + row * SIZE].fill(AIR);
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
        .filter_map(|(id, builder)| builder.finish(id as u8).map(|mesh| (id as u8, mesh)))
        .collect()
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
}

#[derive(Default)]
struct MeshBuilder {
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
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
        // cross(u,v) is -X, -Y, +Z for the bases above. Reverse triangle
        // winding exactly when that basis points into the block.
        let indices = if positive == (axis == 2) {
            [0, 1, 2, 0, 2, 3]
        } else {
            [0, 2, 1, 0, 3, 2]
        };
        self.indices.extend(indices.map(|index| first + index));
    }

    fn finish(self, id: u8) -> Option<Mesh> {
        if self.positions.is_empty() {
            return None;
        }
        // The fixed material-per-block API still allows the horizontal grass
        // surface to read greener than its earthy sides, without an atlas or
        // extra draws. Every vertex in a merged face gets the same tint.
        let grass_colors = (id == GRASS).then(|| {
            self.normals
                .iter()
                .map(|normal| {
                    if normal[1] > 0.0 {
                        [0.62, 1.0, 0.68, 1.0]
                    } else {
                        [1.0; 4]
                    }
                })
                .collect::<Vec<_>>()
        });
        let mut mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::default(),
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, self.positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, self.normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, self.uvs)
        .with_inserted_indices(Indices::U32(self.indices));
        if let Some(colors) = grass_colors {
            mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
        Some(mesh)
    }
}
