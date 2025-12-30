use iced::{Color, Theme, theme::Palette};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Configuration enum for selecting between built-in Iced themes or a custom color palette.
///
/// The reason for storing `Theme` explicitly in `Builtin` is to allow for quicker
/// `PartialEq` comparisons, which aids efficient UI rendering skipping.
#[derive(Debug, Clone, PartialEq)]
pub enum ThemeOption {
    Builtin(Theme),
    Custom(Palette),
}

impl Default for ThemeOption {
    fn default() -> Self {
        Self::Builtin(Theme::Moonfly)
    }
}

/// List of available pre-defined themes supported by the application.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum BuiltinTheme {
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Nightfly,
    Oxocarbon,
    Ferra,
    #[serde(other)]
    #[default]
    Moonfly,
}

impl From<BuiltinTheme> for Theme {
    fn from(builtin: BuiltinTheme) -> Theme {
        match builtin {
            BuiltinTheme::Light => Theme::Light,
            BuiltinTheme::Dark => Theme::Dark,
            BuiltinTheme::Dracula => Theme::Dracula,
            BuiltinTheme::Nord => Theme::Nord,
            BuiltinTheme::SolarizedLight => Theme::SolarizedLight,
            BuiltinTheme::SolarizedDark => Theme::SolarizedDark,
            BuiltinTheme::GruvboxLight => Theme::GruvboxLight,
            BuiltinTheme::GruvboxDark => Theme::GruvboxDark,
            BuiltinTheme::CatppuccinLatte => Theme::CatppuccinLatte,
            BuiltinTheme::CatppuccinFrappe => Theme::CatppuccinFrappe,
            BuiltinTheme::CatppuccinMacchiato => Theme::CatppuccinMacchiato,
            BuiltinTheme::CatppuccinMocha => Theme::CatppuccinMocha,
            BuiltinTheme::TokyoNight => Theme::TokyoNight,
            BuiltinTheme::TokyoNightStorm => Theme::TokyoNightStorm,
            BuiltinTheme::TokyoNightLight => Theme::TokyoNightLight,
            BuiltinTheme::KanagawaWave => Theme::KanagawaWave,
            BuiltinTheme::KanagawaDragon => Theme::KanagawaDragon,
            BuiltinTheme::KanagawaLotus => Theme::KanagawaLotus,
            BuiltinTheme::Moonfly => Theme::Moonfly,
            BuiltinTheme::Nightfly => Theme::Nightfly,
            BuiltinTheme::Oxocarbon => Theme::Oxocarbon,
            BuiltinTheme::Ferra => Theme::Ferra,
        }
    }
}

// Implement conversion to Iced's Theme
impl From<ThemeOption> for Theme {
    fn from(option: ThemeOption) -> Self {
        match option {
            ThemeOption::Builtin(builtin_theme) => builtin_theme,
            ThemeOption::Custom(palette) => Theme::custom("Custom", palette),
        }
    }
}

/// A serializable mirror of `iced::theme::Palette`.
/// Required because `iced` types do not implement Serde traits by default.
/// Actually it does, but the crate iced itself doesn't expose the feature.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomPalette {
    pub name: Option<String>,
    pub background: CustomColor,
    pub text: CustomColor,
    pub primary: CustomColor,
    pub success: CustomColor,
    pub warning: CustomColor,
    pub danger: CustomColor,
}

impl From<CustomPalette> for Palette {
    fn from(val: CustomPalette) -> Self {
        Palette {
            background: val.background.into(),
            text: val.text.into(),
            primary: val.primary.into(),
            success: val.success.into(),
            warning: val.warning.into(),
            danger: val.danger.into(),
        }
    }
}

impl From<Palette> for CustomPalette {
    fn from(palette: Palette) -> CustomPalette {
        CustomPalette {
            name: None,
            background: palette.background.into(),
            text: palette.text.into(),
            primary: palette.primary.into(),
            success: palette.success.into(),
            warning: palette.warning.into(),
            danger: palette.danger.into(),
        }
    }
}

impl From<BuiltinTheme> for Palette {
    fn from(value: BuiltinTheme) -> Self {
        let theme: Theme = value.into();
        theme.palette()
    }
}

/// Wrapper around `u32` to handle Hex color serialization/deserialization.
/// Stores color as `0x00RRGGBB`.
#[derive(Debug, Clone, PartialEq)]
pub struct CustomColor(pub u32);

impl From<CustomColor> for Color {
    fn from(val: CustomColor) -> Self {
        let hex = val.0;

        // Bitwise extraction for RGB (0xRRGGBB)
        let r = ((hex >> 16) & 0xFF) as u8;
        let g = ((hex >> 8) & 0xFF) as u8;
        let b = (hex & 0xFF) as u8;

        iced::Color::from_rgb8(r, g, b)
    }
}

impl From<Color> for CustomColor {
    fn from(color: Color) -> Self {
        // Iced stores colors as f32 (0.0 - 1.0).
        // We use round() before casting to u32 to prevent precision loss (e.g., 254.99 -> 254).
        let r = (color.r * 255.0).round() as u32;
        let g = (color.g * 255.0).round() as u32;
        let b = (color.b * 255.0).round() as u32;

        Self((r << 16) | (g << 8) | b)
    }
}

impl<'de> Deserialize<'de> for CustomColor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        // Sanitize input to allow "0x" prefix or "#"
        let clean_hex = s.trim_start_matches("0x").trim_start_matches('#');

        u32::from_str_radix(clean_hex, 16)
            .map(CustomColor)
            .map_err(serde::de::Error::custom)
    }
}

impl Serialize for CustomColor {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Serialize as standard Hex string (e.g., "#FF00AA")
        let hex_string = format!("#{:<06X}", self.0);
        serializer.serialize_str(&hex_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing::debug;

    fn init_tracing() {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
    }

    #[test]
    fn test_hex_string_parsing_toml() {
        init_tracing();

        // Wrapper struct needed because TOML requires key-values at top level
        #[derive(Deserialize, Debug)]
        struct TestWrapper {
            color: CustomColor,
        }

        let inputs = vec![
            ("color = \"#FF0000\"", 0xFF0000),
            ("color = \"0x00FF00\"", 0x00FF00),
            ("color = \"0000FF\"", 0x0000FF),
            // Test case formatting tolerance
            ("color = \"#abcdef\"", 0xABCDEF),
        ];

        for (toml_input, expected_val) in inputs {
            let wrapper: TestWrapper = toml::from_str(toml_input).expect("Failed to parse TOML");
            debug!(input = %toml_input, parsed = ?wrapper.color, "Parsed color");
            assert_eq!(wrapper.color.0, expected_val);
        }
    }

    #[test]
    fn test_iced_color_conversion_roundtrip() {
        let original_iced = Color::from_rgb8(255, 128, 0); // Orange-ish

        let custom: CustomColor = original_iced.into();
        assert_eq!(
            custom.0, 0xFF8000,
            "RGB values did not map to Hex correctly"
        );

        let back_to_iced: Color = custom.into();
        assert_eq!(back_to_iced, original_iced);
    }
}
