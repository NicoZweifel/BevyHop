use avian3d::prelude::*;
use bevy::{ecs::spawn::SpawnRelatedBundle, prelude::*, window::CursorGrabMode};
use bevy_dev_tools::fps_overlay::*;
use bevy_egui::EguiPlugin;
use bevy_fps_controller::controller::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use crate::prelude::*;

pub struct UiPlugin;

const PADDING: Val = Val::Px(12.);
const MARGIN: Val = Val::Px(12.);
const BORDER: Val = Val::Px(1.);

const NORMAL_BUTTON: Color = Resurrect64::DARK_PURPLE_1;
const HOVERED_BUTTON: Color = Resurrect64::DARK_PURPLE_2;
const PRESSED_BUTTON: Color = Resurrect64::GRAY_PURPLE_1;

const BACKGROUND: Color = Resurrect64::DARK_SLATE_BLUE;

const HUD_TEXT_COLOR: Color = Resurrect64::DARK_PURPLE_1;

const BUTTON_TEXT_COLOR: Color = Color::linear_rgb(0.9, 0.9, 0.9);

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(BACKGROUND))
            .add_plugins((
                FpsOverlayPlugin {
                    config: FpsOverlayConfig {
                        text_config: TextFont {
                            font_size: 16.0,
                            ..default()
                        },
                        text_color: HUD_TEXT_COLOR,
                        enabled: false,
                        ..default()
                    },
                },
                EguiPlugin {
                    enable_multipass_for_primary_context: false,
                },
                WorldInspectorPlugin::default().run_if(in_state(DebugState::Enabled)),
                bevy_console::ConsolePlugin,
            ))
            .add_systems(
                Startup,
                (setup_font, setup_loading_screen.after(setup_font)),
            )
            .add_systems(
                OnExit(AppState::Loading),
                (cleanup::<LoadingScreen>, cleanup::<Camera3d>),
            )
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
pub struct TextResource(Handle<Font>);

impl TextResource {
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

    fn get_text_props(&self, font_size: f32, color: Color) -> (TextFont, TextColor) {
        (self.get_text_font(font_size), TextColor(color))
    }

    fn get_hud_text_props(&self, font_size: f32) -> (TextFont, TextColor) {
        (self.get_text_font(font_size), TextColor(HUD_TEXT_COLOR))
    }

    fn get_button_text_props(&self) -> (TextFont, TextColor) {
        (self.get_text_font(40.), TextColor(BUTTON_TEXT_COLOR))
    }
}

