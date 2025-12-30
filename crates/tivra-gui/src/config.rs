use crate::app::{
    constants::FONT_SIZE,
    styles::theme::{BuiltinTheme, CustomPalette, ThemeOption},
};
use serde::{Deserialize, Serialize};
use std::{fmt, path::PathBuf, str::FromStr};

// Constants for Defaults
const DEFAULT_WINDOW_WIDTH: f32 = 1024.0;
const DEFAULT_WINDOW_HEIGHT: f32 = 768.0;

// =============================================================================
// GUI Configuration
// =============================================================================

/// Main configuration struct for the User Interface.
/// Handles fonts, window decorations, scaling, and localization.
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct GuiConfig {
    pub font: AppFont,
    pub decorations: bool,
    pub scale_factor: f32,
    pub language: Option<GuiLanguage>,
    pub size_unit: ByteStandard,

    /// Runtime active theme state. Skipped during serialization.
    #[serde(skip)]
    pub theme: ThemeOption,

    /// Persisted theme configuration. Serialized as "theme".
    #[serde(rename = "theme")]
    pub saved_theme: StoredTheme,

    pub saved_custom_toml_path: Option<PathBuf>,
}

/// Defines the source of the active theme.
#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum StoredTheme {
    Builtin(BuiltinTheme),
    CustomPalette(CustomPalette),
    CustomToml(PathBuf),
}

impl Default for StoredTheme {
    fn default() -> Self {
        Self::Builtin(BuiltinTheme::default())
    }
}

impl Default for GuiConfig {
    fn default() -> Self {
        Self {
            font: AppFont::default(),
            decorations: false,
            scale_factor: 1.,
            language: Some(GuiLanguage::Hi),
            size_unit: ByteStandard::Decimal,
            theme: ThemeOption::default(),
            saved_theme: StoredTheme::default(),
            saved_custom_toml_path: None,
        }
    }
}

/// Available font choices for the application.
///
/// Serialized as a String to ensure TOML compatibility.
/// - "Laila" <-> `AppFont::Laila`
/// - "Poppins" <-> `AppFont::Poppins`
/// - Other Strings <-> `AppFont::Custom(String)`
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(into = "String", from = "String")]
pub enum AppFont {
    Laila,
    Poppins,
    Custom(String),
}

impl Default for AppFont {
    fn default() -> Self {
        Self::Laila
    }
}

// Conversion logic for Serde
impl From<String> for AppFont {
    fn from(s: String) -> Self {
        match s.as_str() {
            "Laila" => Self::Laila,
            "Poppins" => Self::Poppins,
            other => Self::Custom(other.to_string()),
        }
    }
}

impl From<AppFont> for String {
    fn from(font: AppFont) -> Self {
        match font {
            AppFont::Laila => "Laila".to_string(),
            AppFont::Poppins => "Poppins".to_string(),
            AppFont::Custom(s) => s,
        }
    }
}

// =============================================================================
// Units and Localization
// =============================================================================

/// Defines the standard used for byte unit calculations.
///
/// * `Decimal` (KB, MB): Base 1000 (SI Standard).
/// * `Binary` (KiB, MiB): Base 1024 (IEC Standard).
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ByteStandard {
    Decimal,
    Binary,
}

impl fmt::Display for ByteStandard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Decimal => write!(f, "KB, MB, GB (1000)"),
            Self::Binary => write!(f, "KiB, MiB, GiB (1024)"),
        }
    }
}

impl ByteStandard {
    pub fn multiplier(&self) -> u64 {
        match self {
            Self::Decimal => 1_000,
            Self::Binary => 1_024,
        }
    }
}

/// Supported UI Languages.
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GuiLanguage {
    En,
    Hi,
}

impl fmt::Display for GuiLanguage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GuiLanguage::En => write!(f, "English"),
            GuiLanguage::Hi => write!(f, "हिन्दी"),
        }
    }
}

impl FromStr for GuiLanguage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "en" | "english" => Ok(GuiLanguage::En),
            "hi" | "hindi" | "हिन्दी" => Ok(GuiLanguage::Hi),
            _ => Err(()),
        }
    }
}

