//! Section meshes and original procedural pixel materials for the voxel world.
//!
//! Mesh positions are section-local. Put the resulting render entities at
//! `section * 16`; their count depends on visible materials, never block count.

mod materials;
mod meshing;

pub use materials::{GRASS_BOTTOM, GRASS_TOP, MATERIAL_COUNT, WOOD_END, create_materials};
pub use meshing::mesh_section;

#[cfg(test)]
mod tests;
