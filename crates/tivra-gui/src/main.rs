#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::{
    config::{GuiConfig, GuiState},
    localize::apply_language_settings,
};
use common::{
    config::{
        GUI_CONFIG_FILENAME, GUI_STATE_FILENAME, GUI_TELEMETRY_FILENAME, SILENT_CRATES_GUI,
        get_and_setup_paths, setup_config,
    },
    telemetry::{self, TelemetryConfig},
};
use tracing::{Level, error, info};

mod app;
mod config;
mod localize;

fn main() {
    let (gui_config, gui_state, app_dirs) = match get_and_setup_paths() {
        Ok(directories) => (
            setup_config(&directories.config, GUI_CONFIG_FILENAME),
            setup_config(&directories.state, GUI_STATE_FILENAME),
            Some(directories),
        ),
        Err(err) => {
            eprintln!("somethine went wrong when setting up paths err: {:?}", err);
            (GuiConfig::default(), GuiState::default(), None)
        }
    };

    let mut telemetry_config = TelemetryConfig::new(GUI_TELEMETRY_FILENAME)
        .with_level(Level::DEBUG)
        .silence_crates(SILENT_CRATES_GUI);

    if let Some(dirs) = &app_dirs {
        telemetry_config = telemetry_config.with_file_logging(&dirs.logs);
    }

    let telemetry_handle = telemetry::init(telemetry_config);

    apply_language_settings(gui_config.language.clone());

    match app::run(gui_config, gui_state, app_dirs) {
        Ok(_) => {
            info!("Application exited successfully");
            if let Ok(handle) = telemetry_handle {
                drop(handle.guard);
                if let Some(log_file) = handle.log_file {
                    let _ = std::fs::remove_file(log_file);
                }
            }
        }
        Err(err) => {
            error!("Something went wrong {err}");
        }
    };
}
