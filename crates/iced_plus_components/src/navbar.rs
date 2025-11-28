//! Navigation bar and app bar components.

use crate::icons::{Icon, IconName};
use iced::widget::{button, column, container, row, text, Space};
use iced::{Background, Border, Color, Element, Length, Theme};

/// Navigation item for the navbar.
pub struct NavItem<'a, Message> {
    /// Item label.
    pub label: String,
    /// Optional icon element.
    pub icon_element: Option<Element<'a, Message, Theme>>,
    /// Message to emit when clicked.
    pub on_click: Option<Message>,
    /// Whether this item is active.
    pub active: bool,
}

impl<'a, Message: Clone + 'a> NavItem<'a, Message> {
    /// Create a new navigation item.
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            icon_element: None,
            on_click: None,
            active: false,
        }
    }

    /// Set an icon element for the item.
    #[must_use]
    pub fn icon_element(mut self, icon: impl Into<Element<'a, Message, Theme>>) -> Self {
        self.icon_element = Some(icon.into());
        self
    }

    /// Set an icon by name (convenience method).
    #[must_use]
    pub fn icon(self, name: IconName) -> Self {
        self.icon_element(Icon::new(name).size(16.0))
    }

    /// Set the click handler.
    #[must_use]
    pub fn on_click(mut self, message: Message) -> Self {
        self.on_click = Some(message);
        self
    }

    /// Mark this item as active.
    #[must_use]
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }
}

/// Application navigation bar with hamburger menu support.
///
/// # Example
///
/// ```rust,ignore
/// AppBar::new()
///     .title("My App")
///     .on_menu_toggle(Message::ToggleMenu)
///     .push(NavItem::new("Home").icon(IconName::Home).on_click(Message::GoHome))
///     .push(NavItem::new("Settings").icon(IconName::Settings).on_click(Message::GoSettings))
/// ```
pub struct AppBar<'a, Message> {
    title: Option<String>,
    items: Vec<NavItem<'a, Message>>,
    on_menu_toggle: Option<Message>,
    show_menu_button: bool,
    end_content: Option<Element<'a, Message, Theme>>,
    height: f32,
}

impl<'a, Message: Clone + 'a> AppBar<'a, Message> {
    /// Create a new app bar.
    pub fn new() -> Self {
        Self {
            title: None,
            items: Vec::new(),
            on_menu_toggle: None,
            show_menu_button: true,
            end_content: None,
            height: 56.0,
        }
    }

    /// Set the app title.
    #[must_use]
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Add a navigation item.
    #[must_use]
    pub fn push(mut self, item: NavItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Set the menu toggle handler (hamburger button).
    #[must_use]
    pub fn on_menu_toggle(mut self, message: Message) -> Self {
        self.on_menu_toggle = Some(message);
        self
    }

    /// Hide the hamburger menu button.
    #[must_use]
    pub fn hide_menu_button(mut self) -> Self {
        self.show_menu_button = false;
        self
    }

    /// Add content to the end of the bar (e.g., user avatar, theme toggle).
    #[must_use]
    pub fn end_content(mut self, content: impl Into<Element<'a, Message, Theme>>) -> Self {
        self.end_content = Some(content.into());
        self
    }

    /// Set the bar height.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

impl<'a, Message: Clone + 'a> Default for AppBar<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: Clone + 'a> From<AppBar<'a, Message>> for Element<'a, Message, Theme> {
    fn from(bar: AppBar<'a, Message>) -> Self {
        let mut items: Vec<Element<'a, Message, Theme>> = Vec::new();

        // Hamburger menu button with vector icon
        if bar.show_menu_button {
            let menu_icon: Element<'a, Message, Theme> = Icon::menu().size(20.0).into();
            let menu_btn = if let Some(on_toggle) = bar.on_menu_toggle {
                button(menu_icon)
                    .padding([8, 12])
                    .on_press(on_toggle)
                    .style(|theme: &Theme, status| {
                        let palette = theme.extended_palette();
                        match status {
                            button::Status::Active | button::Status::Pressed => button::Style {
                                background: Some(Background::Color(Color::TRANSPARENT)),
                                text_color: palette.background.base.text,
                                ..Default::default()
                            },
                            button::Status::Hovered => button::Style {
                                background: Some(Background::Color(
                                    palette.background.weak.color,
                                )),
                                text_color: palette.background.base.text,
                                border: Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            button::Status::Disabled => button::Style {
                                background: Some(Background::Color(Color::TRANSPARENT)),
                                text_color: palette.background.weak.text,
                                ..Default::default()
                            },
                        }
                    })
            } else {
                button(menu_icon)
                    .padding([8, 12])
                    .style(|_theme: &Theme, _status| button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        text_color: Color::from_rgb(0.5, 0.5, 0.5),
                        ..Default::default()
                    })
            };
            items.push(menu_btn.into());
        }

        // Title
        if let Some(title) = bar.title {
            items.push(
                text(title)
                    .size(18)
                    .into(),
            );
            items.push(Space::with_width(16).into());
        }

        // Navigation items
        for item in bar.items {
            let label = item.label.clone();
            let btn_content: Element<'a, Message, Theme> = if let Some(icon) = item.icon_element {
                row![icon, Space::with_width(6), text(label).size(14)]
                    .align_y(iced::Alignment::Center)
                    .into()
            } else {
                text(label).size(14).into()
            };

            let is_active = item.active;
            let nav_btn = if let Some(on_click) = item.on_click {
                button(btn_content)
                    .padding([8, 16])
                    .on_press(on_click)
                    .style(move |theme: &Theme, status| {
                        let palette = theme.extended_palette();
                        match status {
                            button::Status::Active | button::Status::Pressed => {
                                if is_active {
                                    button::Style {
                                        background: Some(Background::Color(
                                            palette.primary.weak.color,
                                        )),
                                        text_color: palette.primary.strong.color,
                                        border: Border {
                                            radius: 4.0.into(),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }
                                } else {
                                    button::Style {
                                        background: Some(Background::Color(Color::TRANSPARENT)),
                                        text_color: palette.background.base.text,
                                        ..Default::default()
                                    }
                                }
                            }
                            button::Status::Hovered => button::Style {
                                background: Some(Background::Color(
                                    if is_active {
                                        palette.primary.weak.color
                                    } else {
                                        palette.background.weak.color
                                    },
                                )),
                                text_color: if is_active {
                                    palette.primary.strong.color
                                } else {
                                    palette.background.base.text
                                },
                                border: Border {
                                    radius: 4.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            button::Status::Disabled => button::Style {
                                background: Some(Background::Color(Color::TRANSPARENT)),
                                text_color: palette.background.weak.text,
                                ..Default::default()
                            },
                        }
                    })
            } else {
                button(btn_content)
                    .padding([8, 16])
                    .style(|_theme: &Theme, _status| button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        text_color: Color::from_rgb(0.5, 0.5, 0.5),
                        ..Default::default()
                    })
            };
            items.push(nav_btn.into());
        }

        // Spacer
        items.push(Space::with_width(Length::Fill).into());

        // End content
        if let Some(end) = bar.end_content {
            items.push(end);
        }

        let content = row(items)
            .spacing(4)
            .align_y(iced::Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fixed(bar.height))
            .padding([0, 16])
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(palette.background.base.color)),
                    border: Border {
                        color: palette.background.weak.color,
                        width: 0.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            })
            .into()
    }
}

