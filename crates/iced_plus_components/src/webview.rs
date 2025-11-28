//! WebView component placeholder.
//!
//! Note: iced doesn't have built-in WebView support.
//! This module provides types and state management for webview integration.
//! Actual webview rendering requires platform-specific implementations
//! (e.g., wry, webkit2gtk, webview2).

use std::collections::HashMap;

/// WebView navigation state.
#[derive(Debug, Clone, Default)]
pub struct WebViewState {
    /// Current URL.
    pub url: String,
    /// Page title.
    pub title: Option<String>,
    /// Whether the page is loading.
    pub loading: bool,
    /// Whether we can go back.
    pub can_go_back: bool,
    /// Whether we can go forward.
    pub can_go_forward: bool,
    /// Current zoom level (1.0 = 100%).
    pub zoom: f32,
    /// Last error message.
    pub error: Option<String>,
}

impl WebViewState {
    /// Create a new webview state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            url: String::new(),
            title: None,
            loading: false,
            can_go_back: false,
            can_go_forward: false,
            zoom: 1.0,
            error: None,
        }
    }

    /// Create with an initial URL.
    #[must_use]
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            ..Self::new()
        }
    }

    /// Update URL (called when navigation occurs).
    pub fn set_url(&mut self, url: impl Into<String>) {
        self.url = url.into();
        self.error = None;
    }

    /// Update title.
    pub fn set_title(&mut self, title: impl Into<String>) {
        self.title = Some(title.into());
    }

    /// Set loading state.
    pub fn set_loading(&mut self, loading: bool) {
        self.loading = loading;
    }

    /// Update navigation state.
    pub fn set_navigation(&mut self, can_back: bool, can_forward: bool) {
        self.can_go_back = can_back;
        self.can_go_forward = can_forward;
    }

    /// Set zoom level.
    pub fn set_zoom(&mut self, zoom: f32) {
        self.zoom = zoom.clamp(0.25, 5.0);
    }

    /// Zoom in.
    pub fn zoom_in(&mut self) {
        self.set_zoom(self.zoom + 0.1);
    }

    /// Zoom out.
    pub fn zoom_out(&mut self) {
        self.set_zoom(self.zoom - 0.1);
    }

    /// Reset zoom.
    pub fn reset_zoom(&mut self) {
        self.zoom = 1.0;
    }

    /// Set error.
    pub fn set_error(&mut self, error: impl Into<String>) {
        self.error = Some(error.into());
        self.loading = false;
    }
}

/// WebView command to be executed.
#[derive(Debug, Clone)]
pub enum WebViewCommand {
    /// Navigate to URL.
    Navigate(String),
    /// Go back in history.
    GoBack,
    /// Go forward in history.
    GoForward,
    /// Reload the page.
    Reload,
    /// Stop loading.
    Stop,
    /// Execute JavaScript.
    ExecuteScript(String),
    /// Set zoom level.
    SetZoom(f32),
    /// Load HTML content directly.
    LoadHtml(String),
}

/// WebView configuration.
#[derive(Debug, Clone)]
pub struct WebViewConfig {
    /// Initial URL to load.
    pub initial_url: Option<String>,
    /// User agent string.
    pub user_agent: Option<String>,
    /// Whether to enable JavaScript.
    pub javascript_enabled: bool,
    /// Whether to enable web storage.
    pub storage_enabled: bool,
    /// Custom headers to send with requests.
    pub headers: HashMap<String, String>,
    /// Background color (CSS color string).
    pub background_color: Option<String>,
    /// Whether devtools are enabled.
    pub devtools: bool,
    /// Whether to allow file access.
    pub file_access: bool,
}

impl Default for WebViewConfig {
    fn default() -> Self {
        Self {
            initial_url: None,
            user_agent: None,
            javascript_enabled: true,
            storage_enabled: true,
            headers: HashMap::new(),
            background_color: None,
            devtools: false,
            file_access: false,
        }
    }
}

impl WebViewConfig {
    /// Create a new config.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set initial URL.
    #[must_use]
    pub fn url(mut self, url: impl Into<String>) -> Self {
        self.initial_url = Some(url.into());
        self
    }

    /// Set user agent.
    #[must_use]
    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    /// Disable JavaScript.
    #[must_use]
    pub fn disable_javascript(mut self) -> Self {
        self.javascript_enabled = false;
        self
    }

