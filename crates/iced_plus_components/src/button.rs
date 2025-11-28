//! Type-safe button with phantom type encoding for variants and sizes.
//!
//! The button variant and size are encoded in the type signature, enabling
//! compile-time style resolution and monomorphization for optimal performance.

use std::borrow::Cow;
use std::marker::PhantomData;

use iced::widget::button;
use iced::{Element, Length};
use iced_plus_theme::{AppTheme, ButtonClass};

use crate::private::Sealed;

// ============================================================================
// Variant marker types
// ============================================================================

/// Primary button variant - main call-to-action.
#[derive(Debug, Clone, Copy, Default)]
pub struct Primary;
impl Sealed for Primary {}

/// Secondary button variant - alternative actions.
#[derive(Debug, Clone, Copy, Default)]
pub struct Secondary;
impl Sealed for Secondary {}

/// Ghost button variant - minimal styling.
#[derive(Debug, Clone, Copy, Default)]
pub struct Ghost;
impl Sealed for Ghost {}

/// Destructive button variant - dangerous/irreversible actions.
#[derive(Debug, Clone, Copy, Default)]
pub struct Destructive;
impl Sealed for Destructive {}

/// Outline button variant - bordered with transparent background.
#[derive(Debug, Clone, Copy, Default)]
pub struct Outline;
impl Sealed for Outline {}

/// Trait for button variants.
pub trait ButtonVariant: Sealed + Copy + Default {
    /// Get the corresponding ButtonClass for iced styling.
    fn button_class() -> ButtonClass;
}

impl ButtonVariant for Primary {
    fn button_class() -> ButtonClass {
        ButtonClass::Primary
    }
}

impl ButtonVariant for Secondary {
    fn button_class() -> ButtonClass {
        ButtonClass::Secondary
    }
}

impl ButtonVariant for Ghost {
    fn button_class() -> ButtonClass {
        ButtonClass::Ghost
    }
}

impl ButtonVariant for Destructive {
    fn button_class() -> ButtonClass {
        ButtonClass::Destructive
    }
}

impl ButtonVariant for Outline {
    fn button_class() -> ButtonClass {
        ButtonClass::Outline
    }
}

// ============================================================================
// Size marker types
// ============================================================================

/// Extra small button size.
#[derive(Debug, Clone, Copy, Default)]
pub struct ExtraSmall;
impl Sealed for ExtraSmall {}

/// Small button size.
#[derive(Debug, Clone, Copy, Default)]
pub struct Small;
impl Sealed for Small {}

/// Medium button size (default).
#[derive(Debug, Clone, Copy, Default)]
pub struct Medium;
impl Sealed for Medium {}

/// Large button size.
#[derive(Debug, Clone, Copy, Default)]
pub struct Large;
impl Sealed for Large {}

/// Trait for button sizes.
pub trait ButtonSize: Sealed + Copy + Default {
    /// Horizontal padding in pixels.
    const PADDING_H: f32;
    /// Vertical padding in pixels.
    const PADDING_V: f32;
    /// Font size in pixels.
    const FONT_SIZE: f32;
}

impl ButtonSize for ExtraSmall {
    const PADDING_H: f32 = 8.0;
    const PADDING_V: f32 = 4.0;
    const FONT_SIZE: f32 = 12.0;
}

impl ButtonSize for Small {
    const PADDING_H: f32 = 12.0;
    const PADDING_V: f32 = 6.0;
    const FONT_SIZE: f32 = 14.0;
}

impl ButtonSize for Medium {
    const PADDING_H: f32 = 16.0;
    const PADDING_V: f32 = 8.0;
    const FONT_SIZE: f32 = 14.0;
}

impl ButtonSize for Large {
    const PADDING_H: f32 = 20.0;
    const PADDING_V: f32 = 10.0;
    const FONT_SIZE: f32 = 16.0;
}

// ============================================================================
// Button widget
// ============================================================================

/// A type-safe button with variant and size encoded in the type signature.
///
/// # Type Parameters
///
/// - `V`: The variant type (Primary, Secondary, Ghost, Destructive, Outline)
/// - `S`: The size type (ExtraSmall, Small, Medium, Large)
/// - `Message`: The message type for button press events
///
/// # Example
///
/// ```rust,ignore
/// use iced_plus_components::button::Button;
///
/// // Type: Button<'_, Primary, Medium, Message>
/// let btn = Button::primary("Click me")
///     .on_press(Message::Clicked);
///
/// // Type: Button<'_, Destructive, Small, Message>
/// let delete = Button::destructive("Delete")
///     .small()
///     .on_press(Message::Delete);
/// ```
pub struct Button<'a, V, S, Message>
where
    V: ButtonVariant,
    S: ButtonSize,
{
    label: Cow<'a, str>,
    on_press: Option<Message>,
    width: Length,
    _variant: PhantomData<V>,
    _size: PhantomData<S>,
}

