//! Rich text editor component.
//!
//! Provides a text editor with formatting toolbar.
//!
//! # Example
//!
//! ```rust,ignore
//! RichTextEditor::new(&self.content)
//!     .on_action(Message::EditorAction)
//!     .height(300.0)
//! ```

use crate::icons::{Icon, IconName};
use iced::widget::{button, column, container, row, text_editor, Space, TextEditor};
use iced::{Background, Border, Color, Element, Length, Padding, Theme};
use std::rc::Rc;

/// Rich text editor content (wraps text_editor::Content).
pub type RichTextContent = text_editor::Content;

/// Rich text editor action.
#[derive(Debug, Clone)]
pub enum RichTextAction {
    /// Text editor action from iced.
    Edit(text_editor::Action),
    /// Toggle bold formatting.
    ToggleBold,
    /// Toggle italic formatting.
    ToggleItalic,
    /// Toggle underline formatting.
    ToggleUnderline,
    /// Toggle strikethrough formatting.
    ToggleStrikethrough,
    /// Toggle code/monospace formatting.
    ToggleCode,
    /// Insert heading.
    InsertHeading(u8),
    /// Insert bullet list item.
    InsertBullet,
    /// Insert numbered list item.
    InsertNumber,
    /// Insert link.
    InsertLink,
    /// Insert horizontal rule.
    InsertRule,
    /// Undo last action.
    Undo,
    /// Redo last undone action.
    Redo,
}

/// Formatting state for the toolbar.
#[derive(Debug, Clone, Default)]
pub struct FormattingState {
    /// Bold is active.
    pub bold: bool,
    /// Italic is active.
    pub italic: bool,
    /// Underline is active.
    pub underline: bool,
    /// Strikethrough is active.
    pub strikethrough: bool,
    /// Code/monospace is active.
    pub code: bool,
}

/// A rich text editor with formatting toolbar.
///
/// This provides a text editor with common formatting options.
/// Formatting is applied using markdown-like syntax.
///
/// # Example
///
/// ```rust,ignore
/// // In state:
/// content: text_editor::Content::new(),
///
/// // In view:
/// RichTextEditor::new(&self.content)
///     .on_action(Message::RichTextAction)
///     .placeholder("Write something...")
///     .height(400.0)
/// ```
pub struct RichTextEditor<'a, Message> {
    content: &'a RichTextContent,
    on_action: Option<Rc<dyn Fn(RichTextAction) -> Message + 'a>>,
    placeholder: Option<&'a str>,
    width: Length,
    height: Length,
    show_toolbar: bool,
    formatting: FormattingState,
}

impl<'a, Message: Clone + 'a> RichTextEditor<'a, Message> {
    /// Create a new rich text editor.
    pub fn new(content: &'a RichTextContent) -> Self {
        Self {
            content,
            on_action: None,
            placeholder: None,
            width: Length::Fill,
            height: Length::Fixed(300.0),
            show_toolbar: true,
            formatting: FormattingState::default(),
        }
    }

    /// Set the action handler.
    #[must_use]
    pub fn on_action<F>(mut self, on_action: F) -> Self
    where
        F: Fn(RichTextAction) -> Message + 'a,
    {
        self.on_action = Some(Rc::new(on_action));
        self
    }

    /// Set placeholder text.
    #[must_use]
    pub fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height (includes toolbar).
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Show or hide the formatting toolbar.
    #[must_use]
    pub fn toolbar(mut self, show: bool) -> Self {
        self.show_toolbar = show;
        self
    }

    /// Set the current formatting state (for toolbar button highlighting).
    #[must_use]
    pub fn formatting(mut self, state: FormattingState) -> Self {
        self.formatting = state;
        self
    }
}

impl<'a, Message: Clone + 'a> From<RichTextEditor<'a, Message>> for Element<'a, Message, Theme> {
    fn from(editor: RichTextEditor<'a, Message>) -> Self {
        let on_action = editor.on_action;

        // Create the text editor
        let mut text_ed = TextEditor::new(editor.content);

        if let Some(ref on_action) = on_action {
            let on_action = Rc::clone(on_action);
            text_ed = text_ed.on_action(move |action| on_action(RichTextAction::Edit(action)));
        }

        if let Some(placeholder) = editor.placeholder {
            text_ed = text_ed.placeholder(placeholder);
        }

        text_ed = text_ed
            .padding(Padding::new(12.0))
            .style(|theme: &Theme, status| {
                let palette = theme.extended_palette();

                let border_color = match status {
                    text_editor::Status::Active => palette.background.strong.color,
                    text_editor::Status::Hovered => palette.primary.weak.color,
                    text_editor::Status::Focused => palette.primary.base.color,
                    text_editor::Status::Disabled => palette.background.strong.color,
                };

                text_editor::Style {
                    background: Background::Color(palette.background.base.color),
                    border: Border {
                        color: border_color,
                        width: 1.0,
                        radius: 6.0.into(),
                    },
                    icon: palette.background.weak.text,
                    placeholder: palette.background.weak.text,
                    value: palette.background.base.text,
                    selection: palette.primary.weak.color,
                }
            });

        // Create toolbar if enabled
        let toolbar: Option<Element<'a, Message, Theme>> = if editor.show_toolbar {
            if let Some(ref on_action) = on_action {
                Some(create_toolbar(on_action, &editor.formatting))
            } else {
                None
            }
        } else {
            None
        };

