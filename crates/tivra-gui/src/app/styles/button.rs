use iced::{
    Theme,
    border::Radius,
    widget::button::{self, Status, Style},
};

use crate::app::constants::{BORDER_WIDTH, FONT_SIZE};

pub fn ghost(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let base = Style {
        border: iced::Border {
            radius: Radius::new(FONT_SIZE * 10.),
            ..Default::default()
        },
        text_color: palette.background.base.text,
        ..Default::default()
    };

    match status {
        Status::Active => base,
        Status::Hovered => button::Style {
            background: Some(iced::Background::Color(palette.background.weak.color)),
            ..base
        },
        Status::Pressed => button::Style {
            background: Some(iced::Background::Color(palette.background.weaker.color)),
            border: iced::Border {
                width: BORDER_WIDTH,
                color: palette.background.stronger.color,
                ..base.border
            },
            ..base
        },

        Status::Disabled => button::Style {
            background: base
                .background
                .map(|background| background.scale_alpha(0.5)),
            text_color: base.text_color.scale_alpha(0.5),
            ..base
        },
    }
}
