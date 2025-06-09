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
        layout(),
        children![header(&text_resource), content(&text_resource),],
    ));
}

fn layout() -> impl Bundle {
    (
        NodeBuilder::new()
            .with_grow(true)
            .with_align_items(AlignItems::Start)
            .with_justify_content(JustifyContent::SpaceBetween)
            .get(),
        Pickable::IGNORE,
        Hud,
    )
}

fn header(text_resource: &Res<TextResource>) -> impl Bundle {
    (
        NodeBuilder::new()
            .with_direction(FlexDirection::Row)
            .with_grow(true)
            .with_align_items(AlignItems::Start)
            .with_justify_content(JustifyContent::SpaceAround)
            .get(),
        Pickable::IGNORE,
        children![
            (
                NodeBuilder::new().get_card(),
                Pickable::IGNORE,
                children![(
                    Text(String::from("")),
                    LevelDurationText,
                    text_resource.get_hud_text_props(24.0),
                )]
            ),
            (
                NodeBuilder::new().get_card(),
                Pickable::IGNORE,
                children![(
                    Text(String::from("")),
                    RunDurationText,
                    text_resource.get_hud_text_props(24.),
                )]
            ),
        ],
    )
}

fn content(text_resource: &Res<TextResource>) -> impl Bundle {
    (
        NodeBuilder::new()
            .with_grow(true)
            .with_direction(FlexDirection::Row)
            .with_align_items(AlignItems::Center)
            .with_margin(UiRect::all(MARGIN))
            .with_padding(UiRect::all(PADDING * 2.))
            .with_justify_content(JustifyContent::Center)
            .get(),
        Pickable::IGNORE,
        children![(
            NodeBuilder::new().get_card(),
            Pickable::IGNORE,
            children![(
                Text(String::from("")),
                Speed,
                text_resource.get_hud_text_props(28.0)
            )]
        ),],
    )
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
