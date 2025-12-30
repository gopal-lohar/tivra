use crate::config::GuiLanguage;
use i18n_embed::unic_langid::LanguageIdentifier;
use i18n_embed::{
    DefaultLocalizer, LanguageLoader, Localizer,
    fluent::{FluentLanguageLoader, fluent_language_loader},
};
use rust_embed::RustEmbed;
use std::collections::HashSet;
use std::str::FromStr;
use std::sync::LazyLock;
use tracing::{error, info, warn};

/// Embeds the `i18n` directory into the binary.
/// Expects a folder structure like `i18n/en-US/main.ftl`.
#[derive(RustEmbed)]
#[folder = "i18n/"]
struct Localizations;

/// Global singleton for the Fluent language loader.
/// initialized lazily with the fallback language.
pub static LANGUAGE_LOADER: LazyLock<FluentLanguageLoader> = LazyLock::new(|| {
    let loader: FluentLanguageLoader = fluent_language_loader!();

    loader
        .load_fallback_language(&Localizations)
        .expect("Error while loading fallback language");

    loader
});

/// Macro for localizing messages.
///
/// # Usage
/// ```ignore
/// let msg = fl!("hello-world");
/// let msg_with_args = fl!("greeting", name = "Alice");
/// ```
#[macro_export]
macro_rules! fl {
    ($message_id:literal) => {{
        i18n_embed_fl::fl!($crate::localize::LANGUAGE_LOADER, $message_id)
    }};

    ($message_id:literal, $($args:expr),*) => {{
        i18n_embed_fl::fl!($crate::localize::LANGUAGE_LOADER, $message_id, $($args), *)
    }};
}

/// Creates a new Localizer using the global loader and embedded assets.
pub fn localizer() -> Box<dyn Localizer> {
    Box::from(DefaultLocalizer::new(&*LANGUAGE_LOADER, &Localizations))
}

/// Scans embedded files to determine which language codes are available.
/// Returns a sorted list of unique language codes (e.g., `["en-US", "de"]`).
pub fn get_available_languages() -> Vec<String> {
    let mut languages = HashSet::new();

    for file_path in Localizations::iter() {
        let path_str = file_path.as_ref();
        // Assumes structure: "lang_code/file.ftl"
        let parts: Vec<&str> = path_str.split('/').collect();

        if let Some(lang_code) = parts.first() {
            // Verify it is a valid Unicode Language Identifier
            if LanguageIdentifier::from_str(lang_code).is_ok() {
                languages.insert(lang_code.to_string());
            }
        }
    }

    let mut result: Vec<String> = languages.into_iter().collect();
    result.sort();
    result
}

/// Applies the language based on the following priority:
/// 1. Explicit `config_lang` (if valid).
/// 2. System default (if detected and supported).
/// 3. Fallback language (defined in `i18n-embed.toml` or loader default).
pub fn apply_language_settings(config_lang: Option<GuiLanguage>) {
    let available_languages = get_available_languages();

    if let Some(gui_lang) = config_lang {
        let lang_code = gui_lang.as_str();

        if available_languages.contains(&lang_code.to_string()) {
            match LanguageIdentifier::from_str(lang_code) {
                Ok(lang_id) => match localizer().select(&[lang_id]) {
                    Ok(_) => {
                        info!("Successfully applied configured language: {}", lang_code);
                        return;
                    }
                    Err(e) => {
                        error!("Failed to apply configured language '{}': {}", lang_code, e);
                    }
                },
                Err(e) => {
                    error!("Config language '{}' is malformed: {}", lang_code, e);
                }
            }
        } else {
            warn!(
                "Configured language '{}' is not supported/missing. Available: {:?}",
                lang_code, available_languages
            );
        }
    }

    info!("Using system default language settings.");

    let localizer = localizer();
    let requested_languages = i18n_embed::DesktopLanguageRequester::requested_languages();

    if let Err(error) = localizer.select(&requested_languages) {
        error!("Failed to load system default language: {}", error);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_loader_initialization() {
        // Ensures the lazy static can initialize without panicking
        // and that the fallback language is loaded correctly.
        assert!(!LANGUAGE_LOADER.current_languages().is_empty());
    }

    #[test]
    fn test_get_available_languages() {
        // We cannot guarantee specific languages exist in the test env without the assets,
        // but we can ensure the function runs without panic and returns a sorted vector.
        let languages = get_available_languages();

        // If we have any languages, ensure they are valid identifiers
        for lang in &languages {
            assert!(LanguageIdentifier::from_str(lang).is_ok());
        }

        // Ensure sorted
        let mut sorted = languages.clone();
        sorted.sort();
        assert_eq!(languages, sorted);
    }
}
