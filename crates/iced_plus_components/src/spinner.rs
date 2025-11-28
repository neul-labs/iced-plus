//! Loading spinner components.
//!
//! Provides animated loading indicators for async operations.
//!
//! # Animation
//!
//! Spinners animate by updating `progress` (0.0 to 1.0). Use the provided
//! `spinner_subscription()` helper in your app's subscription:
//!
//! ```rust,ignore
//! use iced_plus_components::spinner::{spinner_subscription, SpinnerMessage};
//!
//! // In your Message enum:
//! SpinnerTick(SpinnerMessage),
//!
//! // In your subscription:
//! fn subscription(&self) -> Subscription<Message> {
//!     spinner_subscription().map(Message::SpinnerTick)
//! }
//!
//! // In your update:
//! Message::SpinnerTick(SpinnerMessage::Tick(progress)) => {
//!     self.spinner_progress = progress;
//! }
//!
//! // In your view:
//! CircularSpinner::new().progress(self.spinner_progress)
//! ```

use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{mouse, Element, Length, Point, Rectangle, Renderer, Size, Subscription, Theme};
use std::f32::consts::PI;
use std::time::{Duration, Instant};

/// Easing function type.
pub type EasingFn = fn(f32) -> f32;

/// Common easing functions for animations.
pub mod easing {
    /// Linear easing (no acceleration).
    pub fn linear(t: f32) -> f32 {
        t
    }

    /// Ease in (accelerate from zero).
    pub fn ease_in(t: f32) -> f32 {
        t * t
    }

    /// Ease out (decelerate to zero).
    pub fn ease_out(t: f32) -> f32 {
        1.0 - (1.0 - t) * (1.0 - t)
    }

    /// Ease in-out (accelerate then decelerate).
    pub fn ease_in_out(t: f32) -> f32 {
        if t < 0.5 {
            2.0 * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
        }
    }

    /// Cubic ease in.
    pub fn ease_in_cubic(t: f32) -> f32 {
        t * t * t
    }

    /// Cubic ease out.
    pub fn ease_out_cubic(t: f32) -> f32 {
        1.0 - (1.0 - t).powi(3)
    }

    /// Emphasized easing (Material Design).
    pub fn emphasized(t: f32) -> f32 {
        if t < 0.5 {
            4.0 * t * t * t
        } else {
            1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
    }
}

/// A circular loading spinner.
///
/// For animation, update `progress` periodically (0.0 to 1.0) using
/// iced's subscription system (e.g., `iced::time::every`).
///
/// # Example
///
/// ```rust,ignore
/// // In your view:
/// CircularSpinner::new()
///     .size(40.0)
///     .progress(self.spinner_progress)
///
/// // In your subscription, update progress every ~16ms
/// ```
pub struct CircularSpinner {
    size: f32,
    bar_height: f32,
    progress: f32,
    easing: EasingFn,
}

impl Default for CircularSpinner {
    fn default() -> Self {
        Self::new()
    }
}

impl CircularSpinner {
    /// Create a new circular spinner.
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: 40.0,
            bar_height: 4.0,
            progress: 0.0,
            easing: easing::ease_in_out,
        }
    }

    /// Set the spinner size (diameter).
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the bar/stroke thickness.
    #[must_use]
    pub fn bar_height(mut self, height: f32) -> Self {
        self.bar_height = height;
        self
    }

    /// Set the animation progress (0.0 to 1.0).
    /// Update this periodically for animation.
    #[must_use]
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress % 1.0;
        self
    }

    /// Set the easing function.
    #[must_use]
    pub fn easing(mut self, easing: EasingFn) -> Self {
        self.easing = easing;
        self
    }
}

struct CircularProgram {
    bar_height: f32,
    progress: f32,
    easing: EasingFn,
}

