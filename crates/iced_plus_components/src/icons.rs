//! Vector icon components.
//!
//! Provides scalable vector icons that render consistently without font dependencies.
//!
//! # Example
//!
//! ```rust,ignore
//! use iced_plus_components::icons::{Icon, IconName};
//!
//! Icon::new(IconName::Home).size(24.0)
//! ```

use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path, Stroke};
use iced::{mouse, Color, Element, Length, Point, Rectangle, Renderer, Theme};

/// Available icon names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    // Navigation
    /// Home icon
    Home,
    /// Menu (hamburger) icon
    Menu,
    /// Close (X) icon
    Close,
    /// Back arrow
    ArrowLeft,
    /// Forward arrow
    ArrowRight,
    /// Up arrow
    ArrowUp,
    /// Down arrow
    ArrowDown,
    /// Chevron left
    ChevronLeft,
    /// Chevron right
    ChevronRight,
    /// Chevron up
    ChevronUp,
    /// Chevron down
    ChevronDown,

    // Actions
    /// Plus/Add icon
    Plus,
    /// Minus/Remove icon
    Minus,
    /// Check/Checkmark icon
    Check,
    /// Search/Magnifier icon
    Search,
    /// Settings/Gear icon
    Settings,
    /// Edit/Pencil icon
    Edit,
    /// Delete/Trash icon
    Trash,
    /// Refresh/Reload icon
    Refresh,
    /// Download icon
    Download,
    /// Upload icon
    Upload,
    /// Share icon
    Share,
    /// Copy icon
    Copy,

    // Status/Feedback
    /// Info circle
    Info,
    /// Warning triangle
    Warning,
    /// Error/X circle
    Error,
    /// Success/Check circle
    Success,

    // Media
    /// Play icon
    Play,
    /// Pause icon
    Pause,
    /// Stop icon
    Stop,
    /// Record icon
    Record,
    /// Volume/Speaker icon
    Volume,
    /// Mute icon
    Mute,
    /// Microphone icon
    Microphone,
    /// Camera/Video icon
    Camera,

    // Content
    /// File/Document icon
    File,
    /// Folder icon
    Folder,
    /// Image/Photo icon
    Image,
    /// Grid/Components icon
    Grid,
    /// List icon
    List,

    // Communication
    /// Mail/Email icon
    Mail,
    /// Chat/Message icon
    Chat,
    /// Bell/Notification icon
    Bell,

    // User
    /// User/Person icon
    User,
    /// Users/People icon
    Users,

    // Misc
    /// Sun (light mode) icon
    Sun,
    /// Moon (dark mode) icon
    Moon,
    /// Globe/Web icon
    Globe,
    /// Link icon
    Link,
    /// Star icon
    Star,
    /// Heart icon
    Heart,
    /// Eye/View icon
    Eye,
    /// Eye off/Hide icon
    EyeOff,
    /// Lock icon
    Lock,
    /// Unlock icon
    Unlock,
}

/// A vector icon component.
pub struct Icon {
    name: IconName,
    size: f32,
    color: Option<Color>,
}

impl Icon {
    /// Create a new icon.
    #[must_use]
    pub fn new(name: IconName) -> Self {
        Self {
            name,
            size: 24.0,
            color: None,
        }
    }

    /// Set the icon size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set the icon color.
    #[must_use]
    pub fn color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    // Convenience constructors for common icons

    /// Home icon
    #[must_use]
    pub fn home() -> Self {
        Self::new(IconName::Home)
    }

    /// Menu icon
    #[must_use]
    pub fn menu() -> Self {
        Self::new(IconName::Menu)
    }

    /// Close icon
    #[must_use]
    pub fn close() -> Self {
        Self::new(IconName::Close)
    }

    /// Settings icon
    #[must_use]
    pub fn settings() -> Self {
        Self::new(IconName::Settings)
    }

    /// Search icon
    #[must_use]
    pub fn search() -> Self {
        Self::new(IconName::Search)
    }

    /// Plus icon
    #[must_use]
    pub fn plus() -> Self {
        Self::new(IconName::Plus)
    }

    /// Check icon
    #[must_use]
    pub fn check() -> Self {
        Self::new(IconName::Check)
    }

    /// Info icon
    #[must_use]
    pub fn info() -> Self {
        Self::new(IconName::Info)
    }

    /// Warning icon
    #[must_use]
    pub fn warning() -> Self {
        Self::new(IconName::Warning)
    }

    /// Error icon
    #[must_use]
    pub fn error() -> Self {
        Self::new(IconName::Error)
    }

    /// Success icon
    #[must_use]
    pub fn success() -> Self {
        Self::new(IconName::Success)
    }

    /// Play icon
    #[must_use]
    pub fn play() -> Self {
        Self::new(IconName::Play)
    }

    /// Pause icon
    #[must_use]
    pub fn pause() -> Self {
        Self::new(IconName::Pause)
    }

    /// Sun icon
    #[must_use]
    pub fn sun() -> Self {
        Self::new(IconName::Sun)
    }

    /// Moon icon
    #[must_use]
    pub fn moon() -> Self {
        Self::new(IconName::Moon)
    }

    /// User icon
    #[must_use]
    pub fn user() -> Self {
        Self::new(IconName::User)
    }

