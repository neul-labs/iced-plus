//! An opinionated companion toolkit for building desktop apps with iced.
//!
//! `iced_plus` provides batteries for building ambitious desktop applications:
//!
//! - **Design Tokens**: Tailwind/Chakra-inspired design tokens with preset themes
//! - **Theme Bridge**: Adapters that wire tokens into iced's styling system
//! - **Layouts**: Type-safe layout primitives (stacks, shells, responsive)
//! - **Components**: 30+ pre-built widgets with ergonomic builder APIs
//! - **Platform**: Cross-platform desktop APIs (tray, notifications, hotkeys)
//!
//! # Quick Start
//!
//! ```rust,ignore
//! use iced_plus::prelude::*;
//!
//! // Use design tokens for consistent styling
//! let theme = AppTheme::light();
//!
//! // Build layouts with stacks
//! let content = VStack::new()
//!     .spacing(16.0)
//!     .push(Button::primary("Click me"))
//!     .push(TextInput::default("Enter text..."));
//! ```
//!
//! # Feature Flags
//!
//! - `tokens` (default): Design tokens and color scales
//! - `theme` (default): Theme bridge adapting tokens to iced
//! - `layouts` (default): Layout primitives (stacks, shells, responsive)
//! - `components` (default): Pre-built UI components
//! - `platform`: Desktop platform APIs (opt-in)
//! - `full`: All features including platform APIs
//!
//! # Crate Organization
//!
//! You can also depend on individual crates for fine-grained control:
//!
//! - [`iced_plus_tokens`](https://docs.rs/iced_plus_tokens) - Design tokens
//! - [`iced_plus_theme`](https://docs.rs/iced_plus_theme) - Theme bridge
//! - [`iced_plus_layouts`](https://docs.rs/iced_plus_layouts) - Layout primitives
//! - [`iced_plus_components`](https://docs.rs/iced_plus_components) - UI components
//! - [`iced_plus_platform`](https://docs.rs/iced_plus_platform) - Platform APIs

#![warn(missing_docs)]
#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

// Re-export sub-crates
#[cfg(feature = "tokens")]
pub use iced_plus_tokens as tokens;

#[cfg(feature = "theme")]
pub use iced_plus_theme as theme;

#[cfg(feature = "layouts")]
pub use iced_plus_layouts as layouts;

#[cfg(feature = "components")]
pub use iced_plus_components as components;

#[cfg(feature = "platform")]
pub use iced_plus_platform as platform;

/// Commonly used types and traits for convenient imports.
///
/// # Usage
///
/// ```rust,ignore
/// use iced_plus::prelude::*;
/// ```
pub mod prelude {
    // Tokens
    #[cfg(feature = "tokens")]
    pub use iced_plus_tokens::{
        Color, ColorPalette, ColorScale, Elevation, ElevationLevel, Motion, RadiusScale,
        RadiusSize, SemanticColors, Shade, SpacingScale, SpacingSize, TextStyle, ThemePreset,
        ThemeTokens, TypographyScale,
    };

    // Theme
    #[cfg(feature = "theme")]
    pub use iced_plus_theme::{
        AppTheme, ButtonVariant, ComponentSize, Destructive, ExtraLarge, ExtraSmall, Ghost, Large,
        Medium, Outline, Primary, Secondary, Small, Status,
    };

    // Layouts
    #[cfg(feature = "layouts")]
    pub use iced_plus_layouts::{Direction, HStack, Horizontal, VStack, Vertical};

    #[cfg(feature = "layouts")]
    pub use iced_plus_layouts::AppShell;

    #[cfg(feature = "layouts")]
    pub use iced_plus_layouts::{modal, Modal};

    #[cfg(feature = "layouts")]
    pub use iced_plus_layouts::{BreakpointTier, Breakpoints, Responsive, ShowOn};

    // Components
    #[cfg(feature = "components")]
    pub use iced_plus_components::{
        Alert, AlertType, Avatar, Badge, Button, Card, Checkbox, Divider, Drawer, Heading, Icon,
        IconName, Image, Menu, MenuBar, MenuItem, Progress, Radio, RadioGroup, Select, Skeleton,
        Slider, Switch, Tab, Tabs, Text, TextInput, Toast, ToastManager, Tooltip,
    };

    // Spinners
    #[cfg(feature = "components")]
    pub use iced_plus_components::{CircularSpinner, DotsSpinner, LinearSpinner, PulseSpinner};

    // Media components
    #[cfg(feature = "components")]
    pub use iced_plus_components::{
        AudioControls, MediaPlayerState, PlaybackState, VideoControls,
    };
}
