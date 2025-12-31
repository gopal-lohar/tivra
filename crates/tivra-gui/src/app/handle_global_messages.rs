use crate::{
    app::{
        app_state::AppState,
        message::{GlobalMessage, Message, WindowCommand, WindowEvent},
    },
    config::{WindowPosition, WindowSize},
};
use common::config::save_config_async;
use iced::{Task, window};
use tracing::error;

impl AppState {
    pub fn handle_global_messages(&mut self, message: GlobalMessage) -> Task<Message> {
        match message {
            GlobalMessage::Event(event) => {
                match event {
                    WindowEvent::Moved(point) => {
                        self.gui_state.position = WindowPosition::Specific(point.x, point.y);
                        Task::none()
                    }
                    WindowEvent::Resized(size) => window::latest()
                        .and_then(window::is_maximized)
                        .map(move |maximized| {
                            Message::Global(GlobalMessage::Event(WindowEvent::ResizedInner(
                                size, maximized,
                            )))
                        }),
                    WindowEvent::ResizedInner(size, maximized) => {
                        if !maximized {
                            self.gui_state.size = WindowSize {
                                width: size.width,
                                height: size.height,
                            };
                        }
                        self.gui_state.maximized = maximized;
                        Task::none()
                    }
                    WindowEvent::CloseRequested => {
                        if let Some(dirs) = &self.app_dirs {
                            let state_file_path = dirs.gui_state_file();
                            let config_file_path = dirs.gui_config_file();
                            let gui_state = self.gui_state.clone();
                            let gui_config = self.config.clone();

                            Task::perform(
                                async move {
                                    if let Err(e) =
                                        save_config_async(&state_file_path, &gui_state).await
                                    {
                                        error!(path = ?state_file_path, error = ?e, "Failed to persist default state to disk.");
                                    }

                                    if let Err(e) =
                                        save_config_async(&config_file_path, &gui_config).await
                                    {
                                        error!(path = ?config_file_path, error = ?e, "Failed to persist default state to disk.");
                                    }
                                    ()
                                },
                                |_| Message::Global(GlobalMessage::Command(WindowCommand::Close)),
                            )
                        } else {
                            Task::done(Message::Global(GlobalMessage::Command(
                                WindowCommand::Close,
                            )))
                        }
                    }
                    WindowEvent::Focused => {
                        self.focused = true;
                        Task::none()
                    }
                    WindowEvent::Unfocused => {
                        self.focused = false;
                        Task::none()
                    }
                    WindowEvent::ScaleFactorShortcut(increase) => {
                        let scale_factor = self.config.scale_factor;
                        if !(scale_factor > 2.99 && increase || scale_factor < 0.31 && !increase) {
                            let delta = if increase { 0.1 } else { -0.1 };
                            self.config.scale_factor += delta;
                        }
                        Task::none()
                    }
                }
            }
            GlobalMessage::Command(command) => match command {
                WindowCommand::CloseRequest => Task::done(Message::Global(GlobalMessage::Event(
                    WindowEvent::CloseRequested,
                ))),
                WindowCommand::Close => window::latest().and_then(window::close),
                WindowCommand::DragStart => window::latest().and_then(window::drag),
                WindowCommand::DragResize(direction) => {
                    window::latest().and_then(move |id| window::drag_resize(id, direction))
                }
                WindowCommand::Minimize => {
                    window::latest().and_then(|id| window::minimize(id, true))
                }
                WindowCommand::ToggleMaximize(maximized) => {
                    window::latest().and_then(move |id| window::maximize(id, !maximized))
                }
            },
        }
    }
}
