use bevy::prelude::*;

use super::*;

#[derive(Resource)]
pub struct TextResource {
    pub default_font: Handle<Font>,
    pub header_font: Handle<Font>,
}

impl TextResource {
    pub fn get_text_font(&self, font_size: f32) -> TextFont {
        TextFont {
            font: self.default_font.clone(),
            font_size,
            ..default()
        }
    }

    pub fn get_header_text_font(&self) -> TextFont {
        TextFont {
            font: self.header_font.clone(),
            font_size: 144.,
            ..default()
        }
    }

    pub fn get_text_props(&self, font_size: f32, color: Color) -> impl Bundle {
        (self.get_text_font(font_size), TextColor(color))
    }

    pub fn get_header_text_props(&self) -> impl Bundle {
        (self.get_header_text_font(), TextColor(Resurrect64::PURPLE))
    }

    pub fn get_hud_text_props(&self, font_size: f32) -> impl Bundle {
        (self.get_text_font(font_size), TextColor(HUD_TEXT_COLOR))
    }

    pub fn get_button_text_props(&self) -> impl Bundle {
        (self.get_text_font(28.), TextColor(BUTTON_TEXT_COLOR))
    }
}
