use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::camera;
use bevy_flycam::PlayerPlugin;
use iyes_perf_ui::PerfUiCompleteBundle;
use noise::{NoiseFn, Perlin};
use world_gen::MeshFacingDirection;
mod world_gen;



#[derive(Resource)]
struct PastCameraDirection(Direction3d);

fn main() {
    App::new()      
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        
        //diagnostics              
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)

        .insert_resource(PastCameraDirection(Direction3d::X))
        .add_event::<CameraDirectionChangeEvent>()
        .add_systems(Startup, setup)
        .add_systems(Startup, world_gen::mesh_setup)
        .add_systems(Update, detect_camera_direction_changed)
        .add_systems(Update, on_camera_direction_change)
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
      
        .run();
}


fn setup(
    mut commands: Commands,    
) {
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


