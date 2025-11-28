//! Desktop platform integration for iced-plus.
//!
//! This crate provides cross-platform desktop APIs for:
//!
//! - **Tray icons** - System tray/notification area integration
//! - **Notifications** - Desktop notifications
//! - **Hotkeys** - Global keyboard shortcuts
//! - **Window** - Extended window management
//! - **Audio** - Audio playback APIs
//! - **Recording** - Audio and video recording APIs
//! - **WebView** - Embedded web browser integration
//!
//! # Platform Support
//!
//! Features are conditionally compiled based on target platform capabilities.
//! The abstractions provide trait-based APIs that can be implemented with
//! platform-specific backends.
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_platform::tray::{TrayIcon, TrayMenu};
//!
//! let tray = TrayIcon::new("My App")
//!     .icon(icon_data)
//!     .menu(TrayMenu::new()
//!         .item("Show", Message::Show)
//!         .separator()
//!         .item("Quit", Message::Quit));
//! ```

#![warn(missing_docs)]

#[cfg(feature = "tray")]
pub mod tray;

#[cfg(feature = "notifications")]
pub mod notifications;

#[cfg(feature = "hotkeys")]
pub mod hotkeys;

#[cfg(feature = "window")]
pub mod window;

#[cfg(feature = "audio")]
pub mod audio;

#[cfg(feature = "recording")]
pub mod recording;

#[cfg(feature = "webview")]
pub mod webview;
