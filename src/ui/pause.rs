use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::core::*;

use super::*;

pub struct PausePlugin;

impl Plugin for PausePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PausedState::Paused), setup_pause_menu)
            .add_systems(OnExit(PausedState::Paused), cleanup::<PauseMenu>);
    }
}

#[derive(Component)]
struct PauseMenu;

fn setup_pause_menu(
    mut cmd: Commands,
    debug_state: Res<State<DebugState>>,
    text_resource: Res<TextResource>,
) {
    pause_menu_layout(&mut cmd, &debug_state).with_children(|cmd| {
        cmd.spawn(NodeBuilder::new().get_card())
            .with_children(|cmd| {
                pause_menu_header(cmd, &text_resource);
                pause_menu_content(cmd, &text_resource);
            });
    });
}

fn pause_menu_layout<'a>(
    cmd: &'a mut Commands,

    debug_state: &Res<State<DebugState>>,
) -> EntityCommands<'a> {
    cmd.spawn((
        NodeBuilder::new().with_grow(true).get(),
        PauseMenu,
        BackgroundColor(BACKGROUND.with_alpha(match debug_state.get() {
            DebugState::Disabled => 0.5,
            DebugState::Enabled => 0.,
        })),
    ))
}

fn pause_menu_header(
    cmd: &mut RelatedSpawnerCommands<'_, ChildOf>,
    text_resource: &Res<TextResource>,
) {
    cmd.spawn(get_header(text_resource));
}

fn pause_menu_content(
    cmd: &mut RelatedSpawnerCommands<'_, ChildOf>,
    text_resource: &Res<TextResource>,
) {
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
    .observe(|_: Trigger<Pointer<Click>>, mut ew: EventWriter<AppExit>| {
        ew.write(AppExit::Success);
    });
}

fn handle_resume(_: Trigger<Pointer<Click>>, mut ns: ResMut<NextState<PausedState>>) {
    ns.set(PausedState::Running);
}
