// player/modes.rs
//
// general mode logic and operations

use crate::player::ui::ScrollContainerIdentifier;
use bevy::prelude::*;
pub struct PlayerPluginModes;

impl Plugin for PlayerPluginModes {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayerMode(Mode::CameraMode));
        app.add_systems(Update, update_modes);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    CameraMode,
    BuildMode(BuildMode),
    OverviewMode(OverviewMode),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BuildMode {
    RoomMode,
    UtilityMode,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OverviewMode {
    PeopleMode,
    TemperatureMode,
}

#[derive(Resource)]
pub struct PlayerMode(pub Mode);

fn update_modes(
    mut commands: Commands,
    mode_menus: Query<Entity, With<ScrollContainerIdentifier>>,
    mut player_mode: ResMut<PlayerMode>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    if kb_input.pressed(KeyCode::Escape) {
        player_mode.0 = Mode::CameraMode;
        for menu in mode_menus.iter() {
            commands.entity(menu).despawn();
        }
    }
}
