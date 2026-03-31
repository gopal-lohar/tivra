use crate::app::app_state::{Page, ShowDownloads};
use crate::app::message::{GlobalMessage, WindowCommand};
use crate::app::styles::icons::IconStyle;
use crate::fl;
use crate::{
    app::constants::{BORDER_WIDTH, FONT_SIZE},
    app::styles::{
        button,
        icons::{Icon, svg_icon},
    },
};
use iced::{
    Alignment, Animation, Element, Length, Padding, Pixels, Theme,
    time::Instant,
    widget::{
        Space,
        button::{Button, Status},
        column, container, row, text,
    },
};

const ICON_SIZE: f32 = 1.25 * FONT_SIZE;
const SIDEBAR_PADDING: f32 = 0.5 * FONT_SIZE;
pub const SIDEBAR_WIDTH: f32 = 3.25 * FONT_SIZE;
const BUTTON_PADDING: f32 = (SIDEBAR_WIDTH - ICON_SIZE - (2. * SIDEBAR_PADDING)) / 2.;

pub struct SidebarContext {
    pub current_page: Page,
    pub sidebar_width: f32,
    pub sidebar_state: Animation<bool>,
}

pub fn sidebar_view<'a>(state: SidebarContext, now: Instant) -> Element<'a, GlobalMessage> {
    let separator = container("")
        .width(Pixels::from(BORDER_WIDTH))
        .height(Length::Fill)
        .style(|theme: &Theme| container::Style {
            background: Some(theme.extended_palette().background.weak.color.into()),
            ..Default::default()
        });

    row![sidebar_view_inner(state, now), separator]
        .height(Length::Fill)
        .into()
}

fn sidebar_view_inner<'a>(state: SidebarContext, now: Instant) -> Element<'a, GlobalMessage> {
    column![
        row![
            Button::new(svg_icon(Icon::Menu, ICON_SIZE, IconStyle::Auto))
                .on_press(GlobalMessage::Command(WindowCommand::ToggleSidebar))
                .padding(BUTTON_PADDING)
                .style(button::ghost),
            Space::new().width(Length::Fill),
            Button::new(svg_icon(Icon::Add, ICON_SIZE, IconStyle::Auto))
                .on_press(GlobalMessage::Command(WindowCommand::ToggleSidebar))
                .padding(BUTTON_PADDING)
                .style(button::ghost)
        ]
        .width(Length::Fill),
        Space::new().height(SIDEBAR_PADDING),
        sidebar_button(
            Icon::Directory,
            fl!("sidebar-home"),
            GlobalMessage::Command(WindowCommand::Navigate(Page::Home(ShowDownloads::All))),
            state.sidebar_state.clone(),
            state.current_page == Page::Home(ShowDownloads::All),
            now
        ),
        sidebar_button(
            Icon::Downloading,
            fl!("sidebar-downloading"),
            GlobalMessage::Command(WindowCommand::Navigate(Page::Home(
                ShowDownloads::Downloading
            ))),
            state.sidebar_state.clone(),
            state.current_page == Page::Home(ShowDownloads::Downloading),
            now
        ),
        sidebar_button(
            Icon::Pause,
            fl!("sidebar-paused"),
            GlobalMessage::Command(WindowCommand::Navigate(Page::Home(ShowDownloads::Paused))),
            state.sidebar_state.clone(),
            state.current_page == Page::Home(ShowDownloads::Paused),
            now
        ),
        Space::new().height(Length::Fill),
        sidebar_button(
            Icon::Settings,
            fl!("sidebar-settings"),
            GlobalMessage::Command(WindowCommand::Navigate(Page::Settings)),
            state.sidebar_state.clone(),
            state.current_page == Page::Settings,
            now
        ),
        sidebar_button(
            Icon::Info,
            fl!("sidebar-about"),
            GlobalMessage::Command(WindowCommand::Navigate(Page::About)),
            state.sidebar_state.clone(),
            false,
            now
        )
    ]
    .width(
        state
            .sidebar_state
            .interpolate(SIDEBAR_WIDTH, state.sidebar_width, now),
    )
    .spacing(SIDEBAR_PADDING)
    .padding(Padding::new(SIDEBAR_PADDING).bottom(SIDEBAR_PADDING * 2.))
    .into()
}

fn sidebar_button<'a>(
    icon: Icon,
    label: String,
    message: GlobalMessage,
    sidebar_state: Animation<bool>,
    current_page: bool,
    now: Instant,
) -> Button<'a, GlobalMessage> {
    let sidebar_state_clone = sidebar_state.clone();
    Button::new(
        row![
            svg_icon(icon, ICON_SIZE, IconStyle::Auto),
            text(label).style(move |theme: &Theme| {
                text::Style {
                    color: Some(
                        theme
                            .extended_palette()
                            .background
                            .base
                            .text
                            .scale_alpha(sidebar_state_clone.interpolate(0.0, 1.0, now)),
                    ),
                }
            })
        ]
        .spacing(sidebar_state.interpolate(0.0, 0.5 * FONT_SIZE, now))
        .align_y(Alignment::Center),
    )
    .on_press(message)
    .width(Length::Fill)
    .padding(BUTTON_PADDING)
    .height(ICON_SIZE + (2. * BUTTON_PADDING))
    .style(move |theme: &Theme, status: Status| {
        let palette = theme.extended_palette();
        let mut style = button::ghost(theme, status);
        if current_page {
            style.background = Some(palette.background.weak.color.scale_alpha(0.6).into());
        }
        match status {
            Status::Hovered => {
                if current_page {
                    style.background = Some(palette.background.weak.color.scale_alpha(0.8).into());
                } else {
                    style.background = Some(palette.background.weakest.color.into());
                }
                style
            }
            _ => style,
        }
    })
}
