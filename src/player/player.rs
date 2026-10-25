// player/player.rs
//
// bundle of all player logic

use crate::player::{build, camera, modes, ui, view};
use bevy::prelude::*;

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
    }
}
