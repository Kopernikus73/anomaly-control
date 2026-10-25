use bevy::{
    input::mouse::MouseWheel, picking::hover::Hovered, prelude::*, ui::Pressed, ui_widgets::Button,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_camera, setup_text, setup_buttons));
        app.add_systems(
            Update,
            (
                update_mode,
                check_mode_buttons_pressed,
                update_container_scroll,
            ),
        );
        app.insert_resource(PlayerMode(Mode::CameraMode));
    }
}
