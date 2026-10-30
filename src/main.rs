use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

mod hologram;
mod player;

#[derive(Clone, Eq, PartialEq, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    LoadingAssets,
    Main,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::LoadingAssets).continue_to_state(GameState::Main),
        )
        .add_plugins(player::player::PlayerPlugin)
        .add_plugins(hologram::HologramPlugin)
        .run();
}
