//! Image component with loading states.

use iced::advanced::image as adv_image;
use iced::widget::Image as IcedImage;
use iced::{ContentFit, Element, Length};
use std::path::PathBuf;

/// Image source type.
#[derive(Debug, Clone)]
pub enum ImageSource {
    /// Load from file path.
    Path(PathBuf),
    /// Load from bytes.
    Bytes(Vec<u8>),
    /// Load from iced Handle.
    Handle(adv_image::Handle),
}

impl From<PathBuf> for ImageSource {
    fn from(path: PathBuf) -> Self {
        Self::Path(path)
    }
}

impl From<&str> for ImageSource {
    fn from(path: &str) -> Self {
        Self::Path(PathBuf::from(path))
    }
}

impl From<String> for ImageSource {
    fn from(path: String) -> Self {
        Self::Path(PathBuf::from(path))
    }
}

impl From<Vec<u8>> for ImageSource {
    fn from(bytes: Vec<u8>) -> Self {
        Self::Bytes(bytes)
    }
}

impl From<&[u8]> for ImageSource {
    fn from(bytes: &[u8]) -> Self {
        Self::Bytes(bytes.to_vec())
    }
}

impl From<adv_image::Handle> for ImageSource {
    fn from(handle: adv_image::Handle) -> Self {
        Self::Handle(handle)
    }
}

/// An image component with convenient sizing options.
///
/// # Example
///
/// ```rust,ignore
/// Image::new("./photo.png")
///     .width(200.0)
///     .height(150.0)
///     .fit(ContentFit::Cover)
/// ```
pub struct Image {
    source: ImageSource,
    width: Length,
    height: Length,
    content_fit: ContentFit,
    filter_method: adv_image::FilterMethod,
}

impl Image {
    /// Create a new image from a source.
    pub fn new(source: impl Into<ImageSource>) -> Self {
        Self {
            source: source.into(),
            width: Length::Shrink,
            height: Length::Shrink,
            content_fit: ContentFit::Contain,
            filter_method: adv_image::FilterMethod::Linear,
        }
    }

    /// Create an image from bytes.
    pub fn from_bytes(bytes: impl Into<Vec<u8>>) -> Self {
        Self::new(ImageSource::Bytes(bytes.into()))
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set both width and height.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.width = Length::Fixed(size);
        self.height = Length::Fixed(size);
        self
    }

    /// Set the content fit mode.
    #[must_use]
    pub fn fit(mut self, fit: ContentFit) -> Self {
        self.content_fit = fit;
        self
    }

    /// Use nearest neighbor filtering (good for pixel art).
    #[must_use]
    pub fn nearest(mut self) -> Self {
        self.filter_method = adv_image::FilterMethod::Nearest;
        self
    }

    /// Use linear filtering (default, good for photos).
    #[must_use]
    pub fn linear(mut self) -> Self {
        self.filter_method = adv_image::FilterMethod::Linear;
        self
    }
}

impl<'a, Message: 'a> From<Image> for Element<'a, Message, iced::Theme> {
    fn from(img: Image) -> Self {
        let handle = match img.source {
            ImageSource::Path(path) => adv_image::Handle::from_path(path),
            ImageSource::Bytes(bytes) => adv_image::Handle::from_bytes(bytes),
            ImageSource::Handle(handle) => handle,
        };

        IcedImage::new(handle)
            .width(img.width)
            .height(img.height)
            .content_fit(img.content_fit)
            .filter_method(img.filter_method)
            .into()
    }
}

/// A placeholder image shown while loading or on error.
///
/// # Example
///
/// ```rust,ignore
/// ImagePlaceholder::new()
///     .width(200.0)
///     .height(150.0)
///     .message("Loading...")
/// ```
pub struct ImagePlaceholder {
    width: Length,
    height: Length,
    message: Option<String>,
}

impl Default for ImagePlaceholder {
    fn default() -> Self {
        Self::new()
    }
}

impl ImagePlaceholder {
    /// Create a new placeholder.
    #[must_use]
    pub fn new() -> Self {
        Self {
            width: Length::Fixed(100.0),
            height: Length::Fixed(100.0),
            message: None,
        }
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Set a placeholder message.
    #[must_use]
    pub fn message(mut self, message: impl Into<String>) -> Self {
        self.message = Some(message.into());
        self
    }
}

impl<'a, Message: 'a> From<ImagePlaceholder> for Element<'a, Message, iced::Theme> {
    fn from(placeholder: ImagePlaceholder) -> Self {
        use iced::widget::{center, container, text};
        use iced::{Background, Border, Color};

        let content: Element<'a, Message, iced::Theme> = if let Some(msg) = placeholder.message {
            center(text(msg).size(14)).into()
        } else {
            center(text("")).into()
        };

        container(content)
            .width(placeholder.width)
            .height(placeholder.height)
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgb(0.9, 0.9, 0.9))),
                border: Border {
                    color: Color::from_rgb(0.8, 0.8, 0.8),
                    width: 1.0,
                    radius: 4.0.into(),
                },
                ..Default::default()
            })
            .into()
    }
}
