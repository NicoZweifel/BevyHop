mod core;
mod game_over;
mod hud;
mod loading;
mod main_menu;
mod node_builder;
mod pause;
mod text_resource;

pub use core::*;

use bevy::{prelude::*, window::CursorGrabMode};
use bevy_dev_tools::fps_overlay::*;
use bevy_egui::EguiPlugin;
use bevy_fps_controller::controller::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game_over::GameOverPlugin;
use hud::HudPlugin;
use loading::LoadingScreenPlugin;
use main_menu::MainMenuPlugin;
use node_builder::*;
use pause::PausePlugin;

use crate::prelude::*;

pub struct UiPlugin;

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
            .add_plugins((
                MainMenuPlugin,
                GameOverPlugin,
                LoadingScreenPlugin,
                PausePlugin,
                HudPlugin,
            ))
            .add_systems(Startup, setup_font)
            .add_systems(Update, button_system);
    }
}
