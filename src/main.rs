use bevy::prelude::*;
use bevy_flycam::PlayerPlugin;
use world_gen::WorldGenPlugin;
mod world_gen;


fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(WorldGenPlugin)
        
        //diagnostics
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins(bevy::diagnostic::EntityCountDiagnosticsPlugin)

        .add_systems(Startup, setup)
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