/// Side navigation menu (for drawer).
///
/// # Example
///
/// ```rust,ignore
/// SideNav::new()
///     .push(NavItem::new("Home").icon(IconName::Home).on_click(Message::GoHome).active(true))
///     .push(NavItem::new("Settings").icon(IconName::Settings).on_click(Message::GoSettings))
/// ```
pub struct SideNav<'a, Message> {
    items: Vec<NavItem<'a, Message>>,
    header: Option<String>,
    width: Length,
}

impl<'a, Message: Clone + 'a> SideNav<'a, Message> {
    /// Create a new side navigation.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            header: None,
            width: Length::Fixed(260.0),
        }
    }

    /// Add a header text.
    #[must_use]
    pub fn header(mut self, header: impl Into<String>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Add a navigation item.
    #[must_use]
    pub fn push(mut self, item: NavItem<'a, Message>) -> Self {
        self.items.push(item);
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }
}

impl<'a, Message: Clone + 'a> Default for SideNav<'a, Message> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message: Clone + 'a> From<SideNav<'a, Message>> for Element<'a, Message, Theme> {
    fn from(nav: SideNav<'a, Message>) -> Self {
        let mut items: Vec<Element<'a, Message, Theme>> = Vec::new();

        // Header
        if let Some(header) = nav.header {
            items.push(
                container(text(header).size(12))
                    .padding(16)
                    .into(),
            );
        }

        // Navigation items
        for item in nav.items {
            let label = item.label.clone();
            let btn_content: Element<'a, Message, Theme> = if let Some(icon) = item.icon_element {
                row![icon, Space::with_width(10), text(label).size(14)]
                    .align_y(iced::Alignment::Center)
                    .into()
            } else {
                text(label).size(14).into()
            };

            let is_active = item.active;
            let nav_btn = if let Some(on_click) = item.on_click {
                button(btn_content)
                    .padding([12, 16])
                    .width(Length::Fill)
                    .on_press(on_click)
                    .style(move |theme: &Theme, status| {
                        let palette = theme.extended_palette();
                        match status {
                            button::Status::Active | button::Status::Pressed => {
                                if is_active {
                                    button::Style {
                                        background: Some(Background::Color(
                                            palette.primary.weak.color,
                                        )),
                                        text_color: palette.primary.strong.color,
                                        border: Border {
                                            radius: 8.0.into(),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }
                                } else {
                                    button::Style {
                                        background: Some(Background::Color(Color::TRANSPARENT)),
                                        text_color: palette.background.base.text,
                                        border: Border {
                                            radius: 8.0.into(),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    }
                                }
                            }
                            button::Status::Hovered => button::Style {
                                background: Some(Background::Color(
                                    palette.background.weak.color,
                                )),
                                text_color: palette.background.base.text,
                                border: Border {
                                    radius: 8.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                            button::Status::Disabled => button::Style {
                                background: Some(Background::Color(Color::TRANSPARENT)),
                                text_color: palette.background.weak.text,
                                border: Border {
                                    radius: 8.0.into(),
                                    ..Default::default()
                                },
                                ..Default::default()
                            },
                        }
                    })
            } else {
                button(btn_content)
                    .padding([12, 16])
                    .width(Length::Fill)
                    .style(|_theme: &Theme, _status| button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        text_color: Color::from_rgb(0.5, 0.5, 0.5),
                        border: Border {
                            radius: 8.0.into(),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
            };

            items.push(
                container(nav_btn)
                    .padding([2, 8])
                    .width(Length::Fill)
                    .into(),
            );
        }

        container(column(items).spacing(2))
            .width(nav.width)
            .height(Length::Fill)
            .style(|theme: &Theme| {
                let palette = theme.extended_palette();
                container::Style {
                    background: Some(Background::Color(palette.background.base.color)),
                    border: Border {
                        color: palette.background.weak.color,
                        width: 1.0,
                        radius: 0.0.into(),
                    },
                    ..Default::default()
                }
            })
            .into()
    }
}
