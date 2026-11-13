//! Array-textured section meshes with tile-local UVs and soft corner occlusion.
//!
//! Mesh positions are section-local. Put the resulting render entities at
//! `section * 16`; their count depends on visible materials, never block count.

mod atlas;
mod block_model;
mod environment;
mod face_textures;
mod icons;
mod materials;
mod meshing;

pub use atlas::{ADDITIONAL_TILE_NAMES, BASE_TILE_NAMES, BlockAtlas, layer_of};
pub use block_model::{BlockModel, BlockModels, ModelElement, ModelFace};
pub use environment::gradient_sky;
pub use face_textures::{
    BlockTextures, FaceTextures, GRASS_BOTTOM, GRASS_TOP, MATERIAL_COUNT, WOOD_END,
};
pub use icons::{bake_atlas_icons, bake_block_icons};
pub use materials::{
    VoxelMaterial, VoxelRenderPlugin, atlas_array, atlas_array_filtered, atlas_png,
    create_materials, create_materials_from_atlas,
};
pub use meshing::{mesh_section, mesh_section_with_models, mesh_section_with_textures};

#[cfg(test)]
mod tests;
