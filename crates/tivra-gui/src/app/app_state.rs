use crate::{
    app::{message::Message, styles::theme::ThemeOption},
    config::{GuiConfig, GuiState},
};
use common::config::AppDirs;
use iced::{Element, Subscription, Theme, task::Task, time::Instant, widget::container};

pub struct AppState {
    now: Instant,
    pub config: GuiConfig,
    pub gui_state: GuiState,
    pub app_dirs: Option<AppDirs>,
    pub focused: bool,
}

impl AppState {
    pub fn new(config: GuiConfig, gui_state: GuiState, app_dirs: Option<AppDirs>) -> Self {
        Self {
            now: Instant::now(),
            config,
            gui_state,
            app_dirs,
            focused: true,
        }
    }

    pub fn boot(self) -> (Self, Task<Message>) {
        (self, Task::none())
    }

    pub fn update(&mut self, message: Message, now: Instant) -> Task<Message> {
        self.now = now;

        match message {
            Message::Animate => Task::none(),
            Message::Global(global_message) => self.handle_global_messages(global_message),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([self.animation_subscription(), Self::events_subscription()])
    }

    pub fn view(&self) -> Element<'_, Message> {
        container("").into()
    }

    pub fn theme(&self) -> Theme {
        match self.config.theme.clone() {
            ThemeOption::Builtin(theme) => theme,
            ThemeOption::Custom(palette) => Theme::custom("Custom", palette),
        }
    }

    pub fn scale_factor(&self) -> f32 {
        self.config.scale_factor
    }
}