impl GuiLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            GuiLanguage::En => "en",
            GuiLanguage::Hi => "hi",
        }
    }
}

// =============================================================================
// Window & Application State
// =============================================================================

/// Persisted state of the GUI window (position, size, layout).
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct GuiState {
    pub maximized: bool,
    pub size: WindowSize,
    pub position: WindowPosition,
    pub sidebar_collapsed: bool,
    pub sidebar_width: f32,
}

impl Default for GuiState {
    fn default() -> Self {
        Self {
            maximized: false,
            size: WindowSize {
                width: DEFAULT_WINDOW_WIDTH,
                height: DEFAULT_WINDOW_HEIGHT,
            },
            position: WindowPosition::default(),
            sidebar_collapsed: false,
            sidebar_width: FONT_SIZE * 12.,
        }
    }
}

/// Represents the physical dimensions of the window.
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub struct WindowSize {
    pub width: f32,
    pub height: f32,
}

/// Represents the startup position of the window on the screen.
#[derive(Serialize, Deserialize, Default, Clone, Copy, Debug, PartialEq)]
pub enum WindowPosition {
    #[default]
    Default,
    Centered,
    Specific(f32, f32),
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_byte_standard_multiplier() {
        assert_eq!(ByteStandard::Decimal.multiplier(), 1_000);
        assert_eq!(ByteStandard::Binary.multiplier(), 1_024);
    }

    #[test]
    fn test_gui_language_from_str() {
        assert_eq!(GuiLanguage::from_str("English"), Ok(GuiLanguage::En));
        assert_eq!(GuiLanguage::from_str("hi"), Ok(GuiLanguage::Hi));
        assert_eq!(GuiLanguage::from_str("हिन्दी"), Ok(GuiLanguage::Hi));
        assert!(GuiLanguage::from_str("invalid").is_err());
    }

    #[test]
    fn test_font_serialization_and_fallback() {
        // Test "Laila" -> AppFont::Laila
        let toml_laila = "font = 'Laila'";
        let config: GuiConfig = toml::from_str(toml_laila).expect("Failed to parse Laila");
        assert_eq!(config.font, AppFont::Laila);

        // Test "Comic Sans" -> AppFont::Custom("Comic Sans")
        let toml_custom = "font = 'Comic Sans'";
        let config: GuiConfig = toml::from_str(toml_custom).expect("Failed to parse Custom");
        assert_eq!(config.font, AppFont::Custom("Comic Sans".to_string()));

        // Test Round Trip (Serialization -> Deserialization)
        let original_font = AppFont::Custom("Victor Mono".to_string());
        let mut config = GuiConfig::default();
        config.font = original_font.clone();

        let serialized = toml::to_string(&config).expect("Failed to serialize");
        assert!(serialized.contains("Victor Mono")); // Should be a plain string in TOML

        let deserialized: GuiConfig = toml::from_str(&serialized).expect("Failed to deserialize");
        assert_eq!(deserialized.font, original_font);
    }

    #[test]
    fn test_theme_field_renaming_and_skipping() {
        // Create a config with a specific saved theme
        let mut config = GuiConfig::default();
        config.saved_theme = StoredTheme::CustomToml(PathBuf::from("themes/dark.toml"));

        // Serialize
        let toml_string = toml::to_string(&config).expect("Serialization failed");

        // Verify internal field name 'saved_theme' is NOT exposed
        assert!(!toml_string.contains("saved_theme"));

        // Verify 'theme' is present.
        // We don't check for exact formatting (inline vs table) as that is an implementation detail of the toml crate.
        assert!(toml_string.contains("theme"));

        // Round Trip (The most important check)
        let loaded: GuiConfig = toml::from_str(&toml_string).expect("Deserialization failed");

        // Verify 'saved_theme' was loaded correctly from the 'theme' key in TOML
        assert_eq!(
            loaded.saved_theme,
            StoredTheme::CustomToml(PathBuf::from("themes/dark.toml"))
        );

        // Verify the runtime 'theme' field is Default (it was skipped during serde)
        assert_eq!(loaded.theme, ThemeOption::default());
    }
}
