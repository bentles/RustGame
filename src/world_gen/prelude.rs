use bevy::{ecs::component::Component, math::primitives::Direction3d};

pub const CHUNKS_PER_AXIS: usize = 15; // chunk constants
pub const SIZE: usize = 32;
pub const X_SIZE: usize = SIZE;
pub const Y_SIZE: usize = SIZE;
pub const Z_SIZE: usize = SIZE;

pub const LAST_X: usize = X_SIZE - 1;
pub const LAST_Y: usize = Y_SIZE - 1;
pub const LAST_Z: usize = Z_SIZE - 1;

pub const TOTAL_SIZE: usize = X_SIZE * Y_SIZE * Z_SIZE;
pub const LAST_XYZ: usize = TOTAL_SIZE - 1;
pub const PERLIN_SAMPLE_SIZE: f32 = 0.09;

pub const BLOCK_SIZE: f32 = 1.0;

#[derive(Clone)]
pub struct Index3D {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

pub struct MeshSide {
    pub vertices: [[f32; 3]; 4],
    pub normals: [[f32; 3]; 4],
    pub indices: [u32; 6],
}

#[derive(Default)]
pub struct MeshData {
    pub vertices: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub indices: Vec<u32>,
}

#[derive(Component, Clone)]
pub struct ChunkIndex(pub Index3D);

#[derive(Component, Clone)]
pub struct MeshFacingDirection(pub Direction3d);