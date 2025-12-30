use crate::app::message::Message;
use iced::{Element, Subscription, task::Task, time::Instant, widget::container};

pub struct AppState {
    pub now: Instant,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            now: Instant::now(),
        }
    }

    pub fn boot(self) -> (Self, Task<Message>) {
        (self, Task::none())
    }

    pub fn update(&mut self, message: Message, now: Instant) -> Task<Message> {
        self.now = now;

        match message {
            Message::Animate => Task::none(),
        }
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([self.animation_subscription()])
    }

    pub fn view(&self) -> Element<'_, Message> {
        container("").into()
    }
}