        // Combine toolbar and editor
        let content: Element<'a, Message, Theme> = if let Some(toolbar) = toolbar {
            column![toolbar, text_ed].spacing(0).into()
        } else {
            text_ed.into()
        };

        container(content)
            .width(editor.width)
            .height(editor.height)
            .into()
    }
}

fn create_toolbar<'a, Message: Clone + 'a>(
    on_action: &Rc<dyn Fn(RichTextAction) -> Message + 'a>,
    formatting: &FormattingState,
) -> Element<'a, Message, Theme> {
    let on_action = Rc::clone(on_action);
    let btn = move |icon_name: IconName, action: RichTextAction, active: bool| {
        let icon: Element<'a, Message, Theme> = Icon::new(icon_name).size(16.0).into();
        let on_action = Rc::clone(&on_action);

        button(icon)
            .padding([6, 8])
            .on_press(on_action(action))
            .style(move |theme: &Theme, status| {
                let palette = theme.extended_palette();
                let bg = if active {
                    palette.primary.weak.color
                } else {
                    match status {
                        button::Status::Hovered => palette.background.weak.color,
                        _ => Color::TRANSPARENT,
                    }
                };
                button::Style {
                    background: Some(Background::Color(bg)),
                    text_color: palette.background.base.text,
                    border: Border {
                        radius: 4.0.into(),
                        ..Default::default()
                    },
                    ..Default::default()
                }
            })
    };

    let separator = || {
        container(Space::new(1, 20)).style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.weak.color)),
                ..Default::default()
            }
        })
    };

    let toolbar_content = row![
        // Text formatting
        btn(IconName::Edit, RichTextAction::ToggleBold, formatting.bold),
        // Using Edit icon for italic since we don't have a specific italic icon
        btn(
            IconName::Edit,
            RichTextAction::ToggleItalic,
            formatting.italic
        ),
        separator(),
        // Lists
        btn(IconName::List, RichTextAction::InsertBullet, false),
        separator(),
        // Links and extras
        btn(IconName::Link, RichTextAction::InsertLink, false),
        btn(IconName::Minus, RichTextAction::InsertRule, false),
        Space::with_width(Length::Fill),
        // Undo/Redo - using refresh for now
        btn(IconName::ArrowLeft, RichTextAction::Undo, false),
        btn(IconName::ArrowRight, RichTextAction::Redo, false),
    ]
    .spacing(2)
    .padding([8, 12])
    .align_y(iced::Alignment::Center);

    container(toolbar_content)
        .width(Length::Fill)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.weak.color)),
                border: Border {
                    color: palette.background.strong.color,
                    width: 1.0,
                    radius: 6.0.into(),
                },
                ..Default::default()
            }
        })
        .into()
}

/// Helper to apply markdown-style formatting to selected text.
pub mod formatting {
    /// Wrap text with bold markers.
    pub fn bold(text: &str) -> String {
        format!("**{}**", text)
    }

    /// Wrap text with italic markers.
    pub fn italic(text: &str) -> String {
        format!("*{}*", text)
    }

    /// Wrap text with strikethrough markers.
    pub fn strikethrough(text: &str) -> String {
        format!("~~{}~~", text)
    }

    /// Wrap text with code markers.
    pub fn code(text: &str) -> String {
        format!("`{}`", text)
    }

    /// Create a heading.
    pub fn heading(level: u8, text: &str) -> String {
        let hashes = "#".repeat(level.min(6) as usize);
        format!("{} {}", hashes, text)
    }

    /// Create a bullet list item.
    pub fn bullet(text: &str) -> String {
        format!("- {}", text)
    }

    /// Create a numbered list item.
    pub fn numbered(num: usize, text: &str) -> String {
        format!("{}. {}", num, text)
    }

    /// Create a link.
    pub fn link(text: &str, url: &str) -> String {
        format!("[{}]({})", text, url)
    }

    /// Create a horizontal rule.
    pub fn horizontal_rule() -> &'static str {
        "---"
    }
}
