use avian3d::prelude::*;
use bevy::{prelude::*, window::CursorGrabMode};
use bevy_egui::EguiPlugin;
use bevy_fps_controller::controller::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

pub struct UiPlugin;

const NORMAL_BUTTON: Color = Resurrect64::DARK_PURPLE_1;
const HOVERED_BUTTON: Color = Resurrect64::DARK_PURPLE_2;
const PRESSED_BUTTON: Color = Resurrect64::GRAY_PURPLE_1;

const BACKGROUND: Color = Resurrect64::DEEP_PURPLE;

const HUD_TEXT_COLOR: Color = Resurrect64::DARK_PURPLE_1;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin {
                enable_multipass_for_primary_context: false,
            },
            WorldInspectorPlugin::default().run_if(in_state(DebugState::Enabled)),
            bevy_console::ConsolePlugin,
        ))
        .add_systems(Startup, setup_font)
        .add_systems(OnEnter(AppState::InGame), setup_hud)
        .add_systems(OnExit(AppState::InGame), cleanup::<Hud>)
        .add_systems(Update, button_system)
        .add_systems(
            Update,
            (
                update_speed_ui,
                update_level_duration_ui,
                update_run_duration_ui,
            )
                .in_set(GameplaySet),
        )
        .add_systems(OnEnter(AppState::MainMenu), setup_main_menu)
        .add_systems(
            OnExit(AppState::MainMenu),
            (cleanup::<MainMenu>, cleanup::<Camera3d>),
        )
        .add_systems(OnEnter(PausedState::Paused), setup_pause_menu)
        .add_systems(OnExit(PausedState::Paused), cleanup::<PauseMenu>)
        .add_systems(OnEnter(AppState::GameOver), setup_game_over_menu)
        .add_systems(
            OnExit(AppState::GameOver),
            (cleanup::<GameOverMenu>, cleanup::<Camera3d>),
        );
    }
}

#[derive(Resource)]
struct FontResource(Handle<Font>);

impl FontResource {
    fn get(&self) -> Handle<Font> {
        self.0.clone()
    }

    fn get_text_font(&self, font_size: f32) -> TextFont {
        TextFont {
            font: self.get(),
            font_size,
            ..default()
        }
    }

    fn get_text_props(&self, font_size: f32) -> (TextFont, TextColor) {
        (self.get_text_font(font_size), TextColor(HUD_TEXT_COLOR))
    }
}

fn setup_font(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.insert_resource(FontResource(asset_server.load("fira_mono.ttf")));
}

#[derive(Component)]
struct Hud;

#[derive(Component)]
struct Speed;

#[derive(Component)]
struct AutoJumpUi;

#[derive(Component)]
struct LevelDurationText;

#[derive(Component)]
struct RunDurationText;

fn setup_hud(mut cmd: Commands, font: Res<FontResource>) {
    cmd.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Start,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            ..Default::default()
        },
        Hud,
        children![
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(MARGIN),
                    row_gap: MARGIN,
                    ..Default::default()
                },
                children![
                    (
                        Text(String::from("")),
                        LevelDurationText,
                        font.get_text_props(24.0),
                    ),
                    (
                        Text(String::from("")),
                        RunDurationText,
                        font.get_text_props(24.),
                    ),
                ]
            ),
            (
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    flex_direction: FlexDirection::Row,
                    align_items: AlignItems::End,
                    justify_content: JustifyContent::SpaceBetween,
                    padding: UiRect::all(MARGIN),
                    row_gap: MARGIN,
                    ..Default::default()
                },
                children![
                    (Text(String::from("")), Speed, font.get_text_props(24.0),),
                    (
                        AutoJumpUi,
                        Node {
                            flex_direction: FlexDirection::Column,
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            padding: UiRect::all(MARGIN),
                            ..Default::default()
                        },
                        BorderRadius::all(Val::Px(10.)),
                        children![
                            (Text::new("Auto-Jump"), font.get_text_props(14.0),),
                            (Text::new("SHIFT+SPACE"), font.get_text_props(10.0),),
                        ],
                    ),
                ]
            ),
        ],
    ));
}

const MARGIN: Val = Val::Px(12.);

#[derive(Component)]
struct PauseMenu;

