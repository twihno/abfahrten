use std::{collections::HashMap, hash::Hash};

pub struct ThemeColor {
    foreground: String,
    background: String,
}

pub type Theme = HashMap<String, ThemeColor>;

pub fn get_color_from_theme<'a>(line_name: &str, theme: &'a Theme) -> &'a ThemeColor {
    for color_pattern in theme.keys() {
        if color_pattern.ends_with("*") {
            let pattern = color_pattern
                .split_at_checked(color_pattern.len() - 1)
                .unwrap_or_else(|| {
                    panic!("Failed to remove last character from theme color pattern \"{color_pattern}\"")
                })
                .0;

            if !line_name.starts_with(&pattern) {
                continue;
            }
        } else {
            if !line_name.eq(color_pattern) {
                continue;
            }
        }

        // Pattern matches
        return theme.get(color_pattern).unwrap();
    }

    // Return default theme
    if let Some(theme) = theme.get("default") {
        return theme;
    }

    panic!("The requested theme doesn't contain a default value.");
}
