// player/build.rs
//
// build mode of the player

use crate::player::{modes::PlayerMode, ui::SubModeButton};
use bevy::{prelude::*, ui::Pressed, ui_widgets::Button};
pub struct PlayerPluginBuild;

impl Plugin for PlayerPluginBuild {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_mode_buttons_pressed);
    }
}

fn check_mode_buttons_pressed(
    pressed_buttons: Query<&SubModeButton, (With<Button>, With<Pressed>)>,
    player_mode: Res<PlayerMode>,
) {
    for text in pressed_buttons {
        println!("{:?}", player_mode.0)
    }
}
