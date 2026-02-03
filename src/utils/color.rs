pub struct Colors;

impl Colors {
    // ANSI Code for colors
    pub const RESET: &'static str = "\x1b[0m";
    pub const BOLD: &'static str = "\x1b[1m";

    // Basic colors
    pub const RED: &'static str = "\x1b[31m";
    pub const GREEN: &'static str = "\x1b[32m";
    pub const YELLOW: &'static str = "\x1b[33m";
    pub const BLUE: &'static str = "\x1b[34m";
    pub const MAGENTA: &'static str = "\x1b[35m";
    pub const CYAN: &'static str = "\x1b[36m";
    pub const WHITE: &'static str = "\x1b[37m";

    pub const BRIGHT_RED: &'static str = "\x1b[91m";
    pub const BRIGHT_GREEN: &'static str = "\x1b[92m";
    pub const BRIGHT_YELLOW: &'static str = "\x1b[93m";
    pub const BRIGHT_BLUE: &'static str = "\x1b[94m";
    pub const BRIGHT_MAGENTA: &'static str = "\x1b[95m";
    pub const BRIGHT_CYAN: &'static str = "\x1b[96m";

    // Specific color for occasions
    pub const SYSTEM: &'static str = "\x1b[93m"; // Amarelo brilhante
    pub const WHISPER: &'static str = "\x1b[95m"; // Magenta brilhante
    pub const ERROR: &'static str = "\x1b[91m"; // Vermelho brilhante
    pub const SUCCESS: &'static str = "\x1b[92m"; // Verde brilhante
    pub const INFO: &'static str = "\x1b[96m"; // Ciano brilhante


    pub fn get_user_colors() -> Vec<&'static str> {
    vec![
        Self::RED,
        Self::GREEN,
        Self::BLUE,
        Self::MAGENTA,
        Self::CYAN,
        Self::BRIGHT_RED,
        Self::BRIGHT_GREEN,
        Self::BRIGHT_BLUE,
        Self::BRIGHT_MAGENTA,
        Self::BRIGHT_CYAN,
    ]}

    pub fn get_color_by_index(index: usize) -> &'static str {
        let colors = Self::get_user_colors();
        colors[index % colors.len()]
    }

    pub fn colorize(text: &str, color: &str) -> String {
        format!("{}{}{}", color, text, Self::RESET)
    }
}