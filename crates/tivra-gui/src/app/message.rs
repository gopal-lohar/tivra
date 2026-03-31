use iced::{Point, Size, window::Direction};

use crate::app::app_state::Page;

#[derive(Debug, Clone)]
pub enum Message {
    Animate,
    Global(GlobalMessage),
}

#[derive(Debug, Clone)]
pub enum GlobalMessage {
    Event(WindowEvent),
    Command(WindowCommand),
}

#[derive(Debug, Clone)]
pub enum WindowEvent {
    Moved(Point),
    Resized(Size),
    ResizedInner(Size, bool),
    CloseRequested,
    Focused,
    Unfocused,
    ScaleFactorShortcut(ScaleFactorShortcut),
}

#[derive(Debug, Clone)]
pub enum ScaleFactorShortcut {
    Increase,
    Decrease,
    Reset,
}

#[derive(Debug, Clone)]
pub enum WindowCommand {
    CloseRequest,
    Close,
    DragStart,
    Minimize,
    ToggleMaximize(bool),
    DragResize(Direction),
    Navigate(Page),
    ToggleSidebar,
}
