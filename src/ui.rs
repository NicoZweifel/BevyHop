use avian3d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_fps_controller::controller::*;

use crate::{core::*, state::*};

pub struct UiPlugin;

#[derive(Component)]
struct ButtonColors {
    normal: Color,
    hovered: Color,
}

impl Default for ButtonColors {
    fn default() -> Self {
        ButtonColors {
            normal: Color::linear_rgb(0.15, 0.15, 0.15),
            hovered: Color::linear_rgb(0.25, 0.25, 0.25),
        }
    }
}

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_hud)
            .add_systems(OnExit(AppState::InGame), cleanup::<Hud>)
            .add_systems(Update, update_speed_ui.run_if(in_state(AppState::InGame)))
            .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
            .add_systems(
                OnExit(AppState::MainMenu),
                (cleanup::<MainMenu>, cleanup::<Camera3d>),
            )
            .add_systems(OnEnter(PausedState::Paused), setup_pause_menu)
            .add_systems(OnExit(PausedState::Paused), cleanup::<PauseMenu>);
    }
}

#[derive(Component)]
struct Hud;

fn setup_hud(mut cmd: Commands, assets: Res<AssetServer>) {
    cmd.spawn((
        Text(String::from("")),
        Hud,
        TextFont {
            font: assets.load("fira_mono.ttf"),
            font_size: 24.0,
            ..default()
        },
        TextColor(Color::BLACK),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(5.0),
            left: Val::Px(5.0),
            ..default()
        },
    ));
}

const MARGIN: Val = Val::Px(12.);

#[derive(Component)]
struct PauseMenu;

fn setup_pause_menu(mut cmd: Commands) {
    cmd.spawn((
        Node {
            // fill the entire window
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            ..Default::default()
        },
        PauseMenu,
    ))
    .with_children(|children| {
        let button_colors = ButtonColors::default();
        children
            .spawn((
                Button,
                Node {
                    width: Val::Px(140.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(button_colors.normal),
                button_colors,
            ))
            .observe(
                |_: Trigger<Pointer<Click>>,
                 mut ns: ResMut<NextState<PausedState>>,

                 mut window_query: Query<&mut Window>,
                 mut controller_query: Query<&mut FpsController>| {
                    ns.set(PausedState::Running);

                    for mut window in &mut window_query {
                        window.cursor_options.grab_mode = CursorGrabMode::Locked;
                        window.cursor_options.visible = false;
                        for mut controller in &mut controller_query {
                            controller.enable_input = true;
                        }
                    }
                },
            )
            .with_child((
                Text::new("Resume"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            ));
    });
}

#[derive(Component)]
struct MainMenu;

fn setup_main_menu(mut cmd: Commands) {
    cmd.spawn(Camera3d::default());
    cmd.spawn((
        Node {
            // fill the entire window
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            ..Default::default()
        },
        MainMenu,
    ))
    .with_children(|children| {
        let button_colors = ButtonColors::default();
        children
            .spawn((
                Button,
                Node {
                    width: Val::Px(140.0),
                    height: Val::Px(50.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                BackgroundColor(button_colors.normal),
                button_colors,
            ))
            .observe(
                |_: Trigger<Pointer<Click>>,
                 mut ns: ResMut<NextState<AppState>>,

                 mut window_query: Query<&mut Window>,
                 mut controller_query: Query<&mut FpsController>| {
                    ns.set(AppState::InGame);

                    for mut window in &mut window_query {
                        window.cursor_options.grab_mode = CursorGrabMode::Locked;
                        window.cursor_options.visible = false;
                        for mut controller in &mut controller_query {
                            controller.enable_input = true;
                        }
                    }
                },
            )
            .with_child((
                Text::new("Play"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            ));
    });
}

fn update_speed_ui(
    mut controller_query: Query<&LinearVelocity, With<LogicalPlayer>>,
    mut text_query: Query<&mut Text, With<Hud>>,
) {
    for velocity in &mut controller_query {
        for mut text in &mut text_query {
            text.0 = format!("{:.2}", velocity.0.xz().length());
        }
    }
}
