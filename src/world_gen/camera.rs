use bevy::{
    ecs::{
        event::{Event, EventReader, EventWriter},
        system::{Query, ResMut, Resource},
    },
    math::{primitives::Direction3d, Vec3, Vec3Swizzles},
    render::view::Visibility,
    transform::components::Transform,
};

use super::MeshFacingDirection;

#[derive(Event)]
pub struct CameraDirectionChangeEvent;

#[derive(Resource)]
pub struct PastCameraDirection(pub Direction3d);

pub fn detect_camera_direction_changed(
    camera_query: Query<(&bevy_flycam::FlyCam, &mut Transform)>,
    mut past_cam_dir: ResMut<PastCameraDirection>,
    mut ev_levelup: EventWriter<CameraDirectionChangeEvent>,
) {
    for (_, transform) in &camera_query {
        let was_facing = past_cam_dir.0;
        let facing: Direction3d = transform.forward();
        past_cam_dir.0 = facing;

        //matrices???
        let x = Vec3 {
            x: 1.0,
            y: 0.0,
            z: 0.0,
        };
        let r1 = was_facing.dot(x);
        let r2 = facing.dot(x);
        let x_changed = r1 * r2 < 0.0;

        let y = Vec3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        };
        let u1 = was_facing.dot(y);
        let u2 = facing.dot(y);
        let y_changed = u1 * u2 < 0.0;

        let z = Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        };
        let f1 = was_facing.dot(z);
        let f2 = facing.dot(z);
        let z_changed = f1 * f2 < 0.0;

        if x_changed || y_changed || z_changed {
            ev_levelup.send(CameraDirectionChangeEvent);
        }
    }
}

pub fn on_camera_direction_change(
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
                } else {
                    *visibility = Visibility::Hidden;
                }
            }
        }
    }
}
