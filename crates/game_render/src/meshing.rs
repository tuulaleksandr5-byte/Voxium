use crate::{BlockTextures, MATERIAL_COUNT};
use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::Mesh,
};
use game_core::{
    AIR, BLOCKS, BlockId, SECTION_SIZE, WATER,
    fluids::{fluid_of, surface_height},
    is_flat, is_fluid, is_half, is_invisible, is_opaque, is_plant, is_water,
};

const SIZE: usize = SECTION_SIZE as usize;
const PADDED_SIZE: usize = SIZE + 2;

/// Builds at most one greedy mesh per visible render material in a 16³ section.
///
/// `sample` receives world block positions, including a one-block border, so
/// adjacent sections must be present in the caller's snapshot for seam culling.
/// Air and unknown block IDs produce no geometry. Opaque neighbors hide faces;
/// matching transparent neighbors also hide their shared internal faces.
#[must_use]
pub fn mesh_section(section: [i32; 3], sample: impl Fn([i32; 3]) -> BlockId) -> Vec<(u16, Mesh)> {
    mesh_section_with_textures(section, sample, &BlockTextures::default())
}

pub fn mesh_section_with_textures(
    section: [i32; 3],
    sample: impl Fn([i32; 3]) -> BlockId,
    textures: &BlockTextures,
) -> Vec<(u16, Mesh)> {
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
                        // Источник воды сливается в крупные грани; остальные жидкости,
                        // растения и плиты строятся отдельно ниже.
                        if id == AIR
                            || is_plant(id)
                            || is_half(id)
                            || is_invisible(id)
                            || (is_fluid(id) && id != WATER)
                        {
                            continue;
                        }
                        local[axis] += if positive { 1 } else { -1 };
                        let neighbor = blocks[padded_index(local)];
                        let merged_water = id == WATER && is_water(neighbor);
                        if !is_opaque(neighbor) && neighbor != id && !merged_water {
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

    // Растения, плиты и жидкости строятся отдельно от greedy-граней. Сдвиг
    // растений зависит только от мировых координат и не «прыгает» при
    // перестройке секции.
    for y in 0..SECTION_SIZE {
        for z in 0..SECTION_SIZE {
            for x in 0..SECTION_SIZE {
                let local = [x, y, z];
                let id = blocks[padded_index(local)];
                if is_invisible(id) {
                    continue;
                }
                if is_flat(id) {
                    let material = textures.layer(id, 1, true);
                    builders[usize::from(material)].flat(local, game_core::is_rail_x(id));
                    continue;
                }
                if let Some(facing) = game_core::is_wall_torch(id)
                    .then(|| game_core::facing_of(id))
                    .flatten()
                {
                    let [dx, _, dz] = facing.offset();
                    let material = textures.layer(id, 0, true);
                    builders[usize::from(material)].lean_cross(local, [dx as f32, dz as f32]);
                    continue;
                }
                if is_plant(id) {
                    let world = std::array::from_fn(|axis| origin[axis] + i64::from(local[axis]));
                    let material = textures.layer(id, 0, true);
                    // Факелы и огонь стоят строго по центру клетки.
                    let hash = if matches!(
                        id,
                        game_core::TORCH
                            | game_core::WALL_TORCH
                            | game_core::FIRE
                            | game_core::SOUL_FIRE
                    ) {
                        0x8080_8080_8080_8080
                    } else {
                        leaf_hash(world)
                    };
                    builders[usize::from(material)].cross(local, hash);
                    continue;
                }
                if is_fluid(id) && id != WATER {
                    fluid_cell(&mut builders, textures, &blocks, local, id);
                    continue;
                }
                if is_half(id) {
                    // Плита: половина куба. Верх виден всегда, бока прячутся за
                    // непрозрачным соседом или другой плитой.
                    for axis in 0..3 {
                        for positive in [false, true] {
                            let mut neighbor = local;
                            neighbor[axis] += if positive { 1 } else { -1 };
                            let other = blocks[padded_index(neighbor)];
                            let hidden = match (axis, positive) {
                                (1, true) => false,
                                (1, false) => is_opaque(other),
                                _ => is_opaque(other) || is_half(other),
                            };
                            if !hidden {
                                let material = textures.layer(id, axis, positive);
                                builders[usize::from(material)].slab_face(local, axis, positive);
                            }
                        }
                    }
                    continue;
                }
            }
        }
    }

    builders
        .into_iter()
        .enumerate()
        .filter_map(|(id, builder)| builder.finish().map(|mesh| (id as u16, mesh)))
        .collect()
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
struct FaceCell {
    id: BlockId,
    ao: [u8; 4],
}
impl FaceCell {
    fn uniform(self) -> bool {
        self.ao.iter().all(|a| *a == self.ao[0])
    }
}

fn corner_ao(blocks: &[BlockId], outside: [i32; 3], u: usize, v: usize) -> [u8; 4] {
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

/// Течение воды и вся лава: коробка высотой по уровню жидкости. Грани между
/// клетками одной жидкости не рисуются; ступенька между уровнями показывается
/// полоской на боку более высокой клетки.
fn fluid_cell(
    builders: &mut [MeshBuilder],
    textures: &BlockTextures,
    blocks: &[BlockId],
    local: [i32; 3],
    id: BlockId,
) {
    let Some((kind, _)) = fluid_of(id) else {
        return;
    };
    let same = |other: BlockId| fluid_of(other).is_some_and(|(k, _)| k == kind);
    let at = |dx: i32, dy: i32, dz: i32| {
        blocks[padded_index([local[0] + dx, local[1] + dy, local[2] + dz])]
    };
    let height = surface_height(id, at(0, 1, 0));
    let origin = [local[0] as f32, local[1] as f32, local[2] as f32];
    let mut emit = |axis: usize, positive: bool, low: f32, high: f32| {
        let material = textures.layer(id, axis, positive);
        builders[usize::from(material)].fluid_face(
            [origin[0], origin[1] + low, origin[2]],
            [origin[0] + 1.0, origin[1] + high, origin[2] + 1.0],
            axis,
            positive,
        );
    };
    let above = at(0, 1, 0);
    if !is_opaque(above) && !same(above) {
        emit(1, true, height, height);
    }
    let below = at(0, -1, 0);
    if !is_opaque(below) && !same(below) {
        emit(1, false, 0.0, 0.0);
    }
    for (axis, positive) in [(0, false), (0, true), (2, false), (2, true)] {
        let step = if positive { 1 } else { -1 };
        let (dx, dz) = if axis == 0 { (step, 0) } else { (0, step) };
        let neighbor = at(dx, 0, dz);
        if is_opaque(neighbor) {
            continue;
        }
        let floor = if same(neighbor) {
            let lower = surface_height(neighbor, at(dx, 1, dz));
            if lower >= height - 1e-4 {
                continue;
            }
            lower
        } else {
            0.0
        };
        emit(axis, positive, floor, height);
    }
}

fn leaf_hash([x, y, z]: [i64; 3]) -> u64 {
    let mut value = (x as u64).wrapping_mul(0x9e37_79b9_7f4a_7c15)
        ^ (y as u64).wrapping_mul(0xc2b2_ae3d_27d4_eb4f)
        ^ (z as u64).wrapping_mul(0xd1b5_4a32_d192_ed03);
    value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^ (value >> 31)
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

    /// Одна грань плиты высотой в полблока; бока берут нижнюю половину тайла.
    fn slab_face(&mut self, local: [i32; 3], axis: usize, positive: bool) {
        let center = [
            local[0] as f32 + 0.5,
            local[1] as f32 + 0.25,
            local[2] as f32 + 0.5,
        ];
        let (v_top, v_bottom) = if axis == 1 { (0.0, 1.0) } else { (0.5, 1.0) };
        self.box_face(center, [0.5, 0.25, 0.5], axis, positive, (v_top, v_bottom));
    }

    /// Грань жидкости: коробка `min`…`max` внутри клетки секции. Боковые грани
    /// берут ту часть тайла, которая соответствует их высоте над дном клетки.
    fn fluid_face(&mut self, min: [f32; 3], max: [f32; 3], axis: usize, positive: bool) {
        let center = std::array::from_fn(|i| (min[i] + max[i]) * 0.5);
        let half = std::array::from_fn(|i| (max[i] - min[i]) * 0.5);
        let floor = min[1] - min[1].floor();
        let v = if axis == 1 {
            (0.0, 1.0)
        } else {
            (1.0 - (max[1] - min[1].floor()), 1.0 - floor)
        };
        self.box_face(center, half, axis, positive, v);
    }

    /// Грань коробки с центром `center` и полуразмерами `half`; `v` — значения
    /// текстурной координаты на верхней и нижней кромках грани.
    fn box_face(
        &mut self,
        center: [f32; 3],
        half: [f32; 3],
        axis: usize,
        positive: bool,
        v: (f32, f32),
    ) {
        let mut normal = [0.0_f32; 3];
        normal[axis] = if positive { 1.0 } else { -1.0 };
        let up: [f32; 3] = match (axis, positive) {
            (1, true) => [0.0, 0.0, -1.0],
            (1, false) => [0.0, 0.0, 1.0],
            _ => [0.0, 1.0, 0.0],
        };
        let across = [
            up[1] * normal[2] - up[2] * normal[1],
            up[2] * normal[0] - up[0] * normal[2],
            up[0] * normal[1] - up[1] * normal[0],
        ];
        let extent =
            |direction: [f32; 3]| -> f32 { (0..3).map(|i| direction[i].abs() * half[i]).sum() };
        let (reach_across, reach_up, reach_out) = (extent(across), extent(up), extent(normal));
        let first = self.positions.len() as u32;
        for (a, b) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
            self.positions.push(std::array::from_fn(|i| {
                center[i]
                    + normal[i] * reach_out
                    + across[i] * a * reach_across
                    + up[i] * b * reach_up
            }));
        }
        self.normals.extend([normal; 4]);
        self.uvs
            .extend([[0.0, v.1], [1.0, v.1], [1.0, v.0], [0.0, v.0]]);
        self.colors.extend([[1.0, 1.0, 1.0, 1.0]; 4]);
        self.indices
            .extend([0, 1, 2, 0, 2, 3].map(|index| first + index));
    }

    /// Рисунок на полу клетки: одна горизонтальная плоскость чуть выше пола.
    fn flat(&mut self, local: [i32; 3], rotated: bool) {
        let (x, y, z) = (local[0] as f32, local[1] as f32 + 0.03, local[2] as f32);
        let first = self.positions.len() as u32;
        self.positions.extend([
            [x, y, z],
            [x, y, z + 1.0],
            [x + 1.0, y, z + 1.0],
            [x + 1.0, y, z],
        ]);
        self.normals.extend([[0.0, 1.0, 0.0]; 4]);
        // Рельсы вдоль X — тот же рисунок, повёрнутый на четверть оборота.
        self.uvs.extend(if rotated {
            [[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]
        } else {
            [[0.0, 0.0], [0.0, 1.0], [1.0, 1.0], [1.0, 0.0]]
        });
        self.colors.extend([[1.0, 1.0, 1.0, 1.0]; 4]);
        self.indices
            .extend([0, 1, 2, 0, 2, 3].map(|index| first + index));
    }

    /// Настенный факел: основание прижато к стене, верх наклонён наружу.
    /// `outward` — горизонтальный единичный вектор от стены.
    fn lean_cross(&mut self, local: [i32; 3], outward: [f32; 2]) {
        let base = [
            local[0] as f32 + 0.5 - outward[0] * 0.34,
            local[2] as f32 + 0.5 - outward[1] * 0.34,
        ];
        let top = [base[0] + outward[0] * 0.3, base[1] + outward[1] * 0.3];
        let (low, high) = (local[1] as f32 + 0.2, local[1] as f32 + 1.0);
        let half = 0.36;
        for (dx, dz) in [(half, half), (half, -half)] {
            let first = self.positions.len() as u32;
            self.positions.extend([
                [base[0] - dx, low, base[1] - dz],
                [base[0] + dx, low, base[1] + dz],
                [top[0] + dx, high, top[1] + dz],
                [top[0] - dx, high, top[1] - dz],
            ]);
            self.normals.extend([[0.0, 1.0, 0.0]; 4]);
            self.uvs
                .extend([[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]);
            self.colors.extend([[1.0, 1.0, 1.0, 1.0]; 4]);
            self.indices
                .extend([0, 1, 2, 0, 2, 3].map(|index| first + index));
        }
    }

    /// Растение: две вертикальные плоскости крест-накрест с небольшим сдвигом,
    /// чтобы цветы на лугу не стояли по линейке.
    fn cross(&mut self, local: [i32; 3], hash: u64) {
        let unit = |shift: u32| ((hash >> shift) & 0xff) as f32 / 255.0;
        let x = local[0] as f32 + 0.5 + (unit(0) - 0.5) * 0.3;
        let z = local[2] as f32 + 0.5 + (unit(8) - 0.5) * 0.3;
        let y = local[1] as f32;
        let half = 0.36;
        for (dx, dz) in [(half, half), (half, -half)] {
            let first = self.positions.len() as u32;
            self.positions.extend([
                [x - dx, y, z - dz],
                [x + dx, y, z + dz],
                [x + dx, y + 1.0, z + dz],
                [x - dx, y + 1.0, z - dz],
            ]);
            self.normals.extend([[0.0, 1.0, 0.0]; 4]);
            self.uvs
                .extend([[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]]);
            self.colors.extend([[1.0, 1.0, 1.0, 1.0]; 4]);
            self.indices
                .extend([0, 1, 2, 0, 2, 3].map(|index| first + index));
        }
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