fn setup_pause_menu(mut cmd: Commands, debug_state: Res<State<DebugState>>) {
    cmd.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            column_gap: MARGIN,
            ..Default::default()
        },
        PauseMenu,
        BackgroundColor(BACKGROUND.with_alpha(match debug_state.get() {
            DebugState::Disabled => 0.5,
            DebugState::Enabled => 0.,
        })),
    ))
    .with_children(|cmd| {
        cmd.spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),
                margin: UiRect::bottom(MARGIN),
                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Resume"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
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
        );

        cmd.spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),

                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Main Menu"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
        ))
        .observe(
            |_: Trigger<Pointer<Click>>,
             mut ns_app_state: ResMut<NextState<AppState>>,
             mut ns_paused: ResMut<NextState<PausedState>>| {
                ns_app_state.set(AppState::MainMenu);
                ns_paused.set(PausedState::Running);
            },
        );

        cmd.spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),

                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Quit"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
        ))
        .observe(|_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
            ew.write(AppExit::Success);
        });
    });
}

#[derive(Component)]
struct GameOverMenu;

fn setup_game_over_menu(mut cmd: Commands) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));
    cmd.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            column_gap: MARGIN,
            ..Default::default()
        },
        GameOverMenu,
        BackgroundColor(BACKGROUND),
    ))
    .with_children(|cmd| {
        cmd.spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),

                margin: UiRect::bottom(MARGIN),
                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Restart"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
        ))
        .observe(
            |_: Trigger<Pointer<Click>>,
             mut ns_paused_state: ResMut<NextState<PausedState>>,
             mut ns_app_state: ResMut<NextState<AppState>>,
             mut window_query: Query<&mut Window>,
             mut controller_query: Query<&mut FpsController>| {
                ns_paused_state.set(PausedState::Running);
                ns_app_state.set(AppState::InGame);

                for mut window in &mut window_query {
                    window.cursor_options.grab_mode = CursorGrabMode::Locked;
                    window.cursor_options.visible = false;
                    for mut controller in &mut controller_query {
                        controller.enable_input = true;
                    }
                }
            },
        );
        cmd.spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),

                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Main Menu"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
        ))
        .observe(
            |_: Trigger<Pointer<Click>>, mut ns: ResMut<NextState<AppState>>| {
                ns.set(AppState::MainMenu);
            },
        );

        cmd.spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),

                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Quit"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
        ))
        .observe(|_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
            ew.write(AppExit::Success);
        });
    });
}

#[derive(Component)]
struct MainMenu;

fn setup_main_menu(mut cmd: Commands) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));
    cmd.spawn((
        BackgroundColor(BACKGROUND),
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_items: JustifyItems::Center,
            padding: UiRect::all(MARGIN),
            row_gap: MARGIN,
            column_gap: MARGIN,
            ..Default::default()
        },
        MainMenu,
    ))
    .with_children(|cmd| {
        cmd.spawn((
            Button,
            Node {
                padding: UiRect::all(MARGIN),
                margin: UiRect::bottom(MARGIN),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Play"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
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
        );

        cmd.spawn((
            Button,
            Node {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(MARGIN),

                ..Default::default()
            },
            BorderRadius::all(Val::Px(10.)),
            children![(
                Text::new("Quit"),
                TextFont {
                    font_size: 40.0,
                    ..default()
                },
                TextColor(Color::linear_rgb(0.9, 0.9, 0.9)),
            )],
        ))
        .observe(|_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
            ew.write(AppExit::Success);
        });
    });
}

fn update_speed_ui(
    mut controller_query: Query<&LinearVelocity, With<LogicalPlayer>>,
    mut text_query: Query<&mut Text, With<Speed>>,
) {
    for velocity in &mut controller_query {
        for mut text in &mut text_query {
            text.0 = format!("Speed: {:.2}", velocity.0.xz().length());
        }
    }
}

fn update_level_duration_ui(
    duration: ResMut<LevelDuration>,
    mut text_query: Query<&mut Text, With<LevelDurationText>>,
    time: Res<Time>,
) {
    let stopwatch = duration.into_inner();
    stopwatch.0.tick(time.delta());
    let secs = stopwatch.0.elapsed_secs();

    let new_text = format!("Level: {}", format_duration(secs));

    for mut text in &mut text_query {
        text.0 = new_text.clone();
    }
}

fn update_run_duration_ui(
    run_duration: Res<RunDuration>,
    level_duration: Res<LevelDuration>,
    mut text_query: Query<&mut Text, With<RunDurationText>>,
) {
    let stopwatch = level_duration.into_inner();
    let secs = run_duration
        .results
        .iter()
        .map(|x| x.as_secs_f32())
        .sum::<f32>()
        + stopwatch.0.elapsed_secs();

    let new_text = format!("Run: {}", format_duration(secs));

    for mut text in &mut text_query {
        text.0 = new_text.clone();
    }
}

fn format_duration(secs: f32) -> String {
    let h = secs / 3600.;
    let m = (secs % 3600.) / 60.;
    let s = secs % 60.;
    format!("{:02.0}:{:02.0}:{:02.0}", h, m, s)
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
