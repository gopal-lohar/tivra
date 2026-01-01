use crate::{
    app::{
        components::decorations::{resize_layer, titlebar_view},
        message::Message,
        styles::theme::ThemeOption,
    },
    config::{GuiConfig, GuiState},
};
use common::config::AppDirs;
use iced::{
    Element, Length, Subscription, Theme,
    task::Task,
    time::Instant,
    widget::{Column, Row, Stack, container},
};

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
        let mut main_view: Vec<Element<Message>> = vec![];

        if !self.config.decorations {
            main_view.push(titlebar_view(self.gui_state.maximized, self.focused))
        }

        let shell: Vec<Element<Message>> = vec![];

        main_view.push(
            Row::from_vec(shell)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        );

        let mut root_stack: Vec<Element<Message>> = vec![
            Column::from_vec(main_view)
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        ];

        if !self.config.decorations {
            root_stack.push(resize_layer().into());
        }

        container(
            Stack::from_vec(root_stack)
                .width(Length::Fill)
                .height(Length::Fill),
        )
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                text_color: Some(palette.background.base.text),
                background: Some(palette.background.base.color.into()),
                ..Default::default()
            }
        })
        .into()
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
