use common::APPNAME_TITLECASE;

use crate::app::app_state::AppState;

pub fn run() -> iced::Result {
    iced::application::timed(
        move || AppState::new().boot(),
        AppState::update,
        AppState::subscription,
        AppState::view,
    )
    .title(APPNAME_TITLECASE)
    .run()
}
