use crate::{
    app::{
        app_state::AppState,
        constants::{FONT_SIZE, LAILA_BYTES, POPPINS_BYTES, WINDOW_ICON},
    },
    config::{AppFont, GuiConfig, GuiState, WindowPosition},
};
use common::{APP_ID, APPNAME_TITLECASE, config::AppDirs};
use iced::window::Position;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use iced::window::settings::PlatformSpecific;
use image::ImageFormat;
use std::borrow::Cow;

pub fn run(config: GuiConfig, state: GuiState, app_dirs: Option<AppDirs>) -> iced::Result {
    let font_name: String = config.font.clone().into();
    let font_name: &'static str = Box::leak(font_name.into_boxed_str());

    let config_clone = config.clone();
    let state_clone = state.clone();
    let app_dirs_clone = app_dirs.clone();

    iced::application::timed(
        move || {
            AppState::new(
                config_clone.clone(),
                state_clone.clone(),
                app_dirs_clone.clone(),
            )
            .boot()
        },
        AppState::update,
        AppState::subscription,
        AppState::view,
    )
    .title(APPNAME_TITLECASE)
    .settings(iced::Settings {
        id: Some(APP_ID.into()),
        fonts: match config.font {
            AppFont::Laila => vec![Cow::Borrowed(LAILA_BYTES)],
            AppFont::Poppins => vec![Cow::Borrowed(POPPINS_BYTES)],
            AppFont::Custom(_) => vec![],
        },
        default_font: iced::Font::with_name(font_name),
        default_text_size: iced::Pixels(FONT_SIZE),
        ..Default::default()
    })
    .window(iced::window::Settings {
        size: iced::Size::new(state.size.width, state.size.height),
        icon: iced::window::icon::from_file_data(WINDOW_ICON, Some(ImageFormat::Png)).ok(),
        #[cfg(not(target_os = "macos"))]
        decorations: config.decorations,
        #[cfg(target_os = "macos")]
        decorations: false,
        position: match state.position {
            WindowPosition::Default => Position::Default,
            WindowPosition::Centered => Position::Centered,
            WindowPosition::Specific(x, y) => Position::Specific(iced::Point::new(x, y)),
        },
        maximized: state.maximized,
        #[cfg(target_os = "linux")]
        platform_specific: PlatformSpecific {
            application_id: String::from(APP_ID),
            ..PlatformSpecific::default()
        },
        #[cfg(target_os = "macos")]
        platform_specific: PlatformSpecific {
            title_hidden: !config.decorations,
            titlebar_transparent: !config.decorations,
            fullsize_content_view: !config.decorations,
        },
        exit_on_close_request: false,
        ..Default::default()
    })
    .theme(AppState::theme)
    .scale_factor(AppState::scale_factor)
    .run()
}
