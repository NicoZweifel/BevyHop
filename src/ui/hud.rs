use avian3d::prelude::*;
use bevy::prelude::*;

use crate::core::*;

use super::*;

#[derive(Component)]
struct Hud;

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), setup_hud)
            .add_systems(OnExit(AppState::InGame), cleanup::<Hud>)
            .add_systems(
                Update,
                (
                    update_speed_ui,
                    update_level_duration_ui,
                    update_run_duration_ui,
                )
                    .in_set(GameplaySet),
            );
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