impl<Message> canvas::Program<Message, Theme> for CircularProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let radius = (bounds.width.min(bounds.height) / 2.0) - self.bar_height;

        let palette = theme.extended_palette();

        // Draw track
        let track = Path::circle(center, radius);
        frame.stroke(
            &track,
            Stroke::default()
                .with_width(self.bar_height)
                .with_color(palette.background.weak.color),
        );

        // Calculate arc based on progress
        let rotation = self.progress * 2.0 * PI * 2.0; // Two full rotations per cycle

        // Expand then contract
        let cycle_progress = (self.progress * 2.0) % 1.0;
        let is_expanding = (self.progress * 2.0) < 1.0;

        let min_angle = 0.1 * PI;
        let max_angle = 1.5 * PI;

        let (start_angle, sweep_angle) = if is_expanding {
            let sweep = min_angle + (max_angle - min_angle) * (self.easing)(cycle_progress);
            (rotation, sweep)
        } else {
            let sweep = max_angle - (max_angle - min_angle) * (self.easing)(cycle_progress);
            let start = rotation + (max_angle - min_angle) * (self.easing)(cycle_progress);
            (start, sweep)
        };

        // Draw animated arc
        let arc = Path::new(|builder| {
            builder.arc(canvas::path::Arc {
                center,
                radius,
                start_angle: iced::Radians(start_angle),
                end_angle: iced::Radians(start_angle + sweep_angle),
            });
        });

        frame.stroke(
            &arc,
            Stroke::default()
                .with_width(self.bar_height)
                .with_color(palette.primary.base.color),
        );

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<CircularSpinner> for Element<'a, Message, Theme> {
    fn from(spinner: CircularSpinner) -> Self {
        let size = spinner.size;
        let program = CircularProgram {
            bar_height: spinner.bar_height,
            progress: spinner.progress,
            easing: spinner.easing,
        };
        Canvas::new(program)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .into()
    }
}

/// A linear loading spinner (indeterminate progress bar).
///
/// # Example
///
/// ```rust,ignore
/// LinearSpinner::new()
///     .width(200.0)
///     .height(4.0)
///     .progress(self.spinner_progress)
/// ```
pub struct LinearSpinner {
    width: Length,
    height: f32,
    progress: f32,
    easing: EasingFn,
}

impl Default for LinearSpinner {
    fn default() -> Self {
        Self::new()
    }
}

impl LinearSpinner {
    /// Create a new linear spinner.
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: Length::Fill,
            height: 4.0,
            progress: 0.0,
            easing: easing::ease_in_out,
        }
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height (bar thickness).
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    /// Set the animation progress (0.0 to 1.0).
    #[must_use]
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress % 1.0;
        self
    }

    /// Set the easing function.
    #[must_use]
    pub fn easing(mut self, easing: EasingFn) -> Self {
        self.easing = easing;
        self
    }
}

struct LinearProgram {
    progress: f32,
    easing: EasingFn,
}

impl<Message> canvas::Program<Message, Theme> for LinearProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let palette = theme.extended_palette();

        // Draw track
        let track = Path::rectangle(Point::ORIGIN, bounds.size());
        frame.fill(&track, palette.background.weak.color);

        // Calculate bar position based on progress
        // Bar moves from left to right, then right to left
        let cycle_progress = (self.progress * 2.0) % 1.0;
        let is_first_half = (self.progress * 2.0) < 1.0;

        let eased = (self.easing)(cycle_progress);

        let bar_width = bounds.width * 0.3; // Bar is 30% of total width

        let bar_x = if is_first_half {
            eased * (bounds.width - bar_width)
        } else {
            (1.0 - eased) * (bounds.width - bar_width)
        };

        // Draw animated bar
        let bar = Path::rectangle(
            Point::new(bar_x, 0.0),
            Size::new(bar_width, bounds.height),
        );
        frame.fill(&bar, palette.primary.base.color);

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<LinearSpinner> for Element<'a, Message, Theme> {
    fn from(spinner: LinearSpinner) -> Self {
        let program = LinearProgram {
            progress: spinner.progress,
            easing: spinner.easing,
        };
        Canvas::new(program)
            .width(spinner.width)
            .height(Length::Fixed(spinner.height))
            .into()
    }
}

