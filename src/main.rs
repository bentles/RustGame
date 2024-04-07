//! This example demonstrates how to create a custom mesh,
//! assign a custom UV mapping for a custom texture,
//! and how to change the UV mapping at run-time.

use bevy::prelude::*;
use bevy::render::{
    mesh::{Indices, VertexAttributeValues},
    render_asset::RenderAssetUsages,
    render_resource::PrimitiveTopology,
};
use bevy_flycam::PlayerPlugin;
use noise::{NoiseFn, Perlin};

// chunk constants
const SIZE: usize = 10;
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

#[derive(Component, Clone)]
struct Chunk(Index3D);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
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
    for xi in 0..2 {
        for yi in 0..2 {
            for zi in 0..2 {
                let chunk = Chunk(Index3D {
                    x: xi,
                    y: yi,
                    z: zi,
                });

                let x_offset = (chunk.0.x * X_SIZE) as f32;
                let y_offset = (chunk.0.y * Y_SIZE) as f32;
                let z_offset = (chunk.0.z * Z_SIZE) as f32;
                // Import the custom texture
                let perlin_chunk = perlin(
                    x_offset * PERLIN_SAMPLE_SIZE,
                    y_offset * PERLIN_SAMPLE_SIZE,
                    z_offset * PERLIN_SAMPLE_SIZE,
                );

                for n in 1..TOTAL_SIZE {
                    let Index3D { x, y, z } = index_reverse(n);
                    let val = perlin_chunk[n];
                    //let custom_texture_handle: Handle<Image> = asset_server.load("textures/array_texture.png");
                    // Render the mesh with the custom texture using a PbrBundle, add the marker.

                    //if the value is big enough we need a mesh
                    if val > 0.0 {
                        // Create and save a handle to the mesh.
                        let cube_mesh_handle: Handle<Mesh> = meshes.add(create_cube_mesh(
                            x as f32 + x_offset as f32 * BLOCK_SIZE,
                            y as f32 + y_offset as f32 * BLOCK_SIZE,
                            z as f32 + z_offset as f32 * BLOCK_SIZE,
                        ));
                        commands.spawn((
                            PbrBundle {
                                mesh: cube_mesh_handle,
                                material: materials.add(StandardMaterial {
                                    //base_color_texture: Some(custom_texture_handle),
                                    base_color: if xi % 2 == 0 { Color::RED } else { Color::BLUE },
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

    // Transform for the camera and lighting, looking at (0,0,0) (the position of the mesh).
    let camera_and_light_transform =
        Transform::from_xyz(18.0, 18.0, 18.0).looking_at(Vec3::ZERO, Vec3::Y);

    // Camera in 3D space.
    //commands.spawn(Camera3dBundle {
    //    transform: camera_and_light_transform,
    //    ..default()
    //}).with(FlyCamera::default());

    // Light up the scene

    commands.spawn(
        TextBundle::from_section(
            "Controls:\nSpace: Change UVs\nX/Y/Z: Rotate\nR: Reset orientation",
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

#[rustfmt::skip]
fn create_cube_mesh(x: f32, y: f32, z: f32) -> Mesh {
    // Keep the mesh data accessible in future frames to be able to mutate it in toggle_texture.
    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD)
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_POSITION,
        // Each array is an [x, y, z] coordinate in local space.
        // Meshes always rotate around their local [0, 0, 0] when a rotation is applied to their Transform.
        // By centering our mesh around the origin, rotating the mesh preserves its center of mass.
        vec![
            // top (facing towards +y)
            [ x + -0.5, y + 0.5, z + -0.5], // vertex with index 0
            [ x + 0.5, y + 0.5, z + -0.5], // vertex with index 1
            [ x + 0.5, y + 0.5, z + 0.5], // etc. until 23
            [ x + -0.5, y + 0.5, z + 0.5],
            // bottom   (-y)
            [ x + -0.5, y + -0.5, z + -0.5],
            [ x + 0.5, y + -0.5, z + -0.5],
            [ x + 0.5, y + -0.5, z + 0.5],
            [ x + -0.5, y + -0.5, z + 0.5],
            // right    (+x)
            [ x + 0.5, y + -0.5, z + -0.5],
            [ x + 0.5, y + -0.5, z + 0.5],
            [ x + 0.5, y + 0.5, z + 0.5], // This vertex is at the same position as vertex with index 2, but they'll have different UV and normal
            [ x + 0.5, y + 0.5, z + -0.5],
            // left     (-x)
            [ x + -0.5, y + -0.5, z + -0.5],
            [ x + -0.5, y + -0.5, z + 0.5],
            [ x + -0.5, y + 0.5, z + 0.5],
            [ x + -0.5, y + 0.5, z + -0.5],
            // back     (+z)
            [ x + -0.5, y + -0.5, z + 0.5],
            [ x + -0.5, y + 0.5, z + 0.5],
            [ x + 0.5, y + 0.5, z + 0.5],
            [ x + 0.5, y + -0.5, z + 0.5],
            // forward  (-z)
            [ x + -0.5, y + -0.5, z + -0.5],
            [ x + -0.5, y + 0.5, z + -0.5],
            [ x + 0.5, y + 0.5, z + -0.5],
            [ x + 0.5, y + -0.5, z + -0.5],
        ],
    )
    // Set-up UV coordinates to point to the upper (V < 0.5), "dirt+grass" part of the texture.
    // Take a look at the custom image (assets/textures/array_texture.png)
    // so the UV coords will make more sense
    // Note: (0.0, 0.0) = Top-Left in UV mapping, (1.0, 1.0) = Bottom-Right in UV mapping
    .with_inserted_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            // Assigning the UV coords for the top side.
            [0.0, 0.2], [0.0, 0.0], [1.0, 0.0], [1.0, 0.25],
            // Assigning the UV coords for the bottom side.
            [0.0, 0.45], [0.0, 0.25], [1.0, 0.25], [1.0, 0.45],
            // Assigning the UV coords for the right side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the left side.
            [1.0, 0.45], [0.0, 0.45], [0.0, 0.2], [1.0, 0.2],
            // Assigning the UV coords for the back side.
            [0.0, 0.45], [0.0, 0.3], [1.0, 0.3], [1.0, 0.45],
            // Assigning the UV coords for the forward side.
            [0.0, 0.45], [0.0, 0.2], [1.0, 0.2], [1.0, 0.45],
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

// Function that changes the UV mapping of the mesh, to apply the other texture.
fn toggle_texture(mesh_to_change: &mut Mesh) {
    // Get a mutable reference to the values of the UV attribute, so we can iterate over it.
    let uv_attribute = mesh_to_change.attribute_mut(Mesh::ATTRIBUTE_UV_0).unwrap();
    // The format of the UV coordinates should be Float32x2.
    let VertexAttributeValues::Float32x2(uv_attribute) = uv_attribute else {
        panic!("Unexpected vertex format, expected Float32x2.");
    };

    // Iterate over the UV coordinates, and change them as we want.
    for uv_coord in uv_attribute.iter_mut() {
        // If the UV coordinate points to the upper, "dirt+grass" part of the texture...
        if uv_coord[1] <= 0.5 {
            // ... point to the equivalent lower, "sand+water" part instead,
            uv_coord[1] += 0.5;
        } else {
            // else, point back to the upper, "dirt+grass" part.
            uv_coord[1] -= 0.5;
        }
    }
}
