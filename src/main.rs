//! This example demonstrates how to create a custom mesh,
//! assign a custom UV mapping for a custom texture,
//! and how to change the UV mapping at run-time.

use bevy::prelude::*;
use bevy::render::camera;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};
use bevy_flycam::PlayerPlugin;
use noise::{NoiseFn, Perlin};

const CHUNKS_PER_AXIS: usize = 5; // chunk constants
const SIZE: usize = 32;
const X_SIZE: usize = SIZE;
const Y_SIZE: usize = SIZE;
const Z_SIZE: usize = SIZE;

const LAST_X: usize = X_SIZE - 1;
const LAST_Y: usize = Y_SIZE - 1;
const LAST_Z: usize = Z_SIZE - 1;

const TOTAL_SIZE: usize = X_SIZE * Y_SIZE * Z_SIZE;
const LAST_XYZ: usize = TOTAL_SIZE - 1;
const PERLIN_SAMPLE_SIZE: f32 = 0.09;

const BLOCK_SIZE: f32 = 1.0;

#[derive(Clone)]
struct Index3D {
    x: usize,
    y: usize,
    z: usize,
}

// fn shouldRender() {
//     // could be wrong but i'd imagine if the dot product of the camera facing direction and the normal of the side is negative then
//     // then I should render the side otherwise it is not needed
// }

#[derive(Component, Clone)]
struct ChunkIndex(Index3D);

#[derive(Component, Clone)]
struct MeshFacingDirection(Direction3d);

#[derive(Resource)]
struct PastCameraDirection(Direction3d);

fn main() {
    App::new()      
        .add_plugins(DefaultPlugins)
        .insert_resource(PastCameraDirection(Direction3d::X))
        .add_event::<CameraDirectionChangeEvent>()
        .add_systems(Startup, setup)
        .add_systems(Update, detect_camera_direction_changed)
        .add_systems(Update, on_camera_direction_change)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(PlayerPlugin)
        .run();
}

fn to_1d(x: usize, y: usize, z: usize) -> usize {
    x + y * X_SIZE + z * Y_SIZE * X_SIZE
}

fn index(arr: [f64; TOTAL_SIZE], x: usize, y: usize, z: usize) -> f64 {
    arr[to_1d(x, y, z)]
}

fn to_3d(n: usize) -> (usize, usize, usize) {
    let x: usize = n % X_SIZE;
    let y: usize = (n / (X_SIZE)) % Y_SIZE;
    let z: usize = n / (X_SIZE * Y_SIZE);
    (x, y, z)
}

fn index_reverse(n: usize) -> Index3D {
    let index = to_3d(n);
    Index3D {
        x: index.0,
        y: index.1,
        z: index.2,
    }
}

fn perlin(x_offset: f32, y_offset: f32, z_offset: f32) -> [f64; TOTAL_SIZE] {
    let perlin = Perlin::new(1234);
    core::array::from_fn(|n| {
        let Index3D { x, y, z } = index_reverse(n);
        let a = perlin.get([
            ((x as f32) * PERLIN_SAMPLE_SIZE + x_offset) as f64,
            ((y as f32) * PERLIN_SAMPLE_SIZE + y_offset) as f64,
            ((z as f32) * PERLIN_SAMPLE_SIZE + z_offset) as f64,
        ]);

        let b = perlin.get([
            ((x as f32) * 0.005 + x_offset) as f64,
            ((y as f32) * 0.005  + y_offset) as f64,
            ((z as f32) * 0.005  + z_offset) as f64,
        ]);

        a
    })
}

struct BlockData {
    visible_faces: Option<VisibleFaces>,
}

const EMPTY_BLOCK: BlockData = BlockData {
    visible_faces: None,
};

struct VisibleFaces {
    top: bool,
    bottom: bool,
    left: bool,
    right: bool,
    front: bool,
    back: bool,
}

fn is_air(x: f64) -> bool {
    x < 0.5
}

fn block_data_from_perlin(chunk: [f64; TOTAL_SIZE]) -> [BlockData; TOTAL_SIZE] {
    let mut output = [EMPTY_BLOCK; TOTAL_SIZE];

    for z in 0..=LAST_Z {
        for y in 0..=LAST_Y {
            for x in 0..=LAST_X {
                let i = to_1d(x, y, z);
                let perlin_val = chunk[i];
                if is_air(perlin_val) {
                    continue;
                }

                let faces = VisibleFaces {
                    right: (x == LAST_X) || is_air(index(chunk, x + 1, y, z)),
                    left: (x == 0) || is_air(index(chunk, x - 1, y, z)),
                    top: (y == LAST_Y) || is_air(index(chunk, x, y + 1, z)),
                    bottom: (y == 0) || is_air(index(chunk, x, y - 1, z)),
                    back: (z == LAST_Z) || is_air(index(chunk, x, y, z + 1)),
                    front: (z == 0) || is_air(index(chunk, x, y, z - 1)),
                };

                output[i].visible_faces = Some(faces);
            }
        }
    }
    output
}