/// A dots loading spinner (bouncing dots).
///
/// # Example
///
/// ```rust,ignore
/// DotsSpinner::new()
///     .dot_count(3)
///     .dot_size(8.0)
///     .progress(self.spinner_progress)
/// ```
pub struct DotsSpinner {
    dot_count: usize,
    dot_size: f32,
    spacing: f32,
    progress: f32,
}

impl Default for DotsSpinner {
    fn default() -> Self {
        Self::new()
    }
}

impl DotsSpinner {
    /// Create a new dots spinner.
    #[must_use]
    pub fn new() -> Self {
        Self {
            dot_count: 3,
            dot_size: 8.0,
            spacing: 8.0,
            progress: 0.0,
        }
    }

    /// Set the number of dots.
    #[must_use]
    pub fn dot_count(mut self, count: usize) -> Self {
        self.dot_count = count.max(1);
        self
    }

    /// Set the dot size.
    #[must_use]
    pub fn dot_size(mut self, size: f32) -> Self {
        self.dot_size = size;
        self
    }

    /// Set the spacing between dots.
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Set the animation progress (0.0 to 1.0).
    #[must_use]
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress % 1.0;
        self
    }
}

struct DotsProgram {
    dot_count: usize,
    dot_size: f32,
    spacing: f32,
    progress: f32,
}

impl<Message> canvas::Program<Message, Theme> for DotsProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let palette = theme.extended_palette();

        let total_width =
            self.dot_count as f32 * self.dot_size + (self.dot_count - 1) as f32 * self.spacing;
        let start_x = (bounds.width - total_width) / 2.0;
        let center_y = bounds.height / 2.0;

        for i in 0..self.dot_count {
            let x = start_x + i as f32 * (self.dot_size + self.spacing) + self.dot_size / 2.0;

            // Calculate wave offset for each dot
            let dot_offset = i as f32 / self.dot_count as f32;
            let dot_progress = (self.progress + dot_offset) % 1.0;

            // Bounce animation using sine wave
            let bounce = (dot_progress * PI).sin();
            let y_offset = bounce * self.dot_size * 0.5;

            let center = Point::new(x, center_y - y_offset);

            // Opacity based on bounce
            let opacity = 0.4 + 0.6 * bounce;
            let color = iced::Color {
                a: opacity,
                ..palette.primary.base.color
            };

            let dot = Path::circle(center, self.dot_size / 2.0);
            frame.fill(&dot, color);
        }

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<DotsSpinner> for Element<'a, Message, Theme> {
    fn from(spinner: DotsSpinner) -> Self {
        let width = spinner.dot_count as f32 * spinner.dot_size
            + (spinner.dot_count - 1) as f32 * spinner.spacing;
        let height = spinner.dot_size * 2.0; // Extra height for bounce
        let program = DotsProgram {
            dot_count: spinner.dot_count,
            dot_size: spinner.dot_size,
            spacing: spinner.spacing,
            progress: spinner.progress,
        };
        Canvas::new(program)
            .width(Length::Fixed(width))
            .height(Length::Fixed(height))
            .into()
    }
}

/// A pulse loading indicator (simple pulsing circle).
///
/// # Example
///
/// ```rust,ignore
/// PulseSpinner::new()
///     .size(24.0)
///     .progress(self.spinner_progress)
/// ```
pub struct PulseSpinner {
    size: f32,
    progress: f32,
}

impl Default for PulseSpinner {
    fn default() -> Self {
        Self::new()
    }
}

impl PulseSpinner {
    /// Create a new pulse spinner.
    #[must_use]
    pub fn new() -> Self {
        Self {
            size: 24.0,
            progress: 0.0,
        }
    }

