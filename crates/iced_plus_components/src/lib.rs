//! Component library for iced-plus.
//!
//! This crate provides pre-styled, type-safe widgets with ergonomic builder APIs.
//!
//! # Design Principles
//!
//! - **Type-safety**: Variants and sizes are encoded in the type signature using phantom types
//! - **Zero-cost abstractions**: All style resolution happens at compile time
//! - **Ergonomic API**: Builder pattern with method chaining
//!
//! # Components
//!
//! ## Buttons & Inputs
//! - [`Button`] - Type-safe buttons with variant and size encoded in types
//! - [`TextInput`] - Enhanced text input with label and helper text
//! - [`Checkbox`] - Styled checkbox component
//! - [`Radio`], [`RadioGroup`] - Radio buttons for single selection
//! - [`Switch`] - Toggle switches
//! - [`Slider`], [`VerticalSlider`] - Range sliders
//! - [`Select`] - Dropdown selection
//!
//! ## Layout & Display
//! - [`Text`], [`Heading`] - Typography components
//! - [`Card`] - Elevated content container
//! - [`Divider`] - Visual separators
//! - [`Avatar`] - User/entity avatars
//! - [`Skeleton`] - Loading placeholders
//! - [`Image`] - Image display with loading states
//!
//! ## Feedback
//! - [`Badge`] - Status indicators and counts
//! - [`Alert`] - Contextual feedback messages
//! - [`Progress`] - Progress indicators
//! - [`Toast`] - Toast notifications
//! - [`Tooltip`] - Hover tooltips
//!
//! ## Navigation & Overlays
//! - [`Tabs`] - Tab navigation
//! - [`Menu`], [`MenuBar`] - Menus and menu bars
//! - [`Drawer`] - Side panel overlays
//!
//! ## Media
//! - [`AudioControls`], [`VideoControls`] - Media player controls
//! - [`MediaPlayerState`] - Playback state management
//! - [`WebViewState`], [`BrowserBar`] - WebView integration helpers
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_components::button::Button;
//!
//! // Type: Button<'_, Primary, Medium, Message>
//! let btn = Button::primary("Save")
//!     .on_press(Message::Save);
//!
//! // Type: Button<'_, Destructive, Small, Message>
//! let delete = Button::destructive("Delete")
//!     .small()
//!     .on_press(Message::Delete);
//! ```

#![warn(missing_docs)]

mod private;

// Core components
pub mod alert;
pub mod avatar;
pub mod badge;
pub mod button;
pub mod card;
pub mod checkbox;
pub mod color_picker;
pub mod divider;
pub mod drawer;
pub mod icons;
pub mod image;
pub mod input;
pub mod media;
pub mod menu;
pub mod navbar;
pub mod progress;
pub mod radio;
pub mod rich_text;
pub mod scrollable;
pub mod select;
pub mod skeleton;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod tabs;
pub mod text;
pub mod textarea;
pub mod toast;
pub mod tooltip;
pub mod webview;

// Re-exports for convenience
pub use alert::{Alert, AlertType};
pub use avatar::{Avatar, AvatarShape, AvatarSize};
pub use badge::{Badge, BadgeVariant};
pub use button::{Button, ButtonSize, ButtonVariant};
pub use button::{Destructive, Ghost, Outline, Primary, Secondary};
pub use button::{ExtraSmall, Large, Medium, Small};
pub use card::{Card, Elevation};
pub use checkbox::Checkbox;
pub use color_picker::{
    color_palette, color_picker_view, color_to_hex, hex_to_color, presets as color_presets,
    ColorSwatch, Hsl, HueSlider, SatLightPicker,
};
pub use divider::Divider;
pub use drawer::{Drawer, DrawerPosition};
pub use icons::{icon, icon_colored, Icon, IconName};
pub use image::{Image, ImagePlaceholder, ImageSource};
pub use input::TextInput;
pub use media::{
    AudioControls, AudioRecorder, MediaPlayerState, PlaybackState, RecorderState, RecordingState,
    VideoControls, VideoRecorder,
};
pub use menu::{Menu, MenuBar, MenuItem};
pub use navbar::{AppBar, NavItem, SideNav};
pub use progress::{Progress, ProgressVariant};
pub use radio::{Radio, RadioGroup};
pub use rich_text::{formatting, FormattingState, RichTextAction, RichTextContent, RichTextEditor};
pub use scrollable::{
    minimal_scrollable, position as scroll_position, styled_scrollable, themed_scrollable,
    AnchorSection, ScrollDirection, ScrollableBuilder, ScrollableConfig, SnapAlignment,
};
pub use select::Select;
pub use skeleton::{Skeleton, SkeletonShape};
pub use slider::{Slider, VerticalSlider};
pub use spinner::{
    calculate_progress, easing, spinner_subscription, spinner_subscription_with_duration,
    CircularSpinner, DotsSpinner, LinearSpinner, PulseSpinner, SpinnerMessage,
    DEFAULT_CYCLE_DURATION, DEFAULT_FRAME_DURATION,
};
pub use switch::Switch;
pub use tabs::{Tab, TabWidth, Tabs};
pub use text::{Heading, HeadingLevel, Text, TextStyle};
pub use textarea::{SimpleTextArea, TextArea, TextAreaContent};
pub use toast::{
    toast_container, toast_view, toasts, Toast, ToastManager, ToastPosition, ToastVariant,
};
pub use tooltip::{Tooltip, TooltipPosition};
pub use webview::{BrowserBar, WebViewCommand, WebViewConfig, WebViewState};
