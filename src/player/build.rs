// player/build.rs
//
// build mode of the player

use crate::{
    GameState,
    hologram::{Hologram, PositionAtMouse},
    player::{
        modes::{BuildMode, Mode, PlayerMode},
        ui::SubModeButton,
    },
};
use bevy::{prelude::*, ui::Pressed, ui_widgets::Button};
use bevy_asset_loader::prelude::*;
pub struct PlayerPluginBuild;

impl Plugin for PlayerPluginBuild {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (check_block_buttons_pressed /*move_asset_left*/,).run_if(in_state(GameState::Main)),
        );
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::LoadingAssets)
                .load_collection::<BuildingBlockAssets>(),
        );
    }
}

#[derive(Component)]
struct BuildingBlockIdentifier;

fn check_block_buttons_pressed(
    pressed_buttons: Query<&SubModeButton, (With<Button>, Changed<Pressed>)>,
    building_holograms: Query<Entity, (With<BuildingBlockIdentifier>, With<Hologram>)>,
    player_mode: Res<PlayerMode>,
    building_blocks_assets: Res<BuildingBlockAssets>,
    mut commands: Commands,
) {
    for button in pressed_buttons {
        println!("{:?}: {}", player_mode.0, &button.index);
        if player_mode.0 == Mode::BuildMode(BuildMode::RoomMode) && (button.index < 4) {
            let mut sprite = Sprite::from_atlas_image(
                building_blocks_assets.sprite.clone(),
                TextureAtlas::from(building_blocks_assets.layout.clone()),
            );

            if let Some(atlas) = &mut sprite.texture_atlas {
                atlas.index = button.index as usize;
            }

            for hologram in building_holograms.iter() {
                commands.entity(hologram).despawn();
            }

            if button.index % 2 == 0 {
                commands.spawn((
                    sprite,
                    Transform::from_xyz(100., 100., 0.),
                    BuildingBlockIdentifier,
                ));
            } else {
                commands.spawn((
                    sprite,
                    Transform::from_xyz(100., 100., 0.),
                    BuildingBlockIdentifier,
                    Hologram,
                    PositionAtMouse,
                ));
            }
        }
    }
}

// fn move_asset_left(assets: Query<&mut Transform, With<BuildingBlockIdentifier>>, time: Res<Time>) {
//     for mut asset in assets.into_iter() {
//         asset.translation +=
//             ((Vec2::ZERO.normalize_or_zero() + Vec2::new(-10., 0.)) * time.delta_secs() * 10.)
//                 .extend(0.);
//     }
// }

// Assets

// Weird implementation...
//
// Brick Wall: 1
// Brick Wall Hologram: 2

#[derive(Resource, AssetCollection)]
struct BuildingBlockAssets {
    #[asset(texture_atlas_layout(
        tile_size_x = 8,
        tile_size_y = 8,
        columns = 2,
        rows = 2,
        padding_x = 2,
        padding_y = 1
    ))]
    layout: Handle<TextureAtlasLayout>,
    #[asset(path = "build-blocks.png")]
    sprite: Handle<Image>,
}
