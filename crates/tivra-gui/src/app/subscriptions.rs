use crate::app::{
    app_state::AppState,
    message::{GlobalMessage, Message, WindowEvent},
};
use iced::{
    Subscription,
    keyboard::{Key, Modifiers},
    window,
};

impl AppState {
    pub fn animation_subscription(&self) -> Subscription<Message> {
        if self.is_animating() {
            window::frames().map(|_| Message::Animate)
        } else {
            Subscription::none()
        }
    }

    fn is_animating(&self) -> bool {
        false
    }

    pub fn events_subscription() -> Subscription<Message> {
        iced::event::listen_with(|event, _, _| match event {
            iced::Event::Window(window_event) => match window_event {
                iced::window::Event::Moved(point) => Some(Message::Global(GlobalMessage::Event(
                    WindowEvent::Moved(point),
                ))),
                iced::window::Event::Resized(size) => Some(Message::Global(GlobalMessage::Event(
                    WindowEvent::Resized(size),
                ))),
                iced::window::Event::CloseRequested => Some(Message::Global(GlobalMessage::Event(
                    WindowEvent::CloseRequested,
                ))),
                iced::window::Event::Focused => {
                    Some(Message::Global(GlobalMessage::Event(WindowEvent::Focused)))
                }
                iced::window::Event::Unfocused => Some(Message::Global(GlobalMessage::Event(
                    WindowEvent::Unfocused,
                ))),
                _ => None,
            },
            iced::Event::Keyboard(keyboard_event) => match keyboard_event {
                iced::keyboard::Event::KeyPressed { key, modifiers, .. } => match modifiers {
                    Modifiers::COMMAND => match key.as_ref() {
                        Key::Character("q") => Some(Message::Global(GlobalMessage::Event(
                            WindowEvent::CloseRequested,
                        ))),
                        Key::Character("=") => Some(Message::Global(GlobalMessage::Event(
                            WindowEvent::ScaleFactorShortcut(true),
                        ))),
                        Key::Character("-") => Some(Message::Global(GlobalMessage::Event(
                            WindowEvent::ScaleFactorShortcut(false),
                        ))),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
    }
}
