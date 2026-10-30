// player/camera.rs
//
// camera mode of the player

use bevy::prelude::*;

pub struct PlayerPluginCamera;

impl Plugin for PlayerPluginCamera {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_background));
        app.add_systems(Update, update_camera);
    }
}

#[derive(Component)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        MainCamera,
        Transform::default(),
        Projection::from(OrthographicProjection {
            scale: 0.25,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

// Move Camera

const CAMERA_SPEED: f32 = 150.0;

fn update_camera(
    mut player: Single<&mut Transform, With<Camera2d>>,
    kb_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut direction = Vec2::ZERO;

    if kb_input.pressed(KeyCode::KeyW) {
        direction[1] += 1.;
    };

    if kb_input.pressed(KeyCode::KeyS) {
        direction[1] -= 1.;
    };

    if kb_input.pressed(KeyCode::KeyD) {
        direction[0] += 1.;
    };

    if kb_input.pressed(KeyCode::KeyA) {
        direction[0] -= 1.;
    };

    let mut delta_direction = direction.normalize_or_zero() * time.delta_secs() * CAMERA_SPEED;

    if kb_input.pressed(KeyCode::ShiftLeft) {
        delta_direction *= 2.;
    }

    player.translation += delta_direction.extend(0.);
}

// Load Background (Not sure if this will stay here)

fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Transform::from_xyz(0.0, 0.0, -10.0),
        Sprite::from_image(asset_server.load("background.png")),
    ));
}
