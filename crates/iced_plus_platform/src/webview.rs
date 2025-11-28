//! Cross-platform WebView integration.
//!
//! This module provides a platform-agnostic WebView API. For actual webview
//! functionality, you'll need to integrate with libraries like `wry` or
//! platform-specific webview APIs.
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_platform::webview::{WebView, WebViewCommand};
//!
//! // Create webview
//! let webview = WebView::new();
//!
//! // Navigate to a URL
//! webview.send(WebViewCommand::Navigate("https://example.com".into()));
//! ```

use std::collections::HashMap;

/// WebView commands.
#[derive(Debug, Clone)]
pub enum WebViewCommand {
    /// Navigate to a URL.
    Navigate(String),
    /// Navigate back in history.
    GoBack,
    /// Navigate forward in history.
    GoForward,
    /// Reload the current page.
    Reload,
    /// Stop loading.
    Stop,
    /// Load HTML content directly.
    LoadHtml {
        /// HTML content.
        html: String,
        /// Base URL for relative paths.
        base_url: Option<String>,
    },
    /// Execute JavaScript code.
    ExecuteScript(String),
    /// Set zoom level (1.0 = 100%).
    SetZoom(f64),
    /// Enable or disable JavaScript.
    SetJavaScriptEnabled(bool),
    /// Clear browsing data.
    ClearData(ClearDataOptions),
    /// Set a custom user agent.
    SetUserAgent(String),
    /// Inject CSS.
    InjectCss(String),
    /// Send a message to JavaScript (for JS <-> Rust communication).
    PostMessage(String),
    /// Find text on page.
    Find {
        /// Text to find.
        text: String,
        /// Search forward.
        forward: bool,
        /// Case sensitive.
        case_sensitive: bool,
    },
    /// Clear find highlights.
    ClearFind,
    /// Print the current page.
    Print,
    /// Take a screenshot.
    Screenshot,
}

/// Options for clearing browsing data.
#[derive(Debug, Clone, Default)]
pub struct ClearDataOptions {
    /// Clear cache.
    pub cache: bool,
    /// Clear cookies.
    pub cookies: bool,
    /// Clear local storage.
    pub local_storage: bool,
    /// Clear session storage.
    pub session_storage: bool,
    /// Clear IndexedDB.
    pub indexed_db: bool,
}

impl ClearDataOptions {
    /// Create options to clear everything.
    pub fn all() -> Self {
        Self {
            cache: true,
            cookies: true,
            local_storage: true,
            session_storage: true,
            indexed_db: true,
        }
    }

    /// Create options to clear just cache.
    pub fn cache_only() -> Self {
        Self {
            cache: true,
            ..Default::default()
        }
    }
}

/// WebView state.
#[derive(Debug, Clone, Default)]
pub struct WebViewState {
    /// Current URL.
    pub url: Option<String>,
    /// Page title.
    pub title: Option<String>,
    /// Whether the page is loading.
    pub loading: bool,
    /// Loading progress (0.0 to 1.0).
    pub progress: f32,
    /// Whether can go back.
    pub can_go_back: bool,
    /// Whether can go forward.
    pub can_go_forward: bool,
    /// Current zoom level.
    pub zoom: f64,
    /// Whether JavaScript is enabled.
    pub javascript_enabled: bool,
    /// Whether the webview is focused.
    pub focused: bool,
    /// Error message if any.
    pub error: Option<String>,
    /// Security info for the current page.
    pub security: SecurityInfo,
}

impl WebViewState {
    /// Create a new default webview state.
    pub fn new() -> Self {
        Self {
            zoom: 1.0,
            javascript_enabled: true,
            security: SecurityInfo::default(),
            ..Default::default()
        }
    }

    /// Check if the current URL is secure (HTTPS).
    pub fn is_secure(&self) -> bool {
        self.url
            .as_ref()
            .map(|u| u.starts_with("https://"))
            .unwrap_or(false)
    }
}

/// Security information for the current page.
#[derive(Debug, Clone, Default)]
pub struct SecurityInfo {
    /// Whether the connection is secure.
    pub secure: bool,
    /// Certificate information (if available).
    pub certificate: Option<CertificateInfo>,
}

/// SSL certificate information.
#[derive(Debug, Clone)]
pub struct CertificateInfo {
    /// Certificate issuer.
    pub issuer: String,
    /// Certificate subject.
    pub subject: String,
    /// Valid from date.
    pub valid_from: String,
    /// Valid until date.
    pub valid_until: String,
}

/// Events from a webview.
#[derive(Debug, Clone)]
pub enum WebViewEvent {
    /// Navigation started.
    NavigationStarted(String),
    /// Navigation completed.
    NavigationCompleted(String),
    /// Navigation failed.
    NavigationFailed { url: String, error: String },
    /// Page title changed.
    TitleChanged(String),
    /// Loading progress changed.
    ProgressChanged(f32),
    /// JavaScript message received (from page).
    MessageReceived(String),
    /// Script execution result.
    ScriptResult { id: String, result: String },
    /// New window requested.
    NewWindowRequested(String),
    /// Download requested.
    DownloadRequested {
        url: String,
        suggested_filename: String,
    },
    /// Permission requested.
    PermissionRequested(Permission),
    /// Find result.
    FindResult {
        current_match: u32,
        total_matches: u32,
    },
    /// Screenshot captured.
    ScreenshotCaptured(Vec<u8>),
    /// WebView created and ready.
    Ready,
    /// Focus changed.
    FocusChanged(bool),
    /// Error occurred.
    Error(String),
}

