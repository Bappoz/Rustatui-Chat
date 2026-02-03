pub struct Formatter;

impl Formatter {
    // Right Alignment
    pub fn align_right(text: &str, width: usize) -> String {
        let text_len = text.chars().count();
        if text_len >= width {
            return text.to_string();
        }
        format!("{}{}", " ".repeat(width - text_len), text)
    }

    // Formats own message
    pub fn format_own_message(message: &str, terminal_width: usize) -> String {
        let prefix = "You: ";
        let full_msg = format!("{}{}", prefix, message);
        Self::align_right(&full_msg, terminal_width)
    }

    pub fn format_other_message(name: &str, message: &str, color: &str) -> String {
        use crate::utils::color::Colors;
        let colored_name = Colors::colorize(name, color);
        format!("{}: {}", colored_name, message)
    }
}