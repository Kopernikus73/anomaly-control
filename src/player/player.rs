// player/player.rs
//
// bundle of all player logic

use crate::player::{
    build,
    camera::{self, MainCamera},
    modes, ui, view,
};
use bevy::{prelude::*, window::PrimaryWindow};
pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            build::PlayerPluginBuild,
            camera::PlayerPluginCamera,
            modes::PlayerPluginModes,
            ui::PlayerPluginUi,
            view::PlayerPluginView,
        ));
        app.init_resource::<MouseWorldPosition>();
        app.init_resource::<PlayerInteractionLevel>();
        app.add_systems(Update, print_cursor_world_coords);
    }
}

// Only for later use so not everything gets closed at once
#[derive(Resource, Default)]
pub struct PlayerInteractionLevel(pub u16);

#[derive(Resource, Default)]
pub struct MouseWorldPosition(pub Vec2);

fn print_cursor_world_coords(
    mut mouse_position: ResMut<MouseWorldPosition>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    let (camera, camera_transform) = *camera;

    if let Some(cursor_screen_pos) = window.cursor_position() {
        if let Ok(world_pos) = camera.viewport_to_world_2d(camera_transform, cursor_screen_pos) {
            mouse_position.0 = world_pos;
        }
    }
}
