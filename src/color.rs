use bevy::color::Color;

// Resurrect 64 color palette, generated from https://lospec.com/palette-list/resurrect-64.
pub struct Resurrect64;

#[allow(dead_code)]
impl Resurrect64 {
    pub const DARK_PURPLE_1: Color = Color::srgb(0.180, 0.133, 0.184); // #2e222f
    pub const DARK_PURPLE_2: Color = Color::srgb(0.243, 0.208, 0.275); // #3e3546
    pub const GRAY_PURPLE_1: Color = Color::srgb(0.384, 0.333, 0.396); // #625565
    pub const BROWN: Color = Color::srgb(0.588, 0.424, 0.424); // #966c6c
    pub const BEIGE: Color = Color::srgb(0.671, 0.580, 0.478); // #ab947a
    pub const DUSTY_PURPLE: Color = Color::srgb(0.412, 0.310, 0.384); // #694f62

    pub const GRAY_PURPLE_2: Color = Color::srgb(0.498, 0.439, 0.541); // #7f708a
    pub const LIGHT_GRAY_PURPLE: Color = Color::srgb(0.608, 0.671, 0.698); // #9babb2
    pub const MINT_GRAY: Color = Color::srgb(0.780, 0.863, 0.816); // #c7dcd0
    pub const WHITE: Color = Color::srgb(1.0, 1.0, 1.0); // #ffffff
    pub const DARK_RED_1: Color = Color::srgb(0.431, 0.153, 0.153); // #6e2727
    pub const CRIMSON: Color = Color::srgb(0.702, 0.220, 0.192); // #b33831

    pub const VERMILION: Color = Color::srgb(0.918, 0.310, 0.212); // #ea4f36
    pub const ORANGE_RED: Color = Color::srgb(0.961, 0.490, 0.290); // #f57d4a
    pub const DARK_SCARLET: Color = Color::srgb(0.682, 0.137, 0.204); // #ae2334
    pub const SCARLET: Color = Color::srgb(0.910, 0.231, 0.231); // #e83b3b
    pub const ORANGE: Color = Color::srgb(0.984, 0.420, 0.114); // #fb6b1d
    pub const BRIGHT_ORANGE: Color = Color::srgb(0.969, 0.588, 0.090); // #f79617

    pub const GOLD: Color = Color::srgb(0.976, 0.761, 0.169); // #f9c22b
    pub const DARK_BURGUNDY: Color = Color::srgb(0.478, 0.188, 0.271); // #7a3045
    pub const RUST: Color = Color::srgb(0.620, 0.271, 0.224); // #9e4539
    pub const DARK_ORANGE: Color = Color::srgb(0.804, 0.408, 0.239); // #cd683d
    pub const LIGHT_ORANGE: Color = Color::srgb(0.902, 0.565, 0.306); // #e6904e
    pub const PEACH: Color = Color::srgb(0.984, 0.725, 0.329); // #fbb954

    pub const DARK_OLIVE: Color = Color::srgb(0.298, 0.243, 0.141); // #4c3e24
    pub const OLIVE: Color = Color::srgb(0.404, 0.400, 0.200); // #676633
    pub const LIGHT_OLIVE: Color = Color::srgb(0.635, 0.663, 0.278); // #a2a947
    pub const LIME_GREEN: Color = Color::srgb(0.835, 0.878, 0.294); // #d5e04b
    pub const BRIGHT_YELLOW: Color = Color::srgb(0.984, 1.0, 0.525); // #fbff86
    pub const DARK_TEAL: Color = Color::srgb(0.086, 0.353, 0.298); // #165a4c

    pub const FOREST_GREEN: Color = Color::srgb(0.137, 0.565, 0.388); // #239063
    pub const BRIGHT_GREEN: Color = Color::srgb(0.118, 0.737, 0.451); // #1ebc73
    pub const GREEN: Color = Color::srgb(0.569, 0.859, 0.412); // #91db69
    pub const PASTEL_GREEN: Color = Color::srgb(0.804, 0.875, 0.424); // #cddf6c
    pub const DARK_GRAY: Color = Color::srgb(0.192, 0.212, 0.220); // #313638
    pub const SLATE_GRAY: Color = Color::srgb(0.216, 0.306, 0.290); // #374e4a

    pub const GRAY_GREEN: Color = Color::srgb(0.329, 0.494, 0.392); // #547e64
    pub const SAGE: Color = Color::srgb(0.573, 0.663, 0.518); // #92a984
    pub const LIGHT_SAGE: Color = Color::srgb(0.698, 0.729, 0.565); // #b2ba90
    pub const DARK_CYAN: Color = Color::srgb(0.043, 0.369, 0.396); // #0b5e65
    pub const CYAN: Color = Color::srgb(0.043, 0.541, 0.561); // #0b8a8f
    pub const BRIGHT_CYAN: Color = Color::srgb(0.055, 0.686, 0.608); // #0eaf9b

    pub const TURQUOISE: Color = Color::srgb(0.188, 0.882, 0.725); // #30e1b9
    pub const LIGHT_TURQUOISE: Color = Color::srgb(0.561, 0.973, 0.886); // #8ff8e2
    pub const DARK_SLATE_BLUE: Color = Color::srgb(0.196, 0.200, 0.325); // #323353
    pub const INDIGO: Color = Color::srgb(0.282, 0.290, 0.467); // #484a77
    pub const BLUE_1: Color = Color::srgb(0.302, 0.396, 0.706); // #4d65b4
    pub const BLUE_2: Color = Color::srgb(0.302, 0.608, 0.902); // #4d9be6

    pub const LIGHT_BLUE: Color = Color::srgb(0.561, 0.827, 1.0); // #8fd3ff
    pub const DEEP_PURPLE: Color = Color::srgb(0.271, 0.161, 0.247); // #45293f
    pub const PURPLE: Color = Color::srgb(0.420, 0.243, 0.459); // #6b3e75
    pub const LIGHT_PURPLE: Color = Color::srgb(0.565, 0.369, 0.663); // #905ea9
    pub const LAVENDER: Color = Color::srgb(0.659, 0.518, 0.953); // #a884f3
    pub const PALE_LAVENDER: Color = Color::srgb(0.918, 0.929, 0.929); // #eaaded

    pub const ROSE: Color = Color::srgb(0.459, 0.235, 0.329); // #753c54
    pub const DEEP_MAGENTA: Color = Color::srgb(0.635, 0.294, 0.435); // #a24b6f
    pub const LIGHT_ROSE: Color = Color::srgb(0.812, 0.396, 0.498); // #cf657f
    pub const BRIGHT_ROSE: Color = Color::srgb(0.929, 0.502, 0.600); // #ed8099
    pub const MAGENTA: Color = Color::srgb(0.514, 0.110, 0.365); // #831c5d
    pub const BRIGHT_MAGENTA: Color = Color::srgb(0.765, 0.141, 0.329); // #c32454

    pub const PINK: Color = Color::srgb(0.941, 0.310, 0.471); // #f04f78
    pub const LIGHT_PINK: Color = Color::srgb(0.965, 0.506, 0.506); // #f68181
    pub const LIGHT_PEACH: Color = Color::srgb(0.988, 0.655, 0.565); // #fca790
    pub const PALE_PEACH: Color = Color::srgb(0.992, 0.796, 0.690); // #fdcbb0
}
