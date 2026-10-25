// player/modes.rs
//
// general mode logic and operations

use bevy::prelude::*;

pub struct PlayerPluginModes;

impl Plugin for PlayerPluginModes {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerMode(Mode::CameraMode));
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    CameraMode,
    BuildMode(BuildMode),
    OverviewMode(OverviewMode),
}

#[derive(Debug, Clone, Copy)]
pub enum BuildMode {
    RoomMode,
    UtilityMode,
}

#[derive(Debug, Clone, Copy)]
pub enum OverviewMode {
    PeopleMode,
    TemperatureMode,
}

#[derive(Resource)]
pub struct PlayerMode(pub Mode);
