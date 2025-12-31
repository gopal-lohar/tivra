use iced::{Point, Size, window::Direction};

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
    ScaleFactorShortcut(bool),
}

#[derive(Debug, Clone)]
pub enum WindowCommand {
    CloseRequest,
    Close,
    DragStart,
    Minimize,
    ToggleMaximize(bool),
    DragResize(Direction),
}