fn setup(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for chunk_x in 0..CHUNKS_PER_AXIS {
        for chunk_y in 0..CHUNKS_PER_AXIS {
            for chunk_z in 0..CHUNKS_PER_AXIS {
                let chunk = ChunkIndex(Index3D {
                    x: chunk_x,
                    y: chunk_y,
                    z: chunk_z,
                });

                let x_chunk_offset = (chunk.0.x * X_SIZE) as f32;
                let y_chunk_offset = (chunk.0.y * Y_SIZE) as f32;
                let z_chunk_offset = (chunk.0.z * Z_SIZE) as f32;
                // Import the custom texture
                let perlin_chunk = perlin(
                    x_chunk_offset * PERLIN_SAMPLE_SIZE,
                    y_chunk_offset * PERLIN_SAMPLE_SIZE,
                    z_chunk_offset * PERLIN_SAMPLE_SIZE,
                );
                let block_data = block_data_from_perlin(perlin_chunk);                

                //let (x_block, y_block, z_block) = to_3d(n);

                let cube_sides = create_chunk_sides(
                    x_chunk_offset as f32 * BLOCK_SIZE,
                    y_chunk_offset as f32 * BLOCK_SIZE,
                    z_chunk_offset as f32 * BLOCK_SIZE,
                    BLOCK_SIZE,
                    block_data,
                );

                for (side, facing) in cube_sides {
                    // Create and save a handle to the mesh.

                    let side_mesh_handle: Handle<Mesh> = meshes.add(side);
                    //let clr = n as f32 / (LAST_XYZ as f32);
                    commands.spawn((
                        PbrBundle {
                            mesh: side_mesh_handle,
                            material: materials.add(StandardMaterial {
                                //base_color_texture: Some(custom_texture_handle),
                                //Color::rgb(clr, clr, clr)
                                base_color:  if chunk_y % 2 == 0 { Color::RED } else { Color::BLUE },
                                ..default()
                            }),
                            ..default()
                        },
                        chunk.clone(),
                        MeshFacingDirection(facing)
                    ));
                }
                
            }
        }
    }

    commands.spawn(
        TextBundle::from_section(
            "Controls:\nMouse: Move camera\nWASD: Move player\nShift: Go down\nSpace: Go up",
            TextStyle {
                font_size: 20.0,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(12.0),
            left: Val::Px(12.0),
            ..default()
        }),
    );
}

struct MeshSide {
    vertices: [[f32; 3]; 4],
    normals: [[f32; 3]; 4],
    indices: [u32; 6],
}

#[derive(Default)]
struct MeshData {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

fn add_side(mesh: &mut MeshData, side: MeshSide) {
    //TODO: totally know what im doing yep
    let length = mesh.vertices.len();
    mesh.vertices.extend_from_slice(&side.vertices);
    mesh.normals.extend_from_slice(&side.normals);
    let next_indices = side.indices.map(|i| i + length as u32);
    mesh.indices.extend_from_slice(&next_indices)
}

fn top_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let up = Vec3::Y.to_array();
    return MeshSide {
        vertices: [
            [x + -size, y + size, z + -size],
            [x + size, y + size, z + -size],
            [x + size, y + size, z + size],
            [x + -size, y + size, z + size],
        ],
        normals: [up, up, up, up],
        indices: [0, 3, 1, 1, 3, 2],
    };
}

fn bottom_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let down = Vec3::NEG_Y.to_array();
    return MeshSide {
        vertices: [
            [x + -size, y + -size, z + -size],
            [x + size, y + -size, z + -size],
            [x + size, y + -size, z + size],
            [x + -size, y + -size, z + size],
        ],
        normals: [down, down, down, down],
        indices: [0, 1, 3, 1, 2, 3],
    };
}

fn right_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let right = Vec3::X.to_array();
    return MeshSide {
        vertices: [
            [x + size, y + -size, z + -size],
            [x + size, y + -size, z + size],
            [x + size, y + size, z + size],
            [x + size, y + size, z + -size],
        ],
        normals: [right, right, right, right],
        indices: [0, 3, 1, 1, 3, 2],
    };
}

fn left_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let left = Vec3::NEG_X.to_array();
    return MeshSide {
        vertices: [
            [x + -size, y + -size, z + -size],
            [x + -size, y + -size, z + size],
            [x + -size, y + size, z + size],
            [x + -size, y + size, z + -size],
        ],
        normals: [left, left, left, left],
        indices: [0, 1, 3, 1, 2, 3],
    };
}