// Constructors for each variant (returns Medium size by default)
impl<'a, Message> Button<'a, Primary, Medium, Message> {
    /// Create a primary button.
    #[must_use]
    pub fn primary(label: impl Into<Cow<'a, str>>) -> Self {
        Self::new(label)
    }
}

impl<'a, Message> Button<'a, Secondary, Medium, Message> {
    /// Create a secondary button.
    #[must_use]
    pub fn secondary(label: impl Into<Cow<'a, str>>) -> Self {
        Self::new(label)
    }
}

impl<'a, Message> Button<'a, Ghost, Medium, Message> {
    /// Create a ghost button.
    #[must_use]
    pub fn ghost(label: impl Into<Cow<'a, str>>) -> Self {
        Self::new(label)
    }
}

impl<'a, Message> Button<'a, Destructive, Medium, Message> {
    /// Create a destructive button.
    #[must_use]
    pub fn destructive(label: impl Into<Cow<'a, str>>) -> Self {
        Self::new(label)
    }
}

impl<'a, Message> Button<'a, Outline, Medium, Message> {
    /// Create an outline button.
    #[must_use]
    pub fn outline(label: impl Into<Cow<'a, str>>) -> Self {
        Self::new(label)
    }
}

impl<'a, V, S, Message> Button<'a, V, S, Message>
where
    V: ButtonVariant,
    S: ButtonSize,
{
    fn new(label: impl Into<Cow<'a, str>>) -> Self {
        Self {
            label: label.into(),
            on_press: None,
            width: Length::Shrink,
            _variant: PhantomData,
            _size: PhantomData,
        }
    }

    /// Set the message to emit when pressed.
    #[must_use]
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);
        self
    }

    /// Set the message to emit when pressed, if Some.
    #[must_use]
    pub fn on_press_maybe(mut self, message: Option<Message>) -> Self {
        self.on_press = message;
        self
    }

    /// Set the button width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Make the button fill the available width.
    #[must_use]
    pub fn fill_width(mut self) -> Self {
        self.width = Length::Fill;
        self
    }

    // Size changers - these change the type parameter S

    /// Change to extra small size.
    #[must_use]
    pub fn extra_small(self) -> Button<'a, V, ExtraSmall, Message> {
        Button {
            label: self.label,
            on_press: self.on_press,
            width: self.width,
            _variant: PhantomData,
            _size: PhantomData,
        }
    }

    /// Change to small size.
    #[must_use]
    pub fn small(self) -> Button<'a, V, Small, Message> {
        Button {
            label: self.label,
            on_press: self.on_press,
            width: self.width,
            _variant: PhantomData,
            _size: PhantomData,
        }
    }

    /// Change to large size.
    #[must_use]
    pub fn large(self) -> Button<'a, V, Large, Message> {
        Button {
            label: self.label,
            on_press: self.on_press,
            width: self.width,
            _variant: PhantomData,
            _size: PhantomData,
        }
    }
}

impl<'a, V, S, Message> From<Button<'a, V, S, Message>> for Element<'a, Message, AppTheme<'a>>
where
    V: ButtonVariant + 'a,
    S: ButtonSize + 'a,
    Message: Clone + 'a,
{
    fn from(btn: Button<'a, V, S, Message>) -> Self {
        let label: String = btn.label.into_owned();
        let content = iced::widget::text(label).size(S::FONT_SIZE);

        let mut button = button(content)
            .padding([S::PADDING_V, S::PADDING_H])
            .width(btn.width)
            .class(V::button_class());

        if let Some(msg) = btn.on_press {
            button = button.on_press(msg);
        }

        button.into()
    }
}

// Also implement for default iced::Theme for flexibility
impl<'a, V, S, Message> From<Button<'a, V, S, Message>> for Element<'a, Message, iced::Theme>
where
    V: ButtonVariant + 'a,
    S: ButtonSize + 'a,
    Message: Clone + 'a,
{
    fn from(btn: Button<'a, V, S, Message>) -> Self {
        let label: String = btn.label.into_owned();
        let content = iced::widget::text(label).size(S::FONT_SIZE);

        let mut button = button(content)
            .padding([S::PADDING_V, S::PADDING_H])
            .width(btn.width);

        if let Some(msg) = btn.on_press {
            button = button.on_press(msg);
        }

        button.into()
    }
}
