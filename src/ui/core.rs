use bevy::{ecs::spawn::SpawnRelatedBundle, prelude::*};

use crate::color::Resurrect64;

use super::*;

pub use super::text_resource::*;

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

pub(super) fn get_header(
    text_resource: &Res<TextResource>,
) -> (impl Bundle, SpawnRelatedBundle<ChildOf, Spawn<impl Bundle>>) {
    (
        NodeBuilder::new()
            .with_padding(UiRect::all(PADDING * 2.))
            .with_margin(UiRect::all(MARGIN * 2.))
            .get_card(),
        children![(
            Text(String::from("BevyHop")),
            text_resource.get_header_text_props(),
        )],
    )
}

pub(super) fn setup_font(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    mut loading: ResMut<AssetsLoading>,
) {
    let default_font = asset_server.load("fira_mono.ttf");
    let header_font = asset_server.load("cherry_bomb_one/CherryBombOne-Regular.ttf");

    loading.0.push(default_font.clone().into());
    loading.0.push(header_font.clone().into());

    cmd.insert_resource(TextResource {
        default_font,
        header_font,
    });
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

pub(super) fn format_duration(secs: f32) -> String {
    let h = secs / 3600.;
    let m = (secs % 3600.) / 60.;
    let s = secs % 60.;
    format!("{:02.0}:{:02.0}:{:02.0}", h, m, s)
}
