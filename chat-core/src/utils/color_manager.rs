use std::collections::hash_map::DefaultHasher;
use std::hash::{Hasher, Hash};

pub struct ColorGenerator;

impl ColorGenerator {
    pub fn generate_user_color(username: &str) -> String {
        let mut hasher = DefaultHasher::new();
        username.hash(&mut hasher);
        let hash = hasher.finish();

        // Color Pallet
        let colors = [
            "#FF6B6B", // Red
            "#4ECDC4", // Cyan
            "#45B7D1", // Light Blue
            "#FFA07A", // Salmon
            "#98D8C8", // light green
            "#F7DC6F", // yellow
            "#BB8FCE", // light purple
            "#85C1E2", // blue
            "#F8B739", // orange
            "#52B788", // green
            "#FF8FA3", // pink
            "#00D9FF", // neon bluen
        ];

        let index = (hash % colors.len() as u64) as usize;
        colors[index].to_string()
    }


    /// Convert hex to RGB
    pub fn hex_to_rgb(hex: &str) -> (u8, u8, u8) {
        let hex = hex.trim_start_matches("#");
        
        // Validate hex length
        if hex.len() != 6 {
            eprintln!("Warning: Invalid hex color '{}', using default gray", hex);
            return (128, 128, 128); // Default gray
        }
        
        let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(255);
        let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(255);
        let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(255);
        (r, g, b)
    }

    pub fn hex_to_ansi(hex: &str) -> String {
        let (r, g, b) = Self::hex_to_rgb(hex);
        format!("\x1b[38;2;{};{};{}m", r, g, b)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_same_username_same_color() {
        let color1 = ColorGenerator::generate_user_color("Alice");
        let color2 = ColorGenerator::generate_user_color("Alice");
        assert_eq!(color1, color2);
    }

    #[test]
    fn test_different_usernames_different_colors() {
        let color1 = ColorGenerator::generate_user_color("Alice");
        let color2 = ColorGenerator::generate_user_color("Bob");
        assert_ne!(color1, color2);
    }
}