fn setup_font(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let handle = asset_server.load("fira_mono.ttf");
    cmd.insert_resource(TextResource(handle.clone()));
    loading.0.push(handle.into());
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

struct NodeBuilder {
    direction: FlexDirection,
    align_items: AlignItems,
    justify_content: JustifyContent,
    grow: bool,
    padding: UiRect,
    margin: UiRect,
    border: UiRect,
}

impl Default for NodeBuilder {
    fn default() -> Self {
        Self {
            direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            grow: false,
            padding: UiRect::ZERO,
            margin: UiRect::ZERO,
            border: UiRect::ZERO,
        }
    }
}

impl From<&NodeBuilder> for Node {
    fn from(value: &NodeBuilder) -> Self {
        Self {
            width: match value.grow {
                true => Val::Percent(100.),
                false => default(),
            },
            height: match value.grow {
                true => Val::Percent(100.),
                false => default(),
            },
            flex_direction: value.direction,
            align_items: value.align_items,
            justify_content: value.justify_content,
            padding: value.margin,
            margin: value.padding,
            row_gap: MARGIN / 2.,
            column_gap: MARGIN / 2.,
            border: value.border,
            ..default()
        }
    }
}

type CardProps = (BorderRadius, BackgroundColor, BorderColor);

impl NodeBuilder {
    fn new() -> Self {
        Self::default()
    }

    fn get(&self) -> Node {
        self.into()
    }

    fn with_direction(&mut self, direction: FlexDirection) -> &mut Self {
        self.direction = direction;
        self
    }

    fn with_align_items(&mut self, align_items: AlignItems) -> &mut Self {
        self.align_items = align_items;
        self
    }

    fn with_justify_content(&mut self, justify_content: JustifyContent) -> &mut Self {
        self.justify_content = justify_content;
        self
    }

    fn with_grow(&mut self, grow: bool) -> &mut Self {
        self.grow = grow;
        self
    }

    fn with_padding(&mut self, padding: UiRect) -> &mut Self {
        self.padding = padding;
        self
    }

    fn with_margin(&mut self, margin: UiRect) -> &mut Self {
        self.margin = margin;
        self
    }

    fn with_border(&mut self, border: UiRect) -> &mut Self {
        self.border = border;
        self
    }

    fn get_button(&mut self) -> (Button, Node, BorderRadius) {
        (
            Button,
            self.with_padding(UiRect::all(PADDING))
                .with_margin(UiRect::all(MARGIN))
                .get(),
            BorderRadius::all(Val::Px(10.)),
        )
    }

    fn get_card(&mut self) -> (Node, CardProps) {
        (
            self.with_padding(UiRect::all(PADDING))
                .with_margin(UiRect::all(MARGIN))
                .with_border(UiRect::all(BORDER))
                .get(),
            NodeBuilder::get_card_props(),
        )
    }

    fn get_card_props() -> CardProps {
        (
            BorderRadius::all(Val::Px(10.)),
            BackgroundColor(NORMAL_BUTTON.with_alpha(0.1)),
            BorderColor(NORMAL_BUTTON.with_alpha(0.5)),
        )
    }
}

fn setup_hud(mut cmd: Commands, text_resource: Res<TextResource>) {
    cmd.spawn((
        NodeBuilder::new()
            .with_grow(true)
            .with_align_items(AlignItems::Start)
            .with_justify_content(JustifyContent::SpaceBetween)
            .with_margin(UiRect::all(MARGIN))
            .get(),
        Hud,
        children![
            (
                NodeBuilder::new()
                    .with_grow(true)
                    .with_direction(FlexDirection::Row)
                    .with_align_items(AlignItems::Start)
                    .with_justify_content(JustifyContent::SpaceBetween)
                    .get(),
                children![
                    (
                        NodeBuilder::new().get_card(),
                        children![(
                            Text(String::from("")),
                            LevelDurationText,
                            text_resource.get_hud_text_props(24.0),
                        )]
                    ),
                    (
                        NodeBuilder::new().get_card(),
                        children![(
                            Text(String::from("")),
                            RunDurationText,
                            text_resource.get_hud_text_props(24.),
                        )]
                    ),
                ]
            ),
            (
                NodeBuilder::new()
                    .with_grow(true)
                    .with_direction(FlexDirection::Row)
                    .with_align_items(AlignItems::End)
                    .with_justify_content(JustifyContent::SpaceBetween)
                    .get(),
                children![
                    (
                        NodeBuilder::new().get_card(),
                        children![(
                            Text(String::from("")),
                            Speed,
                            text_resource.get_hud_text_props(24.0)
                        )]
                    ),
                    (
                        AutoJumpUi,
                        NodeBuilder::new()
                            .with_justify_content(JustifyContent::End)
                            .get_card(),
                        children![
                            (
                                Text::new("Auto-Jump"),
                                text_resource.get_text_props(20.0, HUD_TEXT_COLOR),
                            ),
                            (
                                Text::new("SHIFT+SPACE"),
                                text_resource.get_hud_text_props(16.0),
                            ),
                        ],
                    ),
                ]
            ),
        ],
    ));
}

#[derive(Component)]
struct PauseMenu;

fn setup_pause_menu(
    mut cmd: Commands,
    debug_state: Res<State<DebugState>>,
    text_resource: Res<TextResource>,
) {
    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        PauseMenu,
        BackgroundColor(BACKGROUND.with_alpha(match debug_state.get() {
            DebugState::Disabled => 0.5,
            DebugState::Enabled => 0.,
        })),
    ))
    .with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                cmd.spawn(get_header(&text_resource));

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Resume"), text_resource.get_button_text_props(),)],
                ))
                .observe(handle_resume);

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(
                        Text::new("Main Menu"),
                        text_resource.get_button_text_props()
                    )],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ns_app_state: ResMut<NextState<AppState>>| {
                        ns_app_state.set(AppState::MainMenu);
                    },
                );

                #[cfg(not(target_arch = "wasm32"))]
                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Quit"), text_resource.get_button_text_props())],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
                        ew.write(AppExit::Success);
                    },
                );
            });
    });
}

