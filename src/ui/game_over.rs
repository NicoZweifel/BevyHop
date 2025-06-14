use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::core::*;

use super::*;

#[derive(Component)]
struct GameOver;

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOverMenu;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::GameOver), setup)
            .add_systems(
                OnExit(AppState::GameOver),
                (cleanup::<GameOverMenu>, cleanup::<Camera3d>),
            );
    }
}

fn setup(mut cmd: Commands, text_resource: Res<TextResource>, run_duration: Res<RunDuration>) {
    layout(&mut cmd).with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                header(cmd, &text_resource);
                content(cmd, &text_resource, &run_duration);
                actions(cmd, &text_resource);
            });
    });
}

fn layout<'a>(cmd: &'a mut Commands) -> EntityCommands<'a> {
    cmd.spawn((
        Camera3d::default(),
        Transform::from_translation(Vec3::ZERO.with_y(15.)),
    ));

    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        GameOverMenu,
        BackgroundColor(BACKGROUND),
    ))
}

fn header(cmd: &mut RelatedSpawnerCommands<'_, ChildOf>, text_resource: &Res<TextResource>) {
    cmd.spawn(get_header(text_resource));
}

fn actions(cmd: &mut RelatedSpawnerCommands<'_, ChildOf>, text_resource: &Res<TextResource>) {
    cmd.spawn((NodeBuilder::new().with_direction(FlexDirection::Row).get(),))
        .with_children(|cmd| {
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
        });

    #[cfg(not(target_arch = "wasm32"))]
    cmd.spawn((
        NodeBuilder::new().get_button(),
        children![(Text::new("Quit"), text_resource.get_button_text_props())],
    ))
    .observe(|_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
        ew.write(AppExit::Success);
    });
}

fn content(
    cmd: &mut RelatedSpawnerCommands<'_, ChildOf>,
    text_resource: &Res<TextResource>,
    run_duration: &Res<RunDuration>,
) {
    let secs = run_duration
        .results
        .iter()
        .map(|x| x.as_secs_f32())
        .sum::<f32>();

    cmd.spawn((
        NodeBuilder::new().get_card(),
        children![(
            NodeBuilder::new().with_margin(UiRect::all(MARGIN)).get(),
            children![(
                Text(format!("Run: {}", format_duration(secs))),
                text_resource.get_text_props(32.0, Resurrect64::BRIGHT_GREEN),
            )],
        )],
    ))
    .with_children(|cmd| {
        run_duration.results.iter().enumerate().for_each(|(i, x)| {
            cmd.spawn((
                NodeBuilder::new().get(),
                children![(
                    Text(format!(
                        "Level {}: {}",
                        i + 1,
                        format_duration(x.as_secs_f32())
                    )),
                    text_resource.get_text_props(
                        24.0,
                        match i {
                            1 => Resurrect64::GREEN,
                            2 => Resurrect64::SCARLET,
                            _ => Resurrect64::LIGHT_PURPLE,
                        }
                    ),
                )],
            ));
        });
    });
}

fn handle_restart(_: Trigger<Pointer<Click>>, mut ns_app_state: ResMut<NextState<AppState>>) {
    ns_app_state.set(AppState::InGame);
}
