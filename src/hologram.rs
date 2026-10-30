// hologram.rs
//
// rendering holograms

use bevy::prelude::*;

use crate::player::player::MouseWorldPosition;

pub struct HologramPlugin;

impl Plugin for HologramPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_holograms, delete_unused_holograms));
    }
}

#[derive(Component)]
pub struct Hologram;

#[derive(Component)]
pub struct PositionAtMouse;

fn update_holograms(
    holograms: Query<&mut Transform, (With<Hologram>, With<PositionAtMouse>, With<Sprite>)>,
    mouse_position: Res<MouseWorldPosition>,
) {
    for mut hologram in holograms.into_iter() {
        hologram.translation = mouse_position.0.extend(0.);
        dbg!(&mouse_position.0);
    }
}

fn delete_unused_holograms(
    mut commands: Commands,
    holograms: Query<Entity, With<Hologram>>,
    kb_input: Res<ButtonInput<KeyCode>>,
) {
    if kb_input.pressed(KeyCode::Escape) {
        for hologram in holograms.iter() {
            commands.entity(hologram).despawn();
        }
    }
}
