use bevy::prelude::*;

use super::{BORDER, BORDER_RADIUS, MARGIN, NORMAL_BUTTON, PADDING};

pub struct NodeBuilder {
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
            padding: value.padding,
            margin: value.margin,
            row_gap: MARGIN,
            column_gap: MARGIN,
            border: value.border,
            ..default()
        }
    }
}

impl NodeBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self) -> Node {
        self.into()
    }

    pub fn with_direction(&mut self, direction: FlexDirection) -> &mut Self {
        self.direction = direction;
        self
    }

    pub fn with_align_items(&mut self, align_items: AlignItems) -> &mut Self {
        self.align_items = align_items;
        self
    }

    pub fn with_justify_content(&mut self, justify_content: JustifyContent) -> &mut Self {
        self.justify_content = justify_content;
        self
    }

    pub fn with_grow(&mut self, grow: bool) -> &mut Self {
        self.grow = grow;
        self
    }

    pub fn with_padding(&mut self, padding: UiRect) -> &mut Self {
        self.padding = padding;
        self
    }

    pub fn with_margin(&mut self, margin: UiRect) -> &mut Self {
        self.margin = margin;
        self
    }

    pub fn with_border(&mut self, border: UiRect) -> &mut Self {
        self.border = border;
        self
    }

    pub fn get_button(&mut self) -> impl Bundle {
        let padding = self.padding;
        let margin = self.margin;
        (
            Button,
            self.with_padding(if padding == UiRect::ZERO {
                UiRect::all(PADDING)
            } else {
                padding
            })
            .with_margin(if margin == UiRect::ZERO {
                UiRect::all(MARGIN)
            } else {
                margin
            })
            .get(),
            BorderRadius::all(BORDER_RADIUS),
        )
    }

    pub fn get_card(&mut self) -> impl Bundle + use<> {
        let padding = self.padding;
        let margin = self.margin;
        let border = self.border;
        (
            self.with_padding(if padding == UiRect::ZERO {
                UiRect::all(PADDING)
            } else {
                padding
            })
            .with_margin(if margin == UiRect::ZERO {
                UiRect::all(MARGIN)
            } else {
                margin
            })
            .with_border(if border == UiRect::ZERO {
                UiRect::all(BORDER)
            } else {
                border
            })
            .get(),
            NodeBuilder::get_card_props(),
        )
    }

    pub fn get_card_props() -> impl Bundle {
        (
            BorderRadius::all(BORDER_RADIUS),
            BackgroundColor(NORMAL_BUTTON.with_alpha(0.5)),
            BorderColor(NORMAL_BUTTON.with_alpha(0.8)),
        )
    }
}
