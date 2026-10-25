use bevy::prelude::*;

mod player;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugins(player::player::PlayerPlugin)
        .run();
}
