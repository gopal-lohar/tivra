use iced::{Subscription, window};

use crate::app::{app_state::AppState, message::Message};

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
}
