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

const CHUNKS_PER_AXIS: usize = 1; // chunk constants
const SIZE: usize = 4;
const X_SIZE: usize = SIZE;
const Y_SIZE: usize = SIZE;
const Z_SIZE: usize = SIZE;

const TOTAL_SIZE: usize = X_SIZE * Y_SIZE * Z_SIZE;
const PERLIN_SAMPLE_SIZE: f32 = 0.1;

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
struct Chunk(Index3D);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, hideNonCameraFaces)
        .insert_resource(ClearColor(Color::rgb(0.9, 0.9, 0.9)))
        .add_plugins(PlayerPlugin)
        .run();
}

fn index(arr: [f64; TOTAL_SIZE], x: usize, y: usize, z: usize) -> f64 {
    let index: usize = x + y * X_SIZE + z * Y_SIZE * X_SIZE;
    arr[index]
}

fn index_reverse(n: usize) -> Index3D {
    let x: usize = n % X_SIZE;
    let y: usize = (n / (X_SIZE)) % Y_SIZE;
    let z: usize = n / (X_SIZE * Y_SIZE);
    Index3D { x, y, z }
}

fn perlin(x_offset: f32, y_offset: f32, z_offset: f32) -> [f64; TOTAL_SIZE] {
    let perlin = Perlin::new(1234);
    core::array::from_fn(|n| {
        let Index3D { x, y, z } = index_reverse(n);
        perlin.get([
            ((x as f32) * PERLIN_SAMPLE_SIZE + x_offset) as f64,
            ((y as f32) * PERLIN_SAMPLE_SIZE + y_offset) as f64,
            ((z as f32) * PERLIN_SAMPLE_SIZE + z_offset) as f64,
        ])
    })
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for chunk_x in 0..CHUNKS_PER_AXIS {
        for chunk_y in 0..CHUNKS_PER_AXIS {
            for chunk_z in 0..CHUNKS_PER_AXIS {
                let chunk = Chunk(Index3D {
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

                for n in 0..(TOTAL_SIZE - 1) {
                    let Index3D { x: x_block, y: y_block, z: z_block } = index_reverse(n);
                    let val = perlin_chunk[n];

                    //if the value is big enough we need a mesh
                    if val > -1.0 {
                        let cube_sides = create_cube_sides(
                            x_block as f32 + x_chunk_offset as f32 * BLOCK_SIZE,
                            y_block as f32 + y_chunk_offset as f32 * BLOCK_SIZE,
                            z_block as f32 + z_chunk_offset as f32 * BLOCK_SIZE,
                            BLOCK_SIZE,
                        );

                        for side in cube_sides {
                            // Create and save a handle to the mesh.
                            let side_mesh_handle: Handle<Mesh> = meshes.add(side);
                            commands.spawn((
                                PbrBundle {
                                    mesh: side_mesh_handle,
                                    material: materials.add(StandardMaterial {
                                        //base_color_texture: Some(custom_texture_handle),
                                        base_color: Color::rgb(n as f32, n as f32, n as f32), // if chunk_y % 2 == 0 { Color::RED } else { Color::BLUE },
                                        ..default()
                                    }),
                                    ..default()
                                },
                                chunk.clone(),
                            ));
                        }
                    }
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
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
}

fn top_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let up = Vec3::Y.to_array();
    return MeshSide {
        vertices: vec![
        [x + -size, y + size, z + -size],
        [x + size, y + size, z + -size],
        [x + size, y + size, z + size],
        [x + -size, y + size, z + size],
    ],
        normals: vec![up, up, up, up],
        indices: vec![0, 3, 1, 1, 3, 2],
    };
}

fn bottom_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let down = Vec3::NEG_Y.to_array();
    return MeshSide {
        vertices: vec![
        [x + -size, y + -size, z + -size],
        [x + size, y + -size, z + -size],
        [x + size, y + -size, z + size],
        [x + -size, y + -size, z + size],
    ],
        normals: vec![down, down, down, down],
        indices: vec![0, 1, 3, 1, 2, 3],
    };
}

fn right_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let right = Vec3::X.to_array();
    return MeshSide {
        vertices: vec![
        [x + size, y + -size, z + -size],
        [x + size, y + -size, z + size],
        [x + size, y + size, z + size],
        [x + size, y + size, z + -size],
    ],
        normals: vec![right, right, right, right],
        indices: vec![0, 3, 1, 1, 3, 2],
    };
}

fn left_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let left = Vec3::NEG_X.to_array();
    return MeshSide {
        vertices: vec![
        [x + -size, y + -size, z + -size],
        [x + -size, y + -size, z + size],
        [x + -size, y + size, z + size],
        [x + -size, y + size, z + -size],
    ],
        normals: vec![left, left, left, left],
        indices: vec![0, 1, 3, 1, 2, 3],
    };
}

fn back_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let back = Vec3::Z.to_array();
    MeshSide {
        vertices: vec![
            [x + -size, y + -size, z + size],
            [x + -size, y + size, z + size],
            [x + size, y + size, z + size],
            [x + size, y + -size, z + size],
        ],
        normals: vec![back, back, back, back],
        indices: vec![0, 3, 1, 1, 3, 2]
    }
}

fn front_side(x: f32, y: f32, z: f32, size: f32) -> MeshSide {
    let front = Vec3::NEG_Z.to_array();
    return MeshSide {
        vertices: vec![
        [x + -size, y + -size, z + -size],
        [x + -size, y + size, z + -size],
        [x + size, y + size, z + -size],
        [x + size, y + -size, z + -size],
    ],
        normals: vec![front, front, front, front],
        indices: vec![0, 1, 3, 1, 2, 3],
    };
}

fn hideNonCameraFaces(query: Query<(&bevy_flycam::FlyCam, &mut Transform)>) {
    for (_, transform) in &query {
        let facing: Direction3d = transform.forward();
        println!("{}", facing.dot(Vec3::X));
    }
}

fn create_cube_side(
    MeshSide {
        vertices,
        normals,
        indices,
    }: MeshSide,
) -> Mesh {
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_indices(Indices::U32(indices))
}

fn create_cube_sides(x: f32, y: f32, z: f32, size: f32) -> [Mesh; 6] {
    [
        create_cube_side(top_side(x, y, z, size)),
        create_cube_side(bottom_side(x, y, z, size)),
        create_cube_side(left_side(x, y, z, size)),
        create_cube_side(right_side(x, y, z, size)),
        create_cube_side(front_side(x, y, z, size)),
        create_cube_side(back_side(x, y, z, size)),
    ]
}

#[rustfmt::skip]
fn create_cube_mesh(x: f32, y: f32, z: f32, size: f32) -> Mesh {
    let size = size / 2.0;
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [ x + -size, y + size, z + -size], // vertex with index 0
            [ x + size, y + size, z + -size], // vertex with index 1
            [ x + size, y + size, z + size], // etc. until 23
            [ x + -size, y + size, z + size],
            // bottom   (-y)
            [ x + -size, y + -size, z + -size],
            [ x + size, y + -size, z + -size],
            [ x + size, y + -size, z + size],
            [ x + -size, y + -size, z + size],
            // right    (+x)
            [ x + size, y + -size, z + -size],
            [ x + size, y + -size, z + size],
            [ x + size, y + size, z + size], 
            [ x + size, y + size, z + -size],
            // left     (-x)
            [ x + -size, y + -size, z + -size],
            [ x + -size, y + -size, z + size],
            [ x + -size, y + size, z + size],
            [ x + -size, y + size, z + -size],
            // back     (+z)
            [ x + -size, y + -size, z + size],
            [ x + -size, y + size, z + size],
            [ x + size, y + size, z + size],
            [ x + size, y + -size, z + size],
            // forward  (-z)
            [ x + -size, y + -size, z + -size],
            [ x + -size, y + size, z + -size],
            [ x + size, y + size, z + -size],
            [ x + size, y + -size, z + -size],
        ],
    )
    // For meshes with flat shading, normals are orthogonal (pointing out) from the direction of
    // the surface.
    // Normals are required for correct lighting calculations.
    // Each array represents a normalized vector, which length should be equal to 1.0.
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![
            // Normals for the top side (towards +y)
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            // Normals for the bottom side (towards -y)
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            [0.0, -1.0, 0.0],
            // Normals for the right side (towards +x)
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            // Normals for the left side (towards -x)
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            // Normals for the back side (towards +z)
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            [0.0, 0.0, 1.0],
            // Normals for the forward side (towards -z)
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
            [0.0, 0.0, -1.0],
        ],
    )
    // Create the triangles out of the 24 vertices we created.
    // To construct a square, we need 2 triangles, therefore 12 triangles in total.
    // To construct a triangle, we need the indices of its 3 defined vertices, adding them one
    // by one, in a counter-clockwise order (relative to the position of the viewer, the order
    // should appear counter-clockwise from the front of the triangle, in this case from outside the cube).
    // Read more about how to correctly build a mesh manually in the Bevy documentation of a Mesh,
    // further examples and the implementation of the built-in shapes.
    .with_inserted_indices(Indices::U32(vec![
        0,3,1 , 1,3,2, // triangles making up the top (+y) facing side.
        4,5,7 , 5,6,7, // bottom (-y)
        8,11,9 , 9,11,10, // right (+x)
        12,13,15 , 13,14,15, // left (-x)
        16,19,17 , 17,19,18, // back (+z)
        20,21,23 , 21,22,23, // forward (-z)
    ]))
}
