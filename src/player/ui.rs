// player/ui.rs
//
// ui components regarding the player

use crate::player::modes::{BuildMode, Mode, OverviewMode, PlayerMode};
use bevy::{
    input::mouse::MouseWheel, picking::hover::Hovered, prelude::*, ui::Pressed, ui_widgets::Button,
};

pub struct PlayerPluginUi;

impl Plugin for PlayerPluginUi {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup_buttons, setup_text));
        app.add_systems(
            Update,
            (
                update_mode,
                check_mode_buttons_pressed,
                update_container_scroll,
            ),
        );
    }
}

// Colors
const NORMAL_BUTTON: Color = Color::srgb(0.2, 0.2, 0.2);
const HOVERED_BUTTON: Color = Color::srgb(0.4, 0.4, 0.4);
const SELECTED_BUTTON: Color = Color::srgb(0.8, 0.8, 0.8);

#[derive(Component)]
struct ModeTextIdentifier;

fn setup_text(mut commands: Commands) {
    commands.spawn((
        Text::new("Active Mode: "),
        Node {
            position_type: PositionType::Absolute,
            top: px(20),
            left: px(20),
            ..default()
        },
        ModeTextIdentifier,
    ));
}

fn update_mode(mut text: Query<&mut Text, With<ModeTextIdentifier>>, player_mode: Res<PlayerMode>) {
    for mut text in &mut text {
        text.0 = format!("Active Mode: {:?}", player_mode.0)
    }
}

// Buttons

#[derive(Component)]
struct ModeButton(Mode);

#[derive(PartialEq)]
enum AnchorX {
    Left,
    Right,
}

#[derive(PartialEq)]
enum AnchorY {
    Top,
    Bottom,
}

fn menu_button(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    anchor_x: AnchorX,
    anchor_y: AnchorY,
    mode: Mode,
    text: &str,
) -> impl Bundle {
    let mut node = Node {
        position_type: PositionType::Absolute,
        width: px(w),
        height: px(h),
        border: UiRect::all(px(5)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    if anchor_x == AnchorX::Left {
        node.left = px(x);
    } else {
        node.right = px(x);
    }

    if anchor_y == AnchorY::Top {
        node.top = px(y);
    } else {
        node.bottom = px(y);
    }

    (
        node,
        ModeButton(mode),
        Button,
        BorderColor::all(Color::BLACK),
        BackgroundColor(NORMAL_BUTTON),
        Hovered::default(),
        children![(Text::new(text), TextFont { ..default() })],
    )
}

fn setup_buttons(mut commands: Commands) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
        children![
            menu_button(
                20.,
                20.,
                40.,
                40.,
                AnchorX::Left,
                AnchorY::Bottom,
                Mode::BuildMode(BuildMode::RoomMode),
                "B"
            ),
            menu_button(
                80.,
                20.,
                40.,
                40.,
                AnchorX::Left,
                AnchorY::Bottom,
                Mode::BuildMode(BuildMode::UtilityMode),
                "U"
            ),
            menu_button(
                20.,
                20.,
                40.,
                40.,
                AnchorX::Right,
                AnchorY::Bottom,
                Mode::OverviewMode(OverviewMode::PeopleMode),
                "P"
            )
        ],
    ));
}

fn check_mode_buttons_pressed(
    mode_menus: Query<Entity, With<ScrollContainerIdentifier>>,
    pressed_buttons: Query<&ModeButton, (With<Button>, Changed<Pressed>)>,
    mut player_mode: ResMut<PlayerMode>,
    mut commands: Commands,
) {
    for button in pressed_buttons {
        player_mode.0 = button.0;

        // delete old scroll menu
        for menu in mode_menus.iter() {
            commands.entity(menu).despawn();
        }

        match player_mode.0 {
            Mode::BuildMode(_build_mode) => {
                build_scrollable_container(&mut commands, 32);
            }
            Mode::OverviewMode(_overview_mode) => {
                build_scrollable_container(&mut commands, 50);
            }
            Mode::CameraMode => {}
        }
    }
}

// May be changed later so the flexibility of menus is greater

#[derive(Component)]
pub struct ScrollContainerIdentifier;

#[derive(Component)]
pub struct SubModeButton {
    pub index: u8,
}

#[derive(Component)]
struct MaxScroll(f32);

const ITEMS_IN_ROW: usize = 4;
const ITEM_SIZE: f32 = 20.;
const ITEM_GAP: f32 = 20.;
const STEP: f32 = ITEM_SIZE + ITEM_GAP;

const CONTAINER_WIDTH: f32 = 180.;
const CONTAINER_HEIGHT: f32 = 300.;

fn build_scrollable_container(commands: &mut Commands, children_num: usize) -> impl Bundle {
    let rows = (children_num as f32 / ITEMS_IN_ROW as f32).ceil();
    let mut max_scroll = rows * ITEM_SIZE + (rows + 1.) * ITEM_GAP - CONTAINER_HEIGHT;
    if !(max_scroll >= 0.) {
        max_scroll = 0.;
    }
    #[derive(Component)]
    struct ModeTextIdentifier;
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                width: px(CONTAINER_WIDTH),
                height: px(CONTAINER_HEIGHT),
                left: px(20),
                bottom: px(80),
                border: UiRect::all(px(2.5)),
                overflow: Overflow::scroll_y(),
                ..default()
            },
            ScrollPosition::default(),
            ScrollContainerIdentifier,
            Hovered::default(),
            BorderColor::all(Color::BLACK),
            MaxScroll(max_scroll),
        ))
        .with_children(|parent| {
            for i in 0..(children_num) {
                parent.spawn((
                    Node {
                        position_type: PositionType::Absolute,
                        width: px(ITEM_SIZE),
                        height: px(ITEM_SIZE),
                        top: px(ITEM_GAP + (i as f32 / ITEMS_IN_ROW as f32).floor() * STEP),
                        left: px(ITEM_GAP + (i % ITEMS_IN_ROW) as f32 * STEP),
                        border: UiRect::all(px(2.5)),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    SubModeButton { index: i as u8 },
                    Button,
                    BorderColor::all(Color::BLACK),
                    children![(Text::new(format!("{i}")))],
                ));
            }
        })
        // Add a Pseudo Item underneath the real items to reach maximum scroll height
        .with_children(|parent| {
            parent.spawn((
                Node {
                    position_type: PositionType::Absolute,
                    width: px(ITEM_SIZE),
                    height: px(ITEM_SIZE),
                    top: px(max_scroll + 300.),
                    left: px(20),
                    ..default()
                },
                Text::new("X"),
            ));
        });
}

const SCROLL_SPEED: f32 = ITEM_SIZE + ITEM_GAP;

fn update_container_scroll(
    mut mouse_wheel_events: MessageReader<MouseWheel>,
    mut scroll_containers: Query<
        (&mut ScrollPosition, &MaxScroll, &Hovered),
        With<ScrollContainerIdentifier>,
    >,
) {
    for event in mouse_wheel_events.read() {
        for (mut scroll_position, max_scroll, hovered) in &mut scroll_containers {
            if hovered.get() {
                let offset = scroll_position.y - event.y * SCROLL_SPEED;
                let new_offset = offset.clamp(0.0, max_scroll.0);
                scroll_position.y = new_offset;
            }
        }
    }
}