    /// Set the size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the animation progress (0.0 to 1.0).
    #[must_use]
    pub fn progress(mut self, progress: f32) -> Self {
        self.progress = progress % 1.0;
        self
    }
}

struct PulseProgram {
    progress: f32,
}

impl<Message> canvas::Program<Message, Theme> for PulseProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());
        let palette = theme.extended_palette();
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let max_radius = bounds.width.min(bounds.height) / 2.0;

        // Pulse animation
        let pulse = (self.progress * PI * 2.0).sin() * 0.5 + 0.5;
        let radius = max_radius * (0.6 + 0.4 * pulse);
        let opacity = 0.3 + 0.7 * pulse;

        let color = iced::Color {
            a: opacity,
            ..palette.primary.base.color
        };

        let circle = Path::circle(center, radius);
        frame.fill(&circle, color);

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<PulseSpinner> for Element<'a, Message, Theme> {
    fn from(spinner: PulseSpinner) -> Self {
        let size = spinner.size;
        let program = PulseProgram {
            progress: spinner.progress,
        };
        Canvas::new(program)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .into()
    }
}

// ============================================================================
// Animation Subscription Helpers
// ============================================================================

/// Message type for spinner animation ticks.
#[derive(Debug, Clone)]
pub enum SpinnerMessage {
    /// Animation tick with progress value (0.0 to 1.0).
    Tick(f32),
}

/// Default animation duration for one complete cycle.
pub const DEFAULT_CYCLE_DURATION: Duration = Duration::from_millis(1500);

/// Default frame rate for spinner animations (60fps).
pub const DEFAULT_FRAME_DURATION: Duration = Duration::from_millis(16);

/// Creates a subscription that emits spinner animation ticks.
///
/// The subscription emits `SpinnerMessage::Tick(progress)` approximately 60 times
/// per second, where `progress` cycles from 0.0 to 1.0 over the cycle duration.
///
/// # Example
///
/// ```rust,ignore
/// use iced_plus_components::spinner::{spinner_subscription, SpinnerMessage};
///
/// enum Message {
///     SpinnerTick(SpinnerMessage),
///     // ... other variants
/// }
///
/// fn subscription(&self) -> Subscription<Message> {
///     spinner_subscription().map(Message::SpinnerTick)
/// }
///
/// fn update(&mut self, message: Message) -> Command<Message> {
///     match message {
///         Message::SpinnerTick(SpinnerMessage::Tick(progress)) => {
///             self.spinner_progress = progress;
///         }
///         // ...
///     }
///     Command::none()
/// }
/// ```
pub fn spinner_subscription() -> Subscription<SpinnerMessage> {
    spinner_subscription_with_duration(DEFAULT_CYCLE_DURATION)
}

/// Creates a subscription with a custom cycle duration.
///
/// # Arguments
///
/// * `cycle_duration` - How long one complete animation cycle takes.
pub fn spinner_subscription_with_duration(cycle_duration: Duration) -> Subscription<SpinnerMessage> {
    iced::time::every(DEFAULT_FRAME_DURATION).map(move |_| {
        // Calculate progress based on current time
        let elapsed = Instant::now().elapsed().as_secs_f32();
        let cycle_secs = cycle_duration.as_secs_f32();
        let progress = (elapsed % cycle_secs) / cycle_secs;
        SpinnerMessage::Tick(progress)
    })
}

/// Helper to calculate spinner progress based on time.
///
/// This is useful if you prefer to manage time yourself rather than
/// using the subscription helper.
///
/// # Example
///
/// ```rust,ignore
/// // Track start time in state
/// let progress = calculate_progress(self.animation_start, Duration::from_secs(2));
/// CircularSpinner::new().progress(progress)
/// ```
pub fn calculate_progress(start: Instant, cycle_duration: Duration) -> f32 {
    let elapsed = start.elapsed().as_secs_f32();
    let cycle_secs = cycle_duration.as_secs_f32();
    (elapsed % cycle_secs) / cycle_secs
}
