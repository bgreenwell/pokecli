use colored::Color;

pub struct Theme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub error: Color,
    pub success: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            primary: Color::Cyan,
            secondary: Color::Yellow,
            accent: Color::Green,
            error: Color::Red,
            success: Color::Green,
        }
    }
}

pub struct Themes;

impl Themes {
    pub fn pokemon_theme() -> Theme {
        Theme::default()
    }
    
    pub fn classic() -> Theme {
        Theme {
            primary: Color::Blue,
            secondary: Color::White,
            accent: Color::Yellow,
            error: Color::Red,
            success: Color::Green,
        }
    }
}