    /// Disable web storage.
    #[must_use]
    pub fn disable_storage(mut self) -> Self {
        self.storage_enabled = false;
        self
    }

    /// Add a custom header.
    #[must_use]
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }

    /// Set background color.
    #[must_use]
    pub fn background(mut self, color: impl Into<String>) -> Self {
        self.background_color = Some(color.into());
        self
    }

    /// Enable devtools.
    #[must_use]
    pub fn with_devtools(mut self) -> Self {
        self.devtools = true;
        self
    }

    /// Enable file access.
    #[must_use]
    pub fn with_file_access(mut self) -> Self {
        self.file_access = true;
        self
    }
}

/// Browser navigation bar widget.
///
/// # Example
///
/// ```rust,ignore
/// BrowserBar::new(&self.webview_state)
///     .on_navigate(Message::Navigate)
///     .on_back(Message::GoBack)
///     .on_forward(Message::GoForward)
///     .on_reload(Message::Reload)
/// ```
pub struct BrowserBar<'a, Message> {
    state: &'a WebViewState,
    on_navigate: Option<Box<dyn Fn(String) -> Message + 'a>>,
    on_back: Option<Message>,
    on_forward: Option<Message>,
    on_reload: Option<Message>,
    on_stop: Option<Message>,
}

impl<'a, Message> BrowserBar<'a, Message>
where
    Message: Clone,
{
    /// Create a new browser bar.
    pub fn new(state: &'a WebViewState) -> Self {
        Self {
            state,
            on_navigate: None,
            on_back: None,
            on_forward: None,
            on_reload: None,
            on_stop: None,
        }
    }

    /// Set the navigate callback.
    #[must_use]
    pub fn on_navigate<F>(mut self, f: F) -> Self
    where
        F: Fn(String) -> Message + 'a,
    {
        self.on_navigate = Some(Box::new(f));
        self
    }

    /// Set the back message.
    #[must_use]
    pub fn on_back(mut self, message: Message) -> Self {
        self.on_back = Some(message);
        self
    }

    /// Set the forward message.
    #[must_use]
    pub fn on_forward(mut self, message: Message) -> Self {
        self.on_forward = Some(message);
        self
    }

    /// Set the reload message.
    #[must_use]
    pub fn on_reload(mut self, message: Message) -> Self {
        self.on_reload = Some(message);
        self
    }

    /// Set the stop message.
    #[must_use]
    pub fn on_stop(mut self, message: Message) -> Self {
        self.on_stop = Some(message);
        self
    }
}

impl<'a, Message> From<BrowserBar<'a, Message>>
    for iced::Element<'a, Message, iced::Theme>
where
    Message: Clone + 'a,
{
    fn from(bar: BrowserBar<'a, Message>) -> Self {
        use iced::widget::{button, row, text, text_input};
        use iced::Length;

        let back_btn: iced::Element<'a, Message, iced::Theme> = {
            let mut btn = button(text("←").size(16));
            if bar.state.can_go_back {
                if let Some(msg) = bar.on_back {
                    btn = btn.on_press(msg);
                }
            }
            btn.into()
        };

        let forward_btn: iced::Element<'a, Message, iced::Theme> = {
            let mut btn = button(text("→").size(16));
            if bar.state.can_go_forward {
                if let Some(msg) = bar.on_forward {
                    btn = btn.on_press(msg);
                }
            }
            btn.into()
        };

        let reload_stop_btn: iced::Element<'a, Message, iced::Theme> = {
            if bar.state.loading {
                let mut btn = button(text("✕").size(16));
                if let Some(msg) = bar.on_stop {
                    btn = btn.on_press(msg);
                }
                btn.into()
            } else {
                let mut btn = button(text("↻").size(16));
                if let Some(msg) = bar.on_reload {
                    btn = btn.on_press(msg);
                }
                btn.into()
            }
        };

        let url_input: iced::Element<'a, Message, iced::Theme> = {
            let input = text_input("Enter URL...", &bar.state.url)
                .width(Length::Fill);

            if let Some(on_navigate) = bar.on_navigate {
                input.on_submit(on_navigate(bar.state.url.clone())).into()
            } else {
                input.into()
            }
        };

        row![back_btn, forward_btn, reload_stop_btn, url_input]
            .spacing(4)
            .padding(4)
            .align_y(iced::Alignment::Center)
            .into()
    }
}
