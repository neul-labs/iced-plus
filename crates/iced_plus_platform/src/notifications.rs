//! Desktop notifications.
//!
//! Provides a cross-platform abstraction for desktop notifications.

use std::borrow::Cow;
use std::time::Duration;

/// Notification urgency level.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum NotificationUrgency {
    /// Low priority notification.
    Low,
    /// Normal priority notification.
    #[default]
    Normal,
    /// Critical/high priority notification.
    Critical,
}

/// Desktop notification builder.
///
/// # Example
///
/// ```rust,ignore
/// let notification = Notification::new("Download Complete")
///     .body("Your file has been downloaded successfully.")
///     .urgency(NotificationUrgency::Normal)
///     .timeout(Duration::from_secs(5));
/// ```
pub struct Notification<'a> {
    /// Notification title.
    pub title: Cow<'a, str>,
    /// Notification body text.
    pub body: Option<Cow<'a, str>>,
    /// Application name (for grouping).
    pub app_name: Option<Cow<'a, str>>,
    /// Icon name or path.
    pub icon: Option<Cow<'a, str>>,
    /// Urgency level.
    pub urgency: NotificationUrgency,
    /// Auto-dismiss timeout.
    pub timeout: Option<Duration>,
}

impl<'a> Notification<'a> {
    /// Create a new notification with a title.
    #[must_use]
    pub fn new(title: impl Into<Cow<'a, str>>) -> Self {
        Self {
            title: title.into(),
            body: None,
            app_name: None,
            icon: None,
            urgency: NotificationUrgency::default(),
            timeout: None,
        }
    }

    /// Set the notification body.
    #[must_use]
    pub fn body(mut self, body: impl Into<Cow<'a, str>>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Set the application name.
    #[must_use]
    pub fn app_name(mut self, name: impl Into<Cow<'a, str>>) -> Self {
        self.app_name = Some(name.into());
        self
    }

    /// Set the notification icon.
    #[must_use]
    pub fn icon(mut self, icon: impl Into<Cow<'a, str>>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    /// Set the urgency level.
    #[must_use]
    pub fn urgency(mut self, urgency: NotificationUrgency) -> Self {
        self.urgency = urgency;
        self
    }

    /// Set the auto-dismiss timeout.
    #[must_use]
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
}
