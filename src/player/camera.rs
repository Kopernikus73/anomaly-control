// player/camera.rs
//
// camera mode of the player

use bevy::prelude::*;

pub struct PlayerPluginCamera;

impl Plugin for PlayerPluginCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::default(),
        Projection::from(OrthographicProjection {
            scale: 0.125,
            ..OrthographicProjection::default_2d()
        }),
    ));
}
