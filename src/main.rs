use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;
use bevy::render::camera;
use bevy_flycam::PlayerPlugin;
use iyes_perf_ui::PerfUiCompleteBundle;
use noise::{NoiseFn, Perlin};
use world_gen::camera::{detect_camera_direction_changed, on_camera_direction_change, CameraDirectionChangeEvent, PastCameraDirection};
mod world_gen;


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