    /// Globe icon
    #[must_use]
    pub fn globe() -> Self {
        Self::new(IconName::Globe)
    }

    /// Grid icon
    #[must_use]
    pub fn grid() -> Self {
        Self::new(IconName::Grid)
    }

    /// Edit icon
    #[must_use]
    pub fn edit() -> Self {
        Self::new(IconName::Edit)
    }

    /// Chat icon
    #[must_use]
    pub fn chat() -> Self {
        Self::new(IconName::Chat)
    }

    /// Camera icon
    #[must_use]
    pub fn camera() -> Self {
        Self::new(IconName::Camera)
    }
}

struct IconProgram {
    name: IconName,
    color: Option<Color>,
}

impl IconProgram {
    fn draw_icon(&self, frame: &mut Frame, bounds: Rectangle, color: Color) {
        let size = bounds.width.min(bounds.height);
        let center = Point::new(bounds.width / 2.0, bounds.height / 2.0);
        let stroke = Stroke::default().with_width(size * 0.08).with_color(color);
        let thin_stroke = Stroke::default().with_width(size * 0.06).with_color(color);

        match self.name {
            IconName::Home => {
                // House shape
                let roof = Path::new(|b| {
                    b.move_to(Point::new(center.x, size * 0.15));
                    b.line_to(Point::new(size * 0.85, size * 0.45));
                    b.line_to(Point::new(size * 0.15, size * 0.45));
                    b.close();
                });
                frame.fill(&roof, color);

                let house = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.45));
                    b.line_to(Point::new(size * 0.25, size * 0.85));
                    b.line_to(Point::new(size * 0.75, size * 0.85));
                    b.line_to(Point::new(size * 0.75, size * 0.45));
                });
                frame.stroke(&house, stroke);
            }

            IconName::Menu => {
                // Three horizontal lines
                for i in 0..3 {
                    let y = size * (0.3 + i as f32 * 0.2);
                    let line = Path::line(
                        Point::new(size * 0.2, y),
                        Point::new(size * 0.8, y),
                    );
                    frame.stroke(&line, stroke);
                }
            }

            IconName::Close => {
                // X shape
                let line1 = Path::line(
                    Point::new(size * 0.25, size * 0.25),
                    Point::new(size * 0.75, size * 0.75),
                );
                let line2 = Path::line(
                    Point::new(size * 0.75, size * 0.25),
                    Point::new(size * 0.25, size * 0.75),
                );
                frame.stroke(&line1, stroke);
                frame.stroke(&line2, stroke);
            }

            IconName::ArrowLeft => {
                let arrow = Path::new(|b| {
                    b.move_to(Point::new(size * 0.6, size * 0.25));
                    b.line_to(Point::new(size * 0.3, size * 0.5));
                    b.line_to(Point::new(size * 0.6, size * 0.75));
                });
                frame.stroke(&arrow, stroke);
                let line = Path::line(
                    Point::new(size * 0.3, size * 0.5),
                    Point::new(size * 0.8, size * 0.5),
                );
                frame.stroke(&line, stroke);
            }

            IconName::ArrowRight => {
                let arrow = Path::new(|b| {
                    b.move_to(Point::new(size * 0.4, size * 0.25));
                    b.line_to(Point::new(size * 0.7, size * 0.5));
                    b.line_to(Point::new(size * 0.4, size * 0.75));
                });
                frame.stroke(&arrow, stroke);
                let line = Path::line(
                    Point::new(size * 0.2, size * 0.5),
                    Point::new(size * 0.7, size * 0.5),
                );
                frame.stroke(&line, stroke);
            }

            IconName::ArrowUp => {
                let arrow = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.6));
                    b.line_to(Point::new(size * 0.5, size * 0.3));
                    b.line_to(Point::new(size * 0.75, size * 0.6));
                });
                frame.stroke(&arrow, stroke);
                let line = Path::line(
                    Point::new(size * 0.5, size * 0.3),
                    Point::new(size * 0.5, size * 0.8),
                );
                frame.stroke(&line, stroke);
            }

            IconName::ArrowDown => {
                let arrow = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.4));
                    b.line_to(Point::new(size * 0.5, size * 0.7));
                    b.line_to(Point::new(size * 0.75, size * 0.4));
                });
                frame.stroke(&arrow, stroke);
                let line = Path::line(
                    Point::new(size * 0.5, size * 0.2),
                    Point::new(size * 0.5, size * 0.7),
                );
                frame.stroke(&line, stroke);
            }

            IconName::ChevronLeft => {
                let chevron = Path::new(|b| {
                    b.move_to(Point::new(size * 0.6, size * 0.2));
                    b.line_to(Point::new(size * 0.35, size * 0.5));
                    b.line_to(Point::new(size * 0.6, size * 0.8));
                });
                frame.stroke(&chevron, stroke);
            }

            IconName::ChevronRight => {
                let chevron = Path::new(|b| {
                    b.move_to(Point::new(size * 0.4, size * 0.2));
                    b.line_to(Point::new(size * 0.65, size * 0.5));
                    b.line_to(Point::new(size * 0.4, size * 0.8));
                });
                frame.stroke(&chevron, stroke);
            }

            IconName::ChevronUp => {
                let chevron = Path::new(|b| {
                    b.move_to(Point::new(size * 0.2, size * 0.6));
                    b.line_to(Point::new(size * 0.5, size * 0.35));
                    b.line_to(Point::new(size * 0.8, size * 0.6));
                });
                frame.stroke(&chevron, stroke);
            }

            IconName::ChevronDown => {
                let chevron = Path::new(|b| {
                    b.move_to(Point::new(size * 0.2, size * 0.4));
                    b.line_to(Point::new(size * 0.5, size * 0.65));
                    b.line_to(Point::new(size * 0.8, size * 0.4));
                });
                frame.stroke(&chevron, stroke);
            }

            IconName::Plus => {
                let h = Path::line(
                    Point::new(size * 0.2, size * 0.5),
                    Point::new(size * 0.8, size * 0.5),
                );
                let v = Path::line(
                    Point::new(size * 0.5, size * 0.2),
                    Point::new(size * 0.5, size * 0.8),
                );
                frame.stroke(&h, stroke);
                frame.stroke(&v, stroke);
            }

            IconName::Minus => {
                let line = Path::line(
                    Point::new(size * 0.2, size * 0.5),
                    Point::new(size * 0.8, size * 0.5),
                );
                frame.stroke(&line, stroke);
            }

            IconName::Check => {
                let check = Path::new(|b| {
                    b.move_to(Point::new(size * 0.2, size * 0.5));
                    b.line_to(Point::new(size * 0.4, size * 0.7));
                    b.line_to(Point::new(size * 0.8, size * 0.3));
                });
                frame.stroke(&check, stroke);
            }

            IconName::Search => {
                let circle = Path::circle(Point::new(size * 0.42, size * 0.42), size * 0.25);
                frame.stroke(&circle, stroke);
                let handle = Path::line(
                    Point::new(size * 0.6, size * 0.6),
                    Point::new(size * 0.85, size * 0.85),
                );
                frame.stroke(&handle, stroke);
            }

            IconName::Settings => {
                // Gear shape - simplified
                let circle = Path::circle(center, size * 0.2);
                frame.stroke(&circle, stroke);
                // Gear teeth as lines
                for i in 0..6 {
                    let angle = i as f32 * std::f32::consts::PI / 3.0;
                    let inner = size * 0.28;
                    let outer = size * 0.42;
                    let line = Path::line(
                        Point::new(
                            center.x + angle.cos() * inner,
                            center.y + angle.sin() * inner,
                        ),
                        Point::new(
                            center.x + angle.cos() * outer,
                            center.y + angle.sin() * outer,
                        ),
                    );
                    frame.stroke(&line, stroke);
                }
            }

            IconName::Edit => {
                // Pencil shape
                let pencil = Path::new(|b| {
                    b.move_to(Point::new(size * 0.7, size * 0.2));
                    b.line_to(Point::new(size * 0.8, size * 0.3));
                    b.line_to(Point::new(size * 0.35, size * 0.75));
                    b.line_to(Point::new(size * 0.2, size * 0.8));
                    b.line_to(Point::new(size * 0.25, size * 0.65));
                    b.close();
                });
                frame.stroke(&pencil, thin_stroke);
            }

            IconName::Trash => {
                // Trash can
                let lid = Path::line(
                    Point::new(size * 0.2, size * 0.3),
                    Point::new(size * 0.8, size * 0.3),
                );
                frame.stroke(&lid, stroke);
                let can = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.3));
                    b.line_to(Point::new(size * 0.3, size * 0.85));
                    b.line_to(Point::new(size * 0.7, size * 0.85));
                    b.line_to(Point::new(size * 0.75, size * 0.3));
                });
                frame.stroke(&can, thin_stroke);
                // Handle
                let handle = Path::new(|b| {
                    b.move_to(Point::new(size * 0.4, size * 0.3));
                    b.line_to(Point::new(size * 0.4, size * 0.2));
                    b.line_to(Point::new(size * 0.6, size * 0.2));
                    b.line_to(Point::new(size * 0.6, size * 0.3));
                });
                frame.stroke(&handle, thin_stroke);
            }

            IconName::Refresh => {
                // Circular arrow
                let arc = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center,
                        radius: size * 0.3,
                        start_angle: iced::Radians(-0.5),
                        end_angle: iced::Radians(std::f32::consts::PI + 0.5),
                    });
                });
                frame.stroke(&arc, stroke);
                // Arrow head
                let arrow = Path::new(|b| {
                    b.move_to(Point::new(size * 0.65, size * 0.2));
                    b.line_to(Point::new(size * 0.8, size * 0.35));
                    b.line_to(Point::new(size * 0.65, size * 0.35));
                });
                frame.fill(&arrow, color);
            }

            IconName::Download => {
                let arrow = Path::new(|b| {
                    b.move_to(Point::new(size * 0.5, size * 0.15));
                    b.line_to(Point::new(size * 0.5, size * 0.6));
                });
                frame.stroke(&arrow, stroke);
                let head = Path::new(|b| {
                    b.move_to(Point::new(size * 0.3, size * 0.45));
                    b.line_to(Point::new(size * 0.5, size * 0.65));
                    b.line_to(Point::new(size * 0.7, size * 0.45));
                });
                frame.stroke(&head, stroke);
                let base = Path::line(
                    Point::new(size * 0.2, size * 0.85),
                    Point::new(size * 0.8, size * 0.85),
                );
                frame.stroke(&base, stroke);
            }

            IconName::Upload => {
                let arrow = Path::new(|b| {
                    b.move_to(Point::new(size * 0.5, size * 0.6));
                    b.line_to(Point::new(size * 0.5, size * 0.15));
                });
                frame.stroke(&arrow, stroke);
                let head = Path::new(|b| {
                    b.move_to(Point::new(size * 0.3, size * 0.35));
                    b.line_to(Point::new(size * 0.5, size * 0.15));
                    b.line_to(Point::new(size * 0.7, size * 0.35));
                });
                frame.stroke(&head, stroke);
                let base = Path::line(
                    Point::new(size * 0.2, size * 0.85),
                    Point::new(size * 0.8, size * 0.85),
                );
                frame.stroke(&base, stroke);
            }

            IconName::Share => {
                // Three dots connected
                let top = Path::circle(Point::new(size * 0.7, size * 0.25), size * 0.1);
                let mid = Path::circle(Point::new(size * 0.3, size * 0.5), size * 0.1);
                let bot = Path::circle(Point::new(size * 0.7, size * 0.75), size * 0.1);
                frame.fill(&top, color);
                frame.fill(&mid, color);
                frame.fill(&bot, color);
                let line1 = Path::line(
                    Point::new(size * 0.38, size * 0.45),
                    Point::new(size * 0.62, size * 0.3),
                );
                let line2 = Path::line(
                    Point::new(size * 0.38, size * 0.55),
                    Point::new(size * 0.62, size * 0.7),
                );
                frame.stroke(&line1, thin_stroke);
                frame.stroke(&line2, thin_stroke);
            }

            IconName::Copy => {
                // Two overlapping rectangles
                let back = Path::new(|b| {
                    b.move_to(Point::new(size * 0.3, size * 0.2));
                    b.line_to(Point::new(size * 0.75, size * 0.2));
                    b.line_to(Point::new(size * 0.75, size * 0.65));
                    b.line_to(Point::new(size * 0.3, size * 0.65));
                    b.close();
                });
                frame.stroke(&back, thin_stroke);
                let front = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.35));
                    b.line_to(Point::new(size * 0.7, size * 0.35));
                    b.line_to(Point::new(size * 0.7, size * 0.8));
                    b.line_to(Point::new(size * 0.25, size * 0.8));
                    b.close();
                });
                frame.stroke(&front, thin_stroke);
            }

            IconName::Info => {
                let circle = Path::circle(center, size * 0.4);
                frame.stroke(&circle, thin_stroke);
                // i dot
                let dot = Path::circle(Point::new(center.x, size * 0.3), size * 0.05);
                frame.fill(&dot, color);
                // i stem
                let stem = Path::line(
                    Point::new(center.x, size * 0.45),
                    Point::new(center.x, size * 0.75),
                );
                frame.stroke(&stem, stroke);
            }

            IconName::Warning => {
                // Triangle
                let triangle = Path::new(|b| {
                    b.move_to(Point::new(size * 0.5, size * 0.15));
                    b.line_to(Point::new(size * 0.9, size * 0.85));
                    b.line_to(Point::new(size * 0.1, size * 0.85));
                    b.close();
                });
                frame.stroke(&triangle, thin_stroke);
                // Exclamation
                let line = Path::line(
                    Point::new(center.x, size * 0.35),
                    Point::new(center.x, size * 0.6),
                );
                frame.stroke(&line, stroke);
                let dot = Path::circle(Point::new(center.x, size * 0.72), size * 0.04);
                frame.fill(&dot, color);
            }

            IconName::Error => {
                let circle = Path::circle(center, size * 0.4);
                frame.stroke(&circle, thin_stroke);
                // X
                let line1 = Path::line(
                    Point::new(size * 0.35, size * 0.35),
                    Point::new(size * 0.65, size * 0.65),
                );
                let line2 = Path::line(
                    Point::new(size * 0.65, size * 0.35),
                    Point::new(size * 0.35, size * 0.65),
                );
                frame.stroke(&line1, stroke);
                frame.stroke(&line2, stroke);
            }

            IconName::Success => {
                let circle = Path::circle(center, size * 0.4);
                frame.stroke(&circle, thin_stroke);
                // Checkmark
                let check = Path::new(|b| {
                    b.move_to(Point::new(size * 0.3, size * 0.5));
                    b.line_to(Point::new(size * 0.45, size * 0.65));
                    b.line_to(Point::new(size * 0.7, size * 0.35));
                });
                frame.stroke(&check, stroke);
            }

            IconName::Play => {
                let triangle = Path::new(|b| {
                    b.move_to(Point::new(size * 0.3, size * 0.2));
                    b.line_to(Point::new(size * 0.8, size * 0.5));
                    b.line_to(Point::new(size * 0.3, size * 0.8));
                    b.close();
                });
                frame.fill(&triangle, color);
            }

            IconName::Pause => {
                let left = Path::new(|b| {
                    b.move_to(Point::new(size * 0.3, size * 0.2));
                    b.line_to(Point::new(size * 0.42, size * 0.2));
                    b.line_to(Point::new(size * 0.42, size * 0.8));
                    b.line_to(Point::new(size * 0.3, size * 0.8));
                    b.close();
                });
                let right = Path::new(|b| {
                    b.move_to(Point::new(size * 0.58, size * 0.2));
                    b.line_to(Point::new(size * 0.7, size * 0.2));
                    b.line_to(Point::new(size * 0.7, size * 0.8));
                    b.line_to(Point::new(size * 0.58, size * 0.8));
                    b.close();
                });
                frame.fill(&left, color);
                frame.fill(&right, color);
            }

            IconName::Stop => {
                let square = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.25));
                    b.line_to(Point::new(size * 0.75, size * 0.25));
                    b.line_to(Point::new(size * 0.75, size * 0.75));
                    b.line_to(Point::new(size * 0.25, size * 0.75));
                    b.close();
                });
                frame.fill(&square, color);
            }

            IconName::Record => {
                let circle = Path::circle(center, size * 0.35);
                frame.fill(&circle, color);
            }

            IconName::Volume => {
                // Speaker shape
                let speaker = Path::new(|b| {
                    b.move_to(Point::new(size * 0.2, size * 0.4));
                    b.line_to(Point::new(size * 0.35, size * 0.4));
                    b.line_to(Point::new(size * 0.5, size * 0.25));
                    b.line_to(Point::new(size * 0.5, size * 0.75));
                    b.line_to(Point::new(size * 0.35, size * 0.6));
                    b.line_to(Point::new(size * 0.2, size * 0.6));
                    b.close();
                });
                frame.fill(&speaker, color);
                // Sound waves
                let wave1 = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.5),
                        radius: size * 0.15,
                        start_angle: iced::Radians(-0.8),
                        end_angle: iced::Radians(0.8),
                    });
                });
                let wave2 = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.5),
                        radius: size * 0.28,
                        start_angle: iced::Radians(-0.8),
                        end_angle: iced::Radians(0.8),
                    });
                });
                frame.stroke(&wave1, thin_stroke);
                frame.stroke(&wave2, thin_stroke);
            }

            IconName::Mute => {
                // Speaker without waves
                let speaker = Path::new(|b| {
                    b.move_to(Point::new(size * 0.15, size * 0.4));
                    b.line_to(Point::new(size * 0.3, size * 0.4));
                    b.line_to(Point::new(size * 0.45, size * 0.25));
                    b.line_to(Point::new(size * 0.45, size * 0.75));
                    b.line_to(Point::new(size * 0.3, size * 0.6));
                    b.line_to(Point::new(size * 0.15, size * 0.6));
                    b.close();
                });
                frame.fill(&speaker, color);
                // X
                let line1 = Path::line(
                    Point::new(size * 0.55, size * 0.35),
                    Point::new(size * 0.85, size * 0.65),
                );
                let line2 = Path::line(
                    Point::new(size * 0.85, size * 0.35),
                    Point::new(size * 0.55, size * 0.65),
                );
                frame.stroke(&line1, stroke);
                frame.stroke(&line2, stroke);
            }

            IconName::Microphone => {
                // Mic body
                let mic = Path::new(|b| {
                    b.move_to(Point::new(size * 0.35, size * 0.15));
                    b.line_to(Point::new(size * 0.65, size * 0.15));
                    b.line_to(Point::new(size * 0.65, size * 0.5));
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.5),
                        radius: size * 0.15,
                        start_angle: iced::Radians(0.0),
                        end_angle: iced::Radians(std::f32::consts::PI),
                    });
                    b.line_to(Point::new(size * 0.35, size * 0.15));
                });
                frame.stroke(&mic, thin_stroke);
                // Stand
                let stand = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.5),
                        radius: size * 0.28,
                        start_angle: iced::Radians(0.3),
                        end_angle: iced::Radians(std::f32::consts::PI - 0.3),
                    });
                });
                frame.stroke(&stand, thin_stroke);
                let stem = Path::line(
                    Point::new(size * 0.5, size * 0.78),
                    Point::new(size * 0.5, size * 0.9),
                );
                frame.stroke(&stem, thin_stroke);
            }

            IconName::Camera => {
                // Camera body
                let body = Path::new(|b| {
                    b.move_to(Point::new(size * 0.1, size * 0.35));
                    b.line_to(Point::new(size * 0.3, size * 0.35));
                    b.line_to(Point::new(size * 0.38, size * 0.22));
                    b.line_to(Point::new(size * 0.62, size * 0.22));
                    b.line_to(Point::new(size * 0.7, size * 0.35));
                    b.line_to(Point::new(size * 0.9, size * 0.35));
                    b.line_to(Point::new(size * 0.9, size * 0.78));
                    b.line_to(Point::new(size * 0.1, size * 0.78));
                    b.close();
                });
                frame.stroke(&body, thin_stroke);
                // Lens
                let lens = Path::circle(Point::new(size * 0.5, size * 0.55), size * 0.15);
                frame.stroke(&lens, thin_stroke);
            }

            IconName::File => {
                let file = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.1));
                    b.line_to(Point::new(size * 0.6, size * 0.1));
                    b.line_to(Point::new(size * 0.75, size * 0.25));
                    b.line_to(Point::new(size * 0.75, size * 0.9));
                    b.line_to(Point::new(size * 0.25, size * 0.9));
                    b.close();
                });
                frame.stroke(&file, thin_stroke);
                // Fold corner
                let fold = Path::new(|b| {
                    b.move_to(Point::new(size * 0.6, size * 0.1));
                    b.line_to(Point::new(size * 0.6, size * 0.25));
                    b.line_to(Point::new(size * 0.75, size * 0.25));
                });
                frame.stroke(&fold, thin_stroke);
            }

            IconName::Folder => {
                let folder = Path::new(|b| {
                    b.move_to(Point::new(size * 0.1, size * 0.25));
                    b.line_to(Point::new(size * 0.35, size * 0.25));
                    b.line_to(Point::new(size * 0.42, size * 0.35));
                    b.line_to(Point::new(size * 0.9, size * 0.35));
                    b.line_to(Point::new(size * 0.9, size * 0.8));
                    b.line_to(Point::new(size * 0.1, size * 0.8));
                    b.close();
                });
                frame.stroke(&folder, thin_stroke);
            }

            IconName::Image => {
                // Frame
                let frame_path = Path::new(|b| {
                    b.move_to(Point::new(size * 0.15, size * 0.2));
                    b.line_to(Point::new(size * 0.85, size * 0.2));
                    b.line_to(Point::new(size * 0.85, size * 0.8));
                    b.line_to(Point::new(size * 0.15, size * 0.8));
                    b.close();
                });
                frame.stroke(&frame_path, thin_stroke);
                // Mountain
                let mountain = Path::new(|b| {
                    b.move_to(Point::new(size * 0.15, size * 0.7));
                    b.line_to(Point::new(size * 0.4, size * 0.45));
                    b.line_to(Point::new(size * 0.55, size * 0.55));
                    b.line_to(Point::new(size * 0.75, size * 0.35));
                    b.line_to(Point::new(size * 0.85, size * 0.55));
                });
                frame.stroke(&mountain, thin_stroke);
                // Sun
                let sun = Path::circle(Point::new(size * 0.7, size * 0.35), size * 0.07);
                frame.fill(&sun, color);
            }

            IconName::Grid => {
                // 2x2 grid
                for row in 0..2 {
                    for col in 0..2 {
                        let x = size * (0.2 + col as f32 * 0.35);
                        let y = size * (0.2 + row as f32 * 0.35);
                        let rect = Path::new(|b| {
                            b.move_to(Point::new(x, y));
                            b.line_to(Point::new(x + size * 0.25, y));
                            b.line_to(Point::new(x + size * 0.25, y + size * 0.25));
                            b.line_to(Point::new(x, y + size * 0.25));
                            b.close();
                        });
                        frame.stroke(&rect, thin_stroke);
                    }
                }
            }

            IconName::List => {
                for i in 0..3 {
                    let y = size * (0.3 + i as f32 * 0.2);
                    // Bullet
                    let bullet = Path::circle(Point::new(size * 0.25, y), size * 0.04);
                    frame.fill(&bullet, color);
                    // Line
                    let line = Path::line(
                        Point::new(size * 0.35, y),
                        Point::new(size * 0.8, y),
                    );
                    frame.stroke(&line, thin_stroke);
                }
            }

            IconName::Mail => {
                // Envelope
                let envelope = Path::new(|b| {
                    b.move_to(Point::new(size * 0.1, size * 0.25));
                    b.line_to(Point::new(size * 0.9, size * 0.25));
                    b.line_to(Point::new(size * 0.9, size * 0.75));
                    b.line_to(Point::new(size * 0.1, size * 0.75));
                    b.close();
                });
                frame.stroke(&envelope, thin_stroke);
                // Flap
                let flap = Path::new(|b| {
                    b.move_to(Point::new(size * 0.1, size * 0.25));
                    b.line_to(Point::new(size * 0.5, size * 0.5));
                    b.line_to(Point::new(size * 0.9, size * 0.25));
                });
                frame.stroke(&flap, thin_stroke);
            }

            IconName::Chat => {
                // Speech bubble
                let bubble = Path::new(|b| {
                    b.move_to(Point::new(size * 0.15, size * 0.2));
                    b.line_to(Point::new(size * 0.85, size * 0.2));
                    b.line_to(Point::new(size * 0.85, size * 0.6));
                    b.line_to(Point::new(size * 0.4, size * 0.6));
                    b.line_to(Point::new(size * 0.25, size * 0.8));
                    b.line_to(Point::new(size * 0.25, size * 0.6));
                    b.line_to(Point::new(size * 0.15, size * 0.6));
                    b.close();
                });
                frame.stroke(&bubble, thin_stroke);
            }

            IconName::Bell => {
                // Bell shape
                let bell = Path::new(|b| {
                    b.move_to(Point::new(size * 0.5, size * 0.12));
                    b.line_to(Point::new(size * 0.5, size * 0.2));
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.45),
                        radius: size * 0.25,
                        start_angle: iced::Radians(-std::f32::consts::PI),
                        end_angle: iced::Radians(0.0),
                    });
                    b.line_to(Point::new(size * 0.85, size * 0.7));
                    b.line_to(Point::new(size * 0.15, size * 0.7));
                    b.line_to(Point::new(size * 0.25, size * 0.45));
                });
                frame.stroke(&bell, thin_stroke);
                // Clapper
                let clapper = Path::circle(Point::new(size * 0.5, size * 0.82), size * 0.07);
                frame.fill(&clapper, color);
            }

            IconName::User => {
                // Head
                let head = Path::circle(Point::new(size * 0.5, size * 0.32), size * 0.18);
                frame.stroke(&head, thin_stroke);
                // Body
                let body = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.95),
                        radius: size * 0.35,
                        start_angle: iced::Radians(-std::f32::consts::PI),
                        end_angle: iced::Radians(0.0),
                    });
                });
                frame.stroke(&body, thin_stroke);
            }

            IconName::Users => {
                // Two users overlapping
                // Back user
                let head1 = Path::circle(Point::new(size * 0.35, size * 0.32), size * 0.12);
                frame.stroke(&head1, thin_stroke);
                // Front user
                let head2 = Path::circle(Point::new(size * 0.6, size * 0.35), size * 0.15);
                frame.stroke(&head2, thin_stroke);
                let body2 = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.6, size * 0.95),
                        radius: size * 0.28,
                        start_angle: iced::Radians(-std::f32::consts::PI),
                        end_angle: iced::Radians(0.0),
                    });
                });
                frame.stroke(&body2, thin_stroke);
            }

            IconName::Sun => {
                // Center circle
                let circle = Path::circle(center, size * 0.18);
                frame.fill(&circle, color);
                // Rays
                for i in 0..8 {
                    let angle = i as f32 * std::f32::consts::PI / 4.0;
                    let inner = size * 0.28;
                    let outer = size * 0.42;
                    let line = Path::line(
                        Point::new(
                            center.x + angle.cos() * inner,
                            center.y + angle.sin() * inner,
                        ),
                        Point::new(
                            center.x + angle.cos() * outer,
                            center.y + angle.sin() * outer,
                        ),
                    );
                    frame.stroke(&line, stroke);
                }
            }

            IconName::Moon => {
                // Crescent moon using arc paths
                let crescent = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center,
                        radius: size * 0.35,
                        start_angle: iced::Radians(-2.3),
                        end_angle: iced::Radians(2.3),
                    });
                    b.arc(canvas::path::Arc {
                        center: Point::new(center.x + size * 0.12, center.y),
                        radius: size * 0.28,
                        start_angle: iced::Radians(2.3),
                        end_angle: iced::Radians(-2.3),
                    });
                });
                frame.fill(&crescent, color);
            }

            IconName::Globe => {
                // Circle
                let circle = Path::circle(center, size * 0.38);
                frame.stroke(&circle, thin_stroke);
                // Horizontal line
                let h = Path::line(
                    Point::new(size * 0.12, size * 0.5),
                    Point::new(size * 0.88, size * 0.5),
                );
                frame.stroke(&h, thin_stroke);
                // Vertical ellipse
                let v = Path::new(|b| {
                    b.arc(canvas::path::Arc {
                        center,
                        radius: size * 0.2,
                        start_angle: iced::Radians(0.0),
                        end_angle: iced::Radians(std::f32::consts::PI * 2.0),
                    });
                });
                frame.stroke(&v, thin_stroke);
            }

            IconName::Link => {
                // Chain links
                let link1 = Path::new(|b| {
                    b.move_to(Point::new(size * 0.45, size * 0.35));
                    b.line_to(Point::new(size * 0.3, size * 0.35));
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.3, size * 0.5),
                        radius: size * 0.15,
                        start_angle: iced::Radians(-std::f32::consts::PI / 2.0),
                        end_angle: iced::Radians(std::f32::consts::PI / 2.0),
                    });
                    b.line_to(Point::new(size * 0.45, size * 0.65));
                });
                frame.stroke(&link1, stroke);
                let link2 = Path::new(|b| {
                    b.move_to(Point::new(size * 0.55, size * 0.35));
                    b.line_to(Point::new(size * 0.7, size * 0.35));
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.7, size * 0.5),
                        radius: size * 0.15,
                        start_angle: iced::Radians(-std::f32::consts::PI / 2.0),
                        end_angle: iced::Radians(-std::f32::consts::PI * 1.5),
                    });
                    b.line_to(Point::new(size * 0.55, size * 0.65));
                });
                frame.stroke(&link2, stroke);
            }

            IconName::Star => {
                // 5-pointed star
                let star = Path::new(|b| {
                    let points = 5;
                    let outer_r = size * 0.4;
                    let inner_r = size * 0.18;
                    for i in 0..points * 2 {
                        let r = if i % 2 == 0 { outer_r } else { inner_r };
                        let angle = (i as f32 * std::f32::consts::PI / points as f32)
                            - std::f32::consts::PI / 2.0;
                        let point = Point::new(
                            center.x + angle.cos() * r,
                            center.y + angle.sin() * r,
                        );
                        if i == 0 {
                            b.move_to(point);
                        } else {
                            b.line_to(point);
                        }
                    }
                    b.close();
                });
                frame.stroke(&star, thin_stroke);
            }

            IconName::Heart => {
                // Heart shape using bezier curves
                let heart = Path::new(|b| {
                    b.move_to(Point::new(size * 0.5, size * 0.35));
                    b.bezier_curve_to(
                        Point::new(size * 0.5, size * 0.25),
                        Point::new(size * 0.3, size * 0.2),
                        Point::new(size * 0.2, size * 0.35),
                    );
                    b.bezier_curve_to(
                        Point::new(size * 0.1, size * 0.5),
                        Point::new(size * 0.2, size * 0.65),
                        Point::new(size * 0.5, size * 0.85),
                    );
                    b.bezier_curve_to(
                        Point::new(size * 0.8, size * 0.65),
                        Point::new(size * 0.9, size * 0.5),
                        Point::new(size * 0.8, size * 0.35),
                    );
                    b.bezier_curve_to(
                        Point::new(size * 0.7, size * 0.2),
                        Point::new(size * 0.5, size * 0.25),
                        Point::new(size * 0.5, size * 0.35),
                    );
                });
                frame.stroke(&heart, thin_stroke);
            }

            IconName::Eye => {
                // Eye shape
                let eye = Path::new(|b| {
                    b.move_to(Point::new(size * 0.1, size * 0.5));
                    b.bezier_curve_to(
                        Point::new(size * 0.25, size * 0.25),
                        Point::new(size * 0.75, size * 0.25),
                        Point::new(size * 0.9, size * 0.5),
                    );
                    b.bezier_curve_to(
                        Point::new(size * 0.75, size * 0.75),
                        Point::new(size * 0.25, size * 0.75),
                        Point::new(size * 0.1, size * 0.5),
                    );
                });
                frame.stroke(&eye, thin_stroke);
                // Pupil
                let pupil = Path::circle(center, size * 0.12);
                frame.fill(&pupil, color);
            }

            IconName::EyeOff => {
                // Eye shape
                let eye = Path::new(|b| {
                    b.move_to(Point::new(size * 0.1, size * 0.5));
                    b.bezier_curve_to(
                        Point::new(size * 0.25, size * 0.25),
                        Point::new(size * 0.75, size * 0.25),
                        Point::new(size * 0.9, size * 0.5),
                    );
                    b.bezier_curve_to(
                        Point::new(size * 0.75, size * 0.75),
                        Point::new(size * 0.25, size * 0.75),
                        Point::new(size * 0.1, size * 0.5),
                    );
                });
                frame.stroke(&eye, thin_stroke);
                // Slash
                let slash = Path::line(
                    Point::new(size * 0.2, size * 0.8),
                    Point::new(size * 0.8, size * 0.2),
                );
                frame.stroke(&slash, stroke);
            }

            IconName::Lock => {
                // Lock body
                let body = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.45));
                    b.line_to(Point::new(size * 0.75, size * 0.45));
                    b.line_to(Point::new(size * 0.75, size * 0.85));
                    b.line_to(Point::new(size * 0.25, size * 0.85));
                    b.close();
                });
                frame.stroke(&body, thin_stroke);
                // Shackle
                let shackle = Path::new(|b| {
                    b.move_to(Point::new(size * 0.35, size * 0.45));
                    b.line_to(Point::new(size * 0.35, size * 0.32));
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.32),
                        radius: size * 0.15,
                        start_angle: iced::Radians(std::f32::consts::PI),
                        end_angle: iced::Radians(0.0),
                    });
                    b.line_to(Point::new(size * 0.65, size * 0.45));
                });
                frame.stroke(&shackle, thin_stroke);
            }

            IconName::Unlock => {
                // Lock body
                let body = Path::new(|b| {
                    b.move_to(Point::new(size * 0.25, size * 0.45));
                    b.line_to(Point::new(size * 0.75, size * 0.45));
                    b.line_to(Point::new(size * 0.75, size * 0.85));
                    b.line_to(Point::new(size * 0.25, size * 0.85));
                    b.close();
                });
                frame.stroke(&body, thin_stroke);
                // Open shackle
                let shackle = Path::new(|b| {
                    b.move_to(Point::new(size * 0.35, size * 0.45));
                    b.line_to(Point::new(size * 0.35, size * 0.32));
                    b.arc(canvas::path::Arc {
                        center: Point::new(size * 0.5, size * 0.32),
                        radius: size * 0.15,
                        start_angle: iced::Radians(std::f32::consts::PI),
                        end_angle: iced::Radians(0.2),
                    });
                });
                frame.stroke(&shackle, thin_stroke);
            }
        }
    }
}

impl<Message> canvas::Program<Message, Theme> for IconProgram {
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

        let color = self.color.unwrap_or_else(|| {
            theme.extended_palette().background.base.text
        });

        self.draw_icon(&mut frame, bounds, color);

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<Icon> for Element<'a, Message, Theme> {
    fn from(icon: Icon) -> Self {
        let size = icon.size;
        let program = IconProgram {
            name: icon.name,
            color: icon.color,
        };
        Canvas::new(program)
            .width(Length::Fixed(size))
            .height(Length::Fixed(size))
            .into()
    }
}

/// Create an icon element quickly.
///
/// # Example
///
/// ```rust,ignore
/// icon(IconName::Home, 24.0)
/// ```
pub fn icon<'a, Message: 'a>(name: IconName, size: f32) -> Element<'a, Message, Theme> {
    Icon::new(name).size(size).into()
}

/// Create an icon with a specific color.
pub fn icon_colored<'a, Message: 'a>(
    name: IconName,
    size: f32,
    color: Color,
) -> Element<'a, Message, Theme> {
    Icon::new(name).size(size).color(color).into()
}