/// Permission types that a webpage might request.
#[derive(Debug, Clone)]
pub enum Permission {
    /// Geolocation access.
    Geolocation,
    /// Microphone access.
    Microphone,
    /// Camera access.
    Camera,
    /// Notification permission.
    Notifications,
    /// Clipboard access.
    Clipboard,
    /// Other/unknown permission.
    Other(String),
}

/// Configuration for creating a webview.
#[derive(Debug, Clone)]
pub struct WebViewConfig {
    /// Initial URL to load.
    pub url: Option<String>,
    /// Initial HTML to load.
    pub html: Option<String>,
    /// Enable JavaScript.
    pub javascript_enabled: bool,
    /// Enable developer tools.
    pub devtools_enabled: bool,
    /// Custom user agent.
    pub user_agent: Option<String>,
    /// Background color.
    pub background_color: Option<(u8, u8, u8, u8)>,
    /// Whether to allow file access from file URLs.
    pub allow_file_access: bool,
    /// Custom HTTP headers to send with requests.
    pub custom_headers: HashMap<String, String>,
    /// Whether to enable hardware acceleration.
    pub hardware_acceleration: bool,
    /// Whether to enable autoplay.
    pub autoplay_enabled: bool,
}

impl Default for WebViewConfig {
    fn default() -> Self {
        Self {
            url: None,
            html: None,
            javascript_enabled: true,
            devtools_enabled: false,
            user_agent: None,
            background_color: None,
            allow_file_access: false,
            custom_headers: HashMap::new(),
            hardware_acceleration: true,
            autoplay_enabled: false,
        }
    }
}

impl WebViewConfig {
    /// Create a new config with a URL.
    pub fn with_url(url: impl Into<String>) -> Self {
        Self {
            url: Some(url.into()),
            ..Default::default()
        }
    }

    /// Create a new config with HTML content.
    pub fn with_html(html: impl Into<String>) -> Self {
        Self {
            html: Some(html.into()),
            ..Default::default()
        }
    }

    /// Enable developer tools.
    pub fn devtools(mut self, enabled: bool) -> Self {
        self.devtools_enabled = enabled;
        self
    }

    /// Set custom user agent.
    pub fn user_agent(mut self, agent: impl Into<String>) -> Self {
        self.user_agent = Some(agent.into());
        self
    }

    /// Add a custom header.
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.custom_headers.insert(name.into(), value.into());
        self
    }
}

/// Trait for webview backends.
pub trait WebViewBackend: Send {
    /// Send a command to the webview.
    fn send(&self, command: WebViewCommand) -> Result<(), WebViewError>;

    /// Get the current state.
    fn state(&self) -> WebViewState;

    /// Resize the webview.
    fn resize(&self, width: u32, height: u32);

    /// Handle a permission request.
    fn handle_permission(&self, permission: Permission, allow: bool);
}

/// WebView errors.
#[derive(Debug, Clone)]
pub enum WebViewError {
    /// Invalid URL.
    InvalidUrl(String),
    /// Navigation failed.
    NavigationFailed(String),
    /// Script execution failed.
    ScriptError(String),
    /// Permission denied.
    PermissionDenied,
    /// Backend not available.
    BackendUnavailable,
    /// Not supported on this platform.
    NotSupported,
    /// Generic error.
    Other(String),
}

impl std::fmt::Display for WebViewError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            Self::NavigationFailed(msg) => write!(f, "Navigation failed: {}", msg),
            Self::ScriptError(msg) => write!(f, "Script error: {}", msg),
            Self::PermissionDenied => write!(f, "Permission denied"),
            Self::BackendUnavailable => write!(f, "WebView backend unavailable"),
            Self::NotSupported => write!(f, "WebView not supported on this platform"),
            Self::Other(msg) => write!(f, "WebView error: {}", msg),
        }
    }
}

impl std::error::Error for WebViewError {}

/// No-op webview for when no backend is available.
#[derive(Debug, Default)]
pub struct NoOpWebView {
    state: WebViewState,
}

impl NoOpWebView {
    /// Create a new no-op webview.
    pub fn new() -> Self {
        Self {
            state: WebViewState::new(),
        }
    }
}

impl WebViewBackend for NoOpWebView {
    fn send(&self, _command: WebViewCommand) -> Result<(), WebViewError> {
        Ok(())
    }

    fn state(&self) -> WebViewState {
        self.state.clone()
    }

    fn resize(&self, _width: u32, _height: u32) {}

    fn handle_permission(&self, _permission: Permission, _allow: bool) {}
}

/// Helper for common URL patterns.
pub mod urls {
    /// Check if a URL is a valid HTTP(S) URL.
    pub fn is_http(url: &str) -> bool {
        url.starts_with("http://") || url.starts_with("https://")
    }

    /// Check if a URL is a file URL.
    pub fn is_file(url: &str) -> bool {
        url.starts_with("file://")
    }

    /// Check if a URL is a data URL.
    pub fn is_data(url: &str) -> bool {
        url.starts_with("data:")
    }

    /// Check if a URL is a blob URL.
    pub fn is_blob(url: &str) -> bool {
        url.starts_with("blob:")
    }

    /// Get the domain from a URL.
    pub fn domain(url: &str) -> Option<&str> {
        let url = url.strip_prefix("https://").or_else(|| url.strip_prefix("http://"))?;
        url.split('/').next()
    }
}