fn handle_resume(
    _: Trigger<Pointer<Click>>,
    mut ns: ResMut<NextState<PausedState>>,

    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    ns.set(PausedState::Running);

    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
}

#[derive(Component)]
struct LoadingScreen;

fn setup_loading_screen(mut cmd: Commands, text_resource: Res<TextResource>) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        LoadingScreen,
        BackgroundColor(BACKGROUND),
        children![(
            NodeBuilder::new().get_card(),
            children![(
                Text::new("Loading..."),
                text_resource.get_button_text_props()
            )],
        )],
    ));
}

#[derive(Component)]
struct GameOverMenu;

fn setup_game_over_menu(mut cmd: Commands, text_resource: Res<TextResource>) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        GameOverMenu,
        BackgroundColor(BACKGROUND),
    ))
    .with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                cmd.spawn(get_header(&text_resource));

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Restart"), text_resource.get_button_text_props())],
                ))
                .observe(handle_restart);

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(
                        Text::new("Main Menu"),
                        text_resource.get_button_text_props()
                    )],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ns: ResMut<NextState<AppState>>| {
                        ns.set(AppState::MainMenu);
                    },
                );

                #[cfg(not(target_arch = "wasm32"))]
                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Quit"), text_resource.get_button_text_props())],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
                        ew.write(AppExit::Success);
                    },
                );
            });
    });
}

fn handle_restart(
    _: Trigger<Pointer<Click>>,
    mut ns_paused_state: ResMut<NextState<PausedState>>,
    mut ns_app_state: ResMut<NextState<AppState>>,
    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    ns_paused_state.set(PausedState::Running);
    ns_app_state.set(AppState::InGame);

    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
}

#[derive(Component)]
struct MainMenu;

fn get_header(
    text_resource: &Res<TextResource>,
) -> (
    (Node, CardProps),
    SpawnRelatedBundle<ChildOf, Spawn<(Text, (TextFont, TextColor))>>,
) {
    (
        NodeBuilder::new()
            .with_margin(UiRect::bottom(MARGIN * 4.))
            .get_card(),
        children![(
            Text(String::from("BevyHop")),
            text_resource.get_text_props(60.0, Resurrect64::LIGHT_PURPLE),
        )],
    )
}

fn setup_main_menu(mut cmd: Commands, text_resource: Res<TextResource>) {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        BackgroundColor(BACKGROUND),
        NodeBuilder::new().with_grow(true).get(),
        MainMenu,
    ))
    .with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                cmd.spawn(get_header(&text_resource));

                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Play"), text_resource.get_button_text_props())],
                ))
                .observe(handle_play);

                #[cfg(not(target_arch = "wasm32"))]
                cmd.spawn((
                    NodeBuilder::new().get_button(),
                    children![(Text::new("Quit"), text_resource.get_button_text_props(),)],
                ))
                .observe(
                    |_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
                        ew.write(AppExit::Success);
                    },
                );
            });
    });
}

fn handle_play(
    _: Trigger<Pointer<Click>>,
    mut ns: ResMut<NextState<AppState>>,

    mut window_query: Query<&mut Window>,
    mut controller_query: Query<&mut FpsController>,
) {
    ns.set(AppState::InGame);

    for mut window in &mut window_query {
        window.cursor_options.grab_mode = CursorGrabMode::Locked;
        window.cursor_options.visible = false;
        for mut controller in &mut controller_query {
            controller.enable_input = true;
        }
    }
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
    current_lvl: Res<CurrentLevel>,
) {
    let stopwatch = duration.into_inner();
    stopwatch.0.tick(time.delta());
    let secs = stopwatch.0.elapsed_secs();

    let new_text = format!("Level {}: {}", current_lvl.get(), format_duration(secs));

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
