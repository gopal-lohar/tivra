use crate::app::constants::{BORDER_WIDTH, FONT_SIZE, RESIZE_CORNER_SIZE, RESIZE_EDGE_SIZE};
use crate::app::message::{GlobalMessage, Message, WindowCommand};
use crate::app::styles::icons::{Icon, IconStyle, svg_icon};
use common::APPNAME_TITLECASE;
use iced::widget::Row;
use iced::{
    Alignment, Element, Length, Theme,
    mouse::Interaction,
    widget::{button, column, container, mouse_area, row, space, text},
    window::Direction,
};
use iced::{Border, Padding, Pixels};

pub fn titlebar_view<'a>(maximized: bool, focused: bool) -> Element<'a, Message> {
    #[cfg(not(target_os = "macos"))]
    let titlebar_buttons = {
        let title_bar_button = move |icon: Icon, on_press: Message| {
            button(svg_icon(icon, FONT_SIZE, IconStyle::Auto))
                .padding(FONT_SIZE * 0.5)
                .style(move |theme, status| {
                    let palette = theme.extended_palette();
                    let base = button::Style {
                        text_color: palette.background.weakest.text,
                        border: Border::rounded(Border::default(), FONT_SIZE),
                        ..Default::default()
                    };
                    match status {
                        button::Status::Hovered => button::Style {
                            background: Some(palette.background.weaker.color.into()),
                            ..base
                        },
                        _ => base,
                    }
                })
                .on_press(on_press)
        };

        row![
            title_bar_button(
                Icon::Minimize,
                Message::Global(GlobalMessage::Command(WindowCommand::Minimize))
            ),
            title_bar_button(
                if maximized {
                    Icon::Restore
                } else {
                    Icon::Maximize
                },
                Message::Global(GlobalMessage::Command(WindowCommand::ToggleMaximize(
                    maximized
                )))
            ),
            title_bar_button(
                Icon::Close,
                Message::Global(GlobalMessage::Command(WindowCommand::CloseRequest))
            ),
        ]
    };

    let app_logo_and_title: Element<Message> = row![
        container(svg_icon(
            Icon::AppIcon,
            FONT_SIZE * 1.5,
            IconStyle::Original
        ))
        .padding(Padding::new(0.).bottom(FONT_SIZE * 0.125)),
        text(APPNAME_TITLECASE)
    ]
    .spacing(if cfg!(target_os = "macos") {
        0.0
    } else {
        FONT_SIZE * 0.25
    })
    .align_y(Alignment::Center)
    .into();

    let mut titlebar_row: Vec<Element<Message>> = vec![];

    titlebar_row.push(
        mouse_area(
            column![app_logo_and_title,]
                .align_x(if cfg!(target_os = "macos") {
                    Alignment::Center
                } else {
                    Alignment::Start
                })
                .width(Length::Fill),
        )
        .on_press(Message::Global(GlobalMessage::Command(
            WindowCommand::DragStart,
        )))
        .into(),
    );

    #[cfg(not(target_os = "macos"))]
    titlebar_row.push(titlebar_buttons.into());

    let title_bar = container(
        Row::from_vec(titlebar_row)
            .padding(
                Padding::new(0.)
                    .left(FONT_SIZE * 0.5)
                    .right(FONT_SIZE * 0.5),
            )
            .width(Length::Fill)
            .height(if cfg!(target_os = "macos") {
                FONT_SIZE * 2.
            } else {
                FONT_SIZE * 3.
            })
            .align_y(Alignment::Center),
    )
    .width(Length::Fill)
    .style(move |theme: &Theme| {
        let palette = theme.extended_palette();
        container::Style {
            background: Some(if !focused {
                palette.background.weakest.color.into()
            } else {
                palette.background.base.color.into()
            }),
            ..Default::default()
        }
    });

    let separator = container("")
        .width(Length::Fill)
        .height(Pixels::from(BORDER_WIDTH))
        .style(|theme: &Theme| container::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            ..Default::default()
        });

    column![title_bar, separator].into()
}

pub fn resize_layer<'a>() -> Element<'a, Message> {
    column![
        row![
            resize_element(Direction::NorthWest),
            resize_element(Direction::North),
            resize_element(Direction::NorthEast),
        ],
        row![
            resize_element(Direction::West),
            space().width(Length::Fill).height(Length::Fill),
            resize_element(Direction::East),
        ]
        .height(Length::Fill),
        row![
            resize_element(Direction::SouthWest),
            resize_element(Direction::South),
            resize_element(Direction::SouthEast),
        ]
        .align_y(Alignment::End),
    ]
    .into()
}

fn resize_element<'a>(direction: Direction) -> Element<'a, Message> {
    mouse_area(container(
        space()
            .width(match direction {
                Direction::NorthEast
                | Direction::NorthWest
                | Direction::SouthEast
                | Direction::SouthWest => Length::from(RESIZE_CORNER_SIZE),
                Direction::North | Direction::South => Length::Fill,
                Direction::East | Direction::West => Length::from(RESIZE_EDGE_SIZE),
            })
            .height(match direction {
                Direction::NorthEast
                | Direction::NorthWest
                | Direction::SouthEast
                | Direction::SouthWest => Length::from(RESIZE_CORNER_SIZE),
                Direction::North | Direction::South => Length::from(RESIZE_EDGE_SIZE),
                Direction::East | Direction::West => Length::Fill,
            }),
    ))
    .interaction(match direction {
        Direction::NorthWest | Direction::SouthEast => Interaction::ResizingDiagonallyDown,
        Direction::NorthEast | Direction::SouthWest => Interaction::ResizingDiagonallyUp,
        Direction::North | Direction::South => Interaction::ResizingVertically,
        Direction::East | Direction::West => Interaction::ResizingHorizontally,
    })
    .on_press(Message::Global(GlobalMessage::Command(
        WindowCommand::DragResize(direction),
    )))
    .into()
}