fn back_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let back = Vec3::Z.to_array();
    MeshSide {
        vertices: [
            [x + -size, y + -size, z + size],
            [x + -size, y + size, z + size],
            [x + size, y + size, z + size],
            [x + size, y + -size, z + size],
        ],
        normals: [back, back, back, back],
        indices: [0, 3, 1, 1, 3, 2],
    }
}

fn front_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let front = Vec3::NEG_Z.to_array();
    return MeshSide {
        vertices: [
            [x + -size, y + -size, z + -size],
            [x + -size, y + size, z + -size],
            [x + size, y + size, z + -size],
            [x + size, y + -size, z + -size],
        ],
        normals: [front, front, front, front],
        indices: [0, 1, 3, 1, 2, 3],
    };
}


#[derive(Event)]
struct CameraDirectionChangeEvent;

fn detect_camera_direction_changed(camera_query: Query<(&bevy_flycam::FlyCam, &mut Transform)>, 
                          mut past_cam_dir: ResMut<PastCameraDirection>,
                          mut ev_levelup: EventWriter<CameraDirectionChangeEvent>
) {
    for (_, transform) in &camera_query {
        let was_facing = past_cam_dir.0;
        let facing: Direction3d = transform.forward();
        past_cam_dir.0 = facing;

        //matrices???
        let x = Vec3 { x: 1.0, y: 0.0, z: 0.0};
        let r1 = was_facing.dot(x);
        let r2 = facing.dot(x);
        let x_changed = r1 * r2 < 0.0;

        let y = Vec3 { x: 0.0, y: 1.0, z: 0.0};
        let u1 = was_facing.dot(y);
        let u2 = facing.dot(y);
        let y_changed = u1 * u2 < 0.0;

        let z = Vec3 { x: 0.0, y: 0.0, z: 1.0};
        let f1 = was_facing.dot(z);
        let f2 = facing.dot(z);
        let z_changed = f1 * f2 < 0.0;

        if x_changed || y_changed || z_changed {
            ev_levelup.send(CameraDirectionChangeEvent);
        }
    }
}


fn on_camera_direction_change(
    mut ev: EventReader<CameraDirectionChangeEvent>,
    camera_query: Query<(&bevy_flycam::FlyCam, &mut Transform)>,
    mut chunks_query: Query<(&mut Visibility, &MeshFacingDirection)>,
) {
    for _ in ev.read() {
        for (_, transform) in &camera_query {
            let facing: Direction3d = transform.forward();
            for (mut visibility, mesh_facing) in chunks_query.iter_mut() {
                if mesh_facing.0.dot(facing.xyz()) <= 0.0 {
                    *visibility = Visibility::Visible;
                }
                else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
   
}

fn create_cube_side(
    MeshData {
        vertices,
        normals,
        indices,
    }: MeshData,
) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}

fn create_chunk_sides(x_chunk_offset: f32, y_chunk_offset: f32, z_chunk_offset: f32, size: f32, data: [BlockData;TOTAL_SIZE]) -> Vec<(Mesh, Direction3d)> {
    let size = size / 2.0;

    let mut up: MeshData = MeshData::default();
    let mut down: MeshData = MeshData::default();
    let mut left: MeshData = MeshData::default();
    let mut right: MeshData = MeshData::default();
    let mut front: MeshData = MeshData::default();
    let mut back: MeshData = MeshData::default();

    for i in 1..=LAST_XYZ {
        let (x_block, y_block, z_block) = to_3d(i);
        let x = x_block as f32 + x_chunk_offset as f32 * BLOCK_SIZE;
        let y = y_block as f32 + y_chunk_offset as f32 * BLOCK_SIZE;
        let z = z_block as f32 + z_chunk_offset as f32 * BLOCK_SIZE;

        match &data[i].visible_faces {
            None => (),
            Some(faces) => {
                if faces.top {
                    add_side(&mut up, top_side(x, y, z, size));
                }
                if faces.bottom {
                    add_side(&mut down, bottom_side(x, y, z, size));
                }
                if faces.left {
                    add_side(&mut left, left_side(x, y, z, size));
                }
                if faces.right {
                    add_side(&mut right, right_side(x, y, z, size));
                }
                if faces.front {
                    add_side(&mut front, front_side(x, y, z, size));
                }
                if faces.back {
                    add_side(&mut back, back_side(x, y, z, size));
                }
            }
        }
    }
    vec! [
        (create_cube_side(up), Direction3d::Y),
        (create_cube_side(down), Direction3d::NEG_Y),
        (create_cube_side(left),Direction3d::NEG_X),
        (create_cube_side(right),Direction3d::X),
        (create_cube_side(front),Direction3d::NEG_Z),
        (create_cube_side(back),Direction3d::Z),
    ]

    
}
