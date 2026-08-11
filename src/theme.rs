use crate::cli::ThemeMode;
use image::Rgb;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorRgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ColorRgb {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn to_rgb(self) -> Rgb<u8> {
        Rgb([self.r, self.g, self.b])
    }
}

#[derive(Debug, Clone)]
pub struct ThemePalette {
    pub background: ColorRgb,
    pub card_bg: ColorRgb,
    pub border: ColorRgb,
    pub text_primary: ColorRgb,
    pub text_secondary: ColorRgb,
    pub accent_main: ColorRgb,
    pub accent_secondary: ColorRgb,
    pub addition: ColorRgb,
    pub deletion: ColorRgb,
    pub node_default: ColorRgb,
    pub node_active: ColorRgb,
    pub line_color: ColorRgb,
    pub author_colors: Vec<ColorRgb>,
}

impl ThemePalette {
    pub fn from_mode(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Dark => Self::dark(),
            ThemeMode::Light => Self::light(),
            ThemeMode::Cyber => Self::cyber(),
            ThemeMode::Dracula => Self::dracula(),
            ThemeMode::Solarized => Self::solarized(),
            ThemeMode::Neon => Self::neon(),
        }
    }

    pub fn dark() -> Self {
        Self {
            background: ColorRgb::new(18, 22, 30),
            card_bg: ColorRgb::new(28, 34, 46),
            border: ColorRgb::new(45, 55, 72),
            text_primary: ColorRgb::new(240, 244, 248),
            text_secondary: ColorRgb::new(160, 174, 192),
            accent_main: ColorRgb::new(99, 102, 241),
            accent_secondary: ColorRgb::new(168, 85, 247),
            addition: ColorRgb::new(34, 197, 94),
            deletion: ColorRgb::new(239, 68, 68),
            node_default: ColorRgb::new(71, 85, 105),
            node_active: ColorRgb::new(245, 158, 11),
            line_color: ColorRgb::new(51, 65, 85),
            author_colors: vec![
                ColorRgb::new(99, 102, 241),
                ColorRgb::new(168, 85, 247),
                ColorRgb::new(236, 72, 153),
                ColorRgb::new(14, 165, 233),
                ColorRgb::new(20, 184, 166),
                ColorRgb::new(245, 158, 11),
            ],
        }
    }

    pub fn light() -> Self {
        Self {
            background: ColorRgb::new(248, 250, 252),
            card_bg: ColorRgb::new(255, 255, 255),
            border: ColorRgb::new(226, 232, 240),
            text_primary: ColorRgb::new(15, 23, 42),
            text_secondary: ColorRgb::new(100, 116, 139),
            accent_main: ColorRgb::new(79, 70, 229),
            accent_secondary: ColorRgb::new(147, 51, 234),
            addition: ColorRgb::new(22, 163, 74),
            deletion: ColorRgb::new(220, 38, 38),
            node_default: ColorRgb::new(203, 213, 225),
            node_active: ColorRgb::new(217, 119, 6),
            line_color: ColorRgb::new(203, 213, 225),
            author_colors: vec![
                ColorRgb::new(79, 70, 229),
                ColorRgb::new(147, 51, 234),
                ColorRgb::new(219, 39, 119),
                ColorRgb::new(2, 132, 199),
                ColorRgb::new(13, 148, 136),
                ColorRgb::new(217, 119, 6),
            ],
        }
    }

    pub fn cyber() -> Self {
        Self {
            background: ColorRgb::new(10, 10, 20),
            card_bg: ColorRgb::new(20, 24, 40),
            border: ColorRgb::new(0, 240, 255),
            text_primary: ColorRgb::new(255, 255, 255),
            text_secondary: ColorRgb::new(0, 200, 230),
            accent_main: ColorRgb::new(0, 240, 255),
            accent_secondary: ColorRgb::new(255, 0, 128),
            addition: ColorRgb::new(0, 255, 170),
            deletion: ColorRgb::new(255, 0, 90),
            node_default: ColorRgb::new(40, 50, 80),
            node_active: ColorRgb::new(255, 230, 0),
            line_color: ColorRgb::new(0, 120, 160),
            author_colors: vec![
                ColorRgb::new(0, 240, 255),
                ColorRgb::new(255, 0, 128),
                ColorRgb::new(255, 230, 0),
                ColorRgb::new(0, 255, 170),
                ColorRgb::new(180, 0, 255),
                ColorRgb::new(255, 120, 0),
            ],
        }
    }

    pub fn dracula() -> Self {
        Self {
            background: ColorRgb::new(40, 42, 54),
            card_bg: ColorRgb::new(68, 71, 90),
            border: ColorRgb::new(98, 114, 164),
            text_primary: ColorRgb::new(248, 248, 242),
            text_secondary: ColorRgb::new(189, 147, 249),
            accent_main: ColorRgb::new(189, 147, 249),
            accent_secondary: ColorRgb::new(255, 121, 198),
            addition: ColorRgb::new(80, 250, 123),
            deletion: ColorRgb::new(255, 85, 85),
            node_default: ColorRgb::new(98, 114, 164),
            node_active: ColorRgb::new(255, 184, 108),
            line_color: ColorRgb::new(98, 114, 164),
            author_colors: vec![
                ColorRgb::new(189, 147, 249),
                ColorRgb::new(255, 121, 198),
                ColorRgb::new(139, 233, 253),
                ColorRgb::new(80, 250, 123),
                ColorRgb::new(255, 184, 108),
                ColorRgb::new(241, 250, 140),
            ],
        }
    }

    pub fn solarized() -> Self {
        Self {
            background: ColorRgb::new(0, 43, 54),
            card_bg: ColorRgb::new(7, 54, 66),
            border: ColorRgb::new(88, 110, 117),
            text_primary: ColorRgb::new(253, 246, 227),
            text_secondary: ColorRgb::new(143, 161, 161),
            accent_main: ColorRgb::new(38, 139, 210),
            accent_secondary: ColorRgb::new(211, 54, 130),
            addition: ColorRgb::new(133, 153, 0),
            deletion: ColorRgb::new(220, 50, 47),
            node_default: ColorRgb::new(88, 110, 117),
            node_active: ColorRgb::new(181, 137, 0),
            line_color: ColorRgb::new(88, 110, 117),
            author_colors: vec![
                ColorRgb::new(38, 139, 210),
                ColorRgb::new(211, 54, 130),
                ColorRgb::new(42, 161, 152),
                ColorRgb::new(133, 153, 0),
                ColorRgb::new(181, 137, 0),
                ColorRgb::new(203, 75, 22),
            ],
        }
    }

    pub fn neon() -> Self {
        Self {
            background: ColorRgb::new(15, 15, 15),
            card_bg: ColorRgb::new(30, 30, 30),
            border: ColorRgb::new(50, 50, 50),
            text_primary: ColorRgb::new(255, 255, 255),
            text_secondary: ColorRgb::new(200, 200, 200),
            accent_main: ColorRgb::new(57, 255, 20),
            accent_secondary: ColorRgb::new(255, 0, 255),
            addition: ColorRgb::new(57, 255, 20),
            deletion: ColorRgb::new(255, 36, 0),
            node_default: ColorRgb::new(70, 70, 70),
            node_active: ColorRgb::new(255, 255, 0),
            line_color: ColorRgb::new(80, 80, 80),
            author_colors: vec![
                ColorRgb::new(57, 255, 20),
                ColorRgb::new(255, 0, 255),
                ColorRgb::new(0, 255, 255),
                ColorRgb::new(255, 255, 0),
                ColorRgb::new(255, 102, 0),
                ColorRgb::new(204, 0, 255),
            ],
        }
    }

    pub fn get_author_color(&self, index: usize) -> ColorRgb {
        if self.author_colors.is_empty() {
            self.accent_main
        } else {
            self.author_colors[index % self.author_colors.len()]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_palettes() {
        let modes = vec![
            ThemeMode::Dark,
            ThemeMode::Light,
            ThemeMode::Cyber,
            ThemeMode::Dracula,
            ThemeMode::Solarized,
            ThemeMode::Neon,
        ];

        for mode in modes {
            let palette = ThemePalette::from_mode(mode);
            assert!(!palette.author_colors.is_empty());
            let c = palette.get_author_color(0);
            assert_eq!(c.to_rgb(), palette.author_colors[0].to_rgb());
        }
    }
}
