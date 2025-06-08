use bevy::{ecs::spawn::SpawnRelatedBundle, prelude::*};

use crate::color::Resurrect64;

use super::*;

pub(super) const PADDING: Val = Val::Px(12.);
pub(super) const MARGIN: Val = Val::Px(12.);
pub(super) const BORDER: Val = Val::Px(1.);
pub(super) const BORDER_RADIUS: Val = Val::Px(5.);

pub(super) const NORMAL_BUTTON: Color = Resurrect64::DARK_PURPLE_1;
pub(super) const HOVERED_BUTTON: Color = Resurrect64::DARK_PURPLE_2;
pub(super) const PRESSED_BUTTON: Color = Resurrect64::GRAY_PURPLE_1;

pub(super) const BACKGROUND: Color = Resurrect64::DARK_SLATE_BLUE;

pub(super) const HUD_TEXT_COLOR: Color = BUTTON_TEXT_COLOR;

pub(super) const BUTTON_TEXT_COLOR: Color = Color::linear_rgb(0.9, 0.9, 0.9);

#[derive(Component)]
pub(super) struct Speed;

#[derive(Component)]
pub(super) struct AutoJumpUi;

#[derive(Component)]
pub(super) struct LevelDurationText;

#[derive(Component)]
pub(super) struct RunDurationText;

#[derive(Resource)]
pub struct TextResource(pub Handle<Font>);

pub(super) fn get_header(
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

pub(super) fn setup_font(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let handle = asset_server.load("fira_mono.ttf");
    cmd.insert_resource(TextResource(handle.clone()));
    loading.0.push(handle.into());
}

pub(super) fn button_system(
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
