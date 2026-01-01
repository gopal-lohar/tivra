use iced::{Color, Theme, widget::svg};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Generates the `Icon` enum and the logic to load SVG data from files.
/// This macro ensures the enum variants and file paths remain in sync
/// and validates file existence at compile time.
macro_rules! define_icons {
    (
        $(
            $variant:ident => $filename:literal
        ),+ $(,)?
    ) => {
        /// All available icons in the application.
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum Icon {
            $( $variant ),+
        }

        impl Icon {
            /// Returns a static handle to the SVG data, loading it into memory only once.
            pub fn handle(&self) -> svg::Handle {
                static ICON_CACHE: OnceLock<HashMap<Icon, svg::Handle>> = OnceLock::new();

                let cache = ICON_CACHE.get_or_init(|| {
                    HashMap::from([
                        $(
                            (
                                Icon::$variant,
                                // `concat!` constructs the path at compile time.
                                // `include_bytes!` ensures the file exists relative to this source file.
                                svg::Handle::from_memory(
                                    include_bytes!(concat!("../../../assets/icons/", $filename)).as_slice()
                                )
                            )
                        ),+
                    ])
                });

                // Safe unwrap: The map is generated from the exact same list as the Enum variants.
                cache.get(self).unwrap().clone()
            }
        }
    };
}

define_icons! {
    Add         => "add.svg",
    AppIcon     => "logo.svg",
    ArrowBack   => "arrow_back.svg",
    Check       => "check.svg",
    Close       => "close.svg",
    DarkMode    => "dark_mode.svg",
    Delete      => "delete.svg",
    Directory   => "directory.svg",
    Downloading => "downloading.svg",
    Error       => "error.svg",
    Github      => "github.svg",
    Info        => "info_i.svg",
    LightMode   => "light_mode.svg",
    Minimize    => "minimize.svg",
    Maximize    => "maximize.svg",
    Menu        => "menu.svg",
    Pause       => "pause.svg",
    Play        => "play.svg",
    Restore     => "restore.svg",
    Settings    => "settings.svg",
    Warning     => "warning.svg",
}

/// Creates a themed SVG widget for the given icon.
///
/// # Arguments
/// * `size` - The width and height of the icon.
/// * `color` - Optional tint. If `None`, uses `theme.palette().text`.
pub fn themed_icon<'a, T>(icon: Icon, size: f32, color: Option<Color>) -> iced::Element<'a, T> {
    svg(icon.handle())
        .width(size)
        .height(size)
        .style(move |theme: &Theme, _| svg::Style {
            color: Some(color.unwrap_or(theme.palette().text)),
        })
        .into()
}
