use std::collections::HashMap;

use crate::clients::transport::TransportType;

pub struct ThemeColor {
    pub foreground: String,
    pub background: String,
}

/// A theme contains color mappings for transport lines.
///
/// - `exact` contains exact matches for line names.
/// - `partial` contains prefix matches for line names (e.g., "U" matches "U1", "U2", etc.).
/// - `fallback` is used when no match is found in `exact` or `partial`.
pub struct Theme {
    pub name: String,
    pub exact: ThemeMapping<&'static str>,
    pub partial_fallback: ThemeMapping<TransportType>,
    pub fallback: ThemeColor,
}

pub fn create_default_theme() -> Theme {
    Theme {
        name: String::from("default"),
        exact: ThemeMapping::new(),
        partial_fallback: ThemeMapping::new(),
        fallback: ThemeColor {
            foreground: String::from("#000000"),
            background: String::from("#FFFFFF"),
        },
    }
}

/// A mapping from line name patterns to their corresponding theme colors.
/// The type of mapping is determined by the context in which it is used (exact or partial).
pub type ThemeMapping<T> = HashMap<T, ThemeColor>;

pub fn create_default_german_theme() -> Theme {
    let mut partial_fallback = ThemeMapping::new();

    partial_fallback.insert(
        TransportType::UndergroundTrain,
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#0065b0"),
        },
    );
    partial_fallback.insert(
        TransportType::SuburbanTrain,
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#008e4e"),
        },
    );
    partial_fallback.insert(
        TransportType::Tram,
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#d91a1a"),
        },
    );

    Theme {
        name: String::from("germany (default)"),
        exact: ThemeMapping::new(),
        partial_fallback,
        fallback: ThemeColor {
            foreground: String::from("#000000"),
            background: String::from("#FFFFFF"),
        },
    }
}

pub fn create_de_munich_theme() -> Theme {
    let mut theme = create_default_german_theme();
    theme.name = String::from("de.munich");
    theme.fallback = ThemeColor {
        foreground: String::from("#FFFFFF"),
        background: String::from("#0065cc"), // Button color mvg.de
    };

    // U-Bahn
    theme.exact.insert(
        "U1",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#3c7333"),
        },
    );
    theme.exact.insert(
        "U2",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#c3022d"),
        },
    );
    theme.exact.insert(
        "U3",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#ed6720"),
        },
    );
    theme.exact.insert(
        "U4",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#00ab85"),
        },
    );
    theme.exact.insert(
        "U5",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#bd7b00"),
        },
    );
    theme.exact.insert(
        "U6",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#0065b0"),
        },
    );
    theme.exact.insert(
        "U7",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("half::tl-br::#51832b::#c3022d"),
        },
    );
    theme.exact.insert(
        "U8",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("half::tl-br::#c3022d::#ed6720"),
        },
    );

    // S-Bahn
    theme.exact.insert(
        "S1",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#0ec1ea"),
        },
    );
    theme.exact.insert(
        "S2",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#72c042"),
        },
    );
    theme.exact.insert(
        "S3",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#7c087e"),
        },
    );
    theme.exact.insert(
        "S4",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#ef1620"),
        },
    );
    theme.exact.insert(
        "S5",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#00537f"),
        },
    );
    theme.exact.insert(
        "S6",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#008b50"),
        },
    );
    theme.exact.insert(
        "S7",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#973530"),
        },
    );
    theme.exact.insert(
        "S8",
        ThemeColor {
            foreground: String::from("#fecb00"),
            background: String::from("#000000"),
        },
    );
    theme.exact.insert(
        "S20",
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#f15a74"),
        },
    );

    // Rest
    theme.partial_fallback.insert(
        TransportType::Bus,
        ThemeColor {
            foreground: String::from("#ffffff"),
            background: String::from("#00586a"),
        },
    );

    theme.partial_fallback.insert(
        TransportType::BusRapidTransit,
        ThemeColor {
            foreground: String::from("#FFFFFF"),
            background: String::from("#2f53a0"),
        },
    );

    theme
}

// pub fn get_color_from_theme<'a>(line_name: &str, theme: &'a ThemeMapping) -> &'a ThemeColor {
//     for color_pattern in theme.keys() {
//         if color_pattern.ends_with("*") {
//             let pattern = color_pattern
//                 .split_at_checked(color_pattern.len() - 1)
//                 .unwrap_or_else(|| {
//                     panic!("Failed to remove last character from theme color pattern \"{color_pattern}\"")
//                 })
//                 .0;

//             if !line_name.starts_with(&pattern) {
//                 continue;
//             }
//         } else {
//             if !line_name.eq(color_pattern) {
//                 continue;
//             }
//         }

//         // Pattern matches
//         return theme.get(color_pattern).unwrap();
//     }

//     // Return default theme
//     if let Some(theme) = theme.get("default") {
//         return theme;
//     }

//     panic!("The requested theme doesn't contain a default value.");
// }
