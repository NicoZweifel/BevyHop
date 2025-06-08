use bevy::prelude::*;

use super::*;

impl TextResource {
    pub fn get(&self) -> Handle<Font> {
        self.0.clone()
    }

    pub fn get_text_font(&self, font_size: f32) -> TextFont {
        TextFont {
            font: self.get(),
            font_size,
            ..default()
        }
    }

    pub fn get_text_props(&self, font_size: f32, color: Color) -> (TextFont, TextColor) {
        (self.get_text_font(font_size), TextColor(color))
    }

    pub fn get_hud_text_props(&self, font_size: f32) -> (TextFont, TextColor) {
        (self.get_text_font(font_size), TextColor(HUD_TEXT_COLOR))
    }

    pub fn get_button_text_props(&self) -> (TextFont, TextColor) {
        (self.get_text_font(40.), TextColor(BUTTON_TEXT_COLOR))
    }
}
