//! Color picker component.
//!
//! Provides interactive color selection with visual palette and hex input.
//!
//! # Example
//!
//! ```rust,ignore
//! ColorPicker::new(self.color, Message::ColorChanged)
//!     .width(200.0)
//! ```

use iced::widget::canvas::{self, Canvas, Frame, Geometry, Path};
use iced::widget::{column, container, row, text, text_input, Space};
use iced::{
    alignment, mouse, Background, Border, Color, Element, Length, Point, Rectangle, Renderer,
    Size, Theme,
};

/// HSL color representation.
#[derive(Debug, Clone, Copy)]
pub struct Hsl {
    /// Hue (0.0 - 360.0)
    pub h: f32,
    /// Saturation (0.0 - 1.0)
    pub s: f32,
    /// Lightness (0.0 - 1.0)
    pub l: f32,
}

impl Hsl {
    /// Create a new HSL color.
    #[must_use]
    pub fn new(h: f32, s: f32, l: f32) -> Self {
        Self {
            h: h.clamp(0.0, 360.0),
            s: s.clamp(0.0, 1.0),
            l: l.clamp(0.0, 1.0),
        }
    }

    /// Convert HSL to RGB Color.
    #[must_use]
    pub fn to_color(self) -> Color {
        let h = self.h / 360.0;
        let s = self.s;
        let l = self.l;

        if s == 0.0 {
            return Color::from_rgb(l, l, l);
        }

        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;

        let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
        let g = hue_to_rgb(p, q, h);
        let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

        Color::from_rgb(r, g, b)
    }

    /// Create HSL from RGB Color.
    #[must_use]
    pub fn from_color(color: Color) -> Self {
        let r = color.r;
        let g = color.g;
        let b = color.b;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let l = (max + min) / 2.0;

        if max == min {
            return Self { h: 0.0, s: 0.0, l };
        }

        let d = max - min;
        let s = if l > 0.5 {
            d / (2.0 - max - min)
        } else {
            d / (max + min)
        };

        let h = if max == r {
            (g - b) / d + (if g < b { 6.0 } else { 0.0 })
        } else if max == g {
            (b - r) / d + 2.0
        } else {
            (r - g) / d + 4.0
        };

        Self {
            h: h * 60.0,
            s,
            l,
        }
    }
}

fn hue_to_rgb(p: f32, q: f32, t: f32) -> f32 {
    let t = if t < 0.0 {
        t + 1.0
    } else if t > 1.0 {
        t - 1.0
    } else {
        t
    };

    if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
    } else if t < 1.0 / 2.0 {
        q
    } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
    } else {
        p
    }
}

/// Convert Color to hex string.
#[must_use]
pub fn color_to_hex(color: Color) -> String {
    format!(
        "#{:02X}{:02X}{:02X}",
        (color.r * 255.0) as u8,
        (color.g * 255.0) as u8,
        (color.b * 255.0) as u8
    )
}

/// Parse hex string to Color.
#[must_use]
pub fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some(Color::from_rgb8(r, g, b))
}

/// A color swatch display.
pub struct ColorSwatch {
    color: Color,
    size: f32,
    border: bool,
}

impl ColorSwatch {
    /// Create a new color swatch.
    #[must_use]
    pub fn new(color: Color) -> Self {
        Self {
            color,
            size: 32.0,
            border: true,
        }
    }

    /// Set the swatch size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    /// Set whether to show a border.
    #[must_use]
    pub fn border(mut self, border: bool) -> Self {
        self.border = border;
        self
    }
}

impl<'a, Message: 'a> From<ColorSwatch> for Element<'a, Message, Theme> {
    fn from(swatch: ColorSwatch) -> Self {
        container(Space::new(swatch.size, swatch.size))
            .style(move |_theme| container::Style {
                background: Some(Background::Color(swatch.color)),
                border: if swatch.border {
                    Border {
                        color: Color::from_rgb(0.5, 0.5, 0.5),
                        width: 1.0,
                        radius: 4.0.into(),
                    }
                } else {
                    Border::default()
                },
                ..container::Style::default()
            })
            .into()
    }
}

/// A hue slider for color selection.
pub struct HueSlider {
    hue: f32,
    width: f32,
    height: f32,
}

impl HueSlider {
    /// Create a new hue slider.
    #[must_use]
    pub fn new(hue: f32) -> Self {
        Self {
            hue: hue.clamp(0.0, 360.0),
            width: 200.0,
            height: 20.0,
        }
    }

    /// Set the width.
    #[must_use]
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }

    /// Set the height.
    #[must_use]
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}

struct HueSliderProgram {
    hue: f32,
}

impl<Message> canvas::Program<Message, Theme> for HueSliderProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Draw hue gradient
        let step_count = 36;
        let step_width = bounds.width / step_count as f32;

        for i in 0..step_count {
            let hue = i as f32 * (360.0 / step_count as f32);
            let color = Hsl::new(hue, 1.0, 0.5).to_color();
            let rect = Path::rectangle(
                Point::new(i as f32 * step_width, 0.0),
                Size::new(step_width + 1.0, bounds.height),
            );
            frame.fill(&rect, color);
        }

        // Draw indicator
        let indicator_x = (self.hue / 360.0) * bounds.width;
        let indicator = Path::rectangle(
            Point::new(indicator_x - 2.0, 0.0),
            Size::new(4.0, bounds.height),
        );
        frame.fill(&indicator, Color::WHITE);

        // Outline for indicator
        let outline = Path::rectangle(
            Point::new(indicator_x - 3.0, 0.0),
            Size::new(6.0, bounds.height),
        );
        frame.stroke(
            &outline,
            canvas::Stroke::default()
                .with_width(1.0)
                .with_color(Color::BLACK),
        );

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<HueSlider> for Element<'a, Message, Theme> {
    fn from(slider: HueSlider) -> Self {
        let program = HueSliderProgram { hue: slider.hue };
        Canvas::new(program)
            .width(Length::Fixed(slider.width))
            .height(Length::Fixed(slider.height))
            .into()
    }
}

/// A saturation-lightness picker area.
pub struct SatLightPicker {
    hue: f32,
    saturation: f32,
    lightness: f32,
    size: f32,
}

impl SatLightPicker {
    /// Create a new saturation-lightness picker.
    #[must_use]
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        Self {
            hue: hue.clamp(0.0, 360.0),
            saturation: saturation.clamp(0.0, 1.0),
            lightness: lightness.clamp(0.0, 1.0),
            size: 150.0,
        }
    }

    /// Set the size.
    #[must_use]
    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }
}

struct SatLightProgram {
    hue: f32,
    saturation: f32,
    lightness: f32,
}

impl<Message> canvas::Program<Message, Theme> for SatLightProgram {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let mut frame = Frame::new(renderer, bounds.size());

        // Draw saturation/lightness gradient
        let step_count = 20;
        let step_w = bounds.width / step_count as f32;
        let step_h = bounds.height / step_count as f32;

        for x in 0..step_count {
            for y in 0..step_count {
                let sat = x as f32 / (step_count - 1) as f32;
                let light = 1.0 - (y as f32 / (step_count - 1) as f32);
                let color = Hsl::new(self.hue, sat, light).to_color();
                let rect = Path::rectangle(
                    Point::new(x as f32 * step_w, y as f32 * step_h),
                    Size::new(step_w + 1.0, step_h + 1.0),
                );
                frame.fill(&rect, color);
            }
        }

        // Draw indicator
        let indicator_x = self.saturation * bounds.width;
        let indicator_y = (1.0 - self.lightness) * bounds.height;
        let indicator = Path::circle(Point::new(indicator_x, indicator_y), 6.0);
        frame.stroke(
            &indicator,
            canvas::Stroke::default()
                .with_width(2.0)
                .with_color(Color::WHITE),
        );
        let outline = Path::circle(Point::new(indicator_x, indicator_y), 7.0);
        frame.stroke(
            &outline,
            canvas::Stroke::default()
                .with_width(1.0)
                .with_color(Color::BLACK),
        );

        vec![frame.into_geometry()]
    }
}

impl<'a, Message: 'a> From<SatLightPicker> for Element<'a, Message, Theme> {
    fn from(picker: SatLightPicker) -> Self {
        let program = SatLightProgram {
            hue: picker.hue,
            saturation: picker.saturation,
            lightness: picker.lightness,
        };
        Canvas::new(program)
            .width(Length::Fixed(picker.size))
            .height(Length::Fixed(picker.size))
            .into()
    }
}

/// Color picker display component (view-only, use with your own state management).
///
/// This displays the color picker UI. For interactivity, you need to:
/// 1. Track color state in your app
/// 2. Handle click events on the canvas areas
/// 3. Update the hex input
///
/// # Example
///
/// ```rust,ignore
/// // In state:
/// selected_color: Color,
/// hex_input: String,
///
/// // In view:
/// color_picker_view(
///     self.selected_color,
///     &self.hex_input,
///     Message::HexInputChanged,
/// )
/// ```
pub fn color_picker_view<'a, Message>(
    color: Color,
    hex_input: &str,
    on_hex_change: impl Fn(String) -> Message + 'a,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    let hsl = Hsl::from_color(color);

    let sat_light_picker = SatLightPicker::new(hsl.h, hsl.s, hsl.l).size(150.0);
    let hue_slider = HueSlider::new(hsl.h).width(150.0).height(16.0);
    let preview = ColorSwatch::new(color).size(40.0);

    let hex_field = text_input("Hex", hex_input)
        .on_input(on_hex_change)
        .width(Length::Fixed(100.0));

    let content = column![
        Element::from(sat_light_picker),
        Space::with_height(8),
        Element::from(hue_slider),
        Space::with_height(8),
        row![
            Element::from(preview),
            Space::with_width(8),
            column![
                text("Hex").size(12),
                hex_field,
            ]
            .spacing(4),
        ]
        .align_y(alignment::Vertical::Center),
    ]
    .spacing(8);

    container(content)
        .padding(16)
        .style(|theme: &Theme| {
            let palette = theme.extended_palette();
            container::Style {
                background: Some(Background::Color(palette.background.base.color)),
                border: Border {
                    color: palette.background.strong.color,
                    width: 1.0,
                    radius: 8.0.into(),
                },
                ..container::Style::default()
            }
        })
        .into()
}

/// Color palette display with selectable swatches.
pub fn color_palette<'a, Message>(
    colors: &'a [Color],
    selected: Option<usize>,
    on_select: impl Fn(usize) -> Message + Copy + 'a,
) -> Element<'a, Message, Theme>
where
    Message: Clone + 'a,
{
    use iced::widget::button;

    let swatches: Vec<Element<'a, Message, Theme>> = colors
        .iter()
        .enumerate()
        .map(|(i, &color)| {
            let is_selected = selected == Some(i);
            let swatch_content = container(Space::new(28.0, 28.0))
                .style(move |_theme| container::Style {
                    background: Some(Background::Color(color)),
                    border: Border {
                        color: if is_selected {
                            Color::WHITE
                        } else {
                            Color::from_rgb(0.3, 0.3, 0.3)
                        },
                        width: if is_selected { 2.0 } else { 1.0 },
                        radius: 4.0.into(),
                    },
                    ..container::Style::default()
                });

            button(swatch_content)
                .style(|_theme, _status| button::Style {
                    background: None,
                    ..button::Style::default()
                })
                .padding(2)
                .on_press(on_select(i))
                .into()
        })
        .collect();

    row(swatches).spacing(4).wrap().into()
}

/// Common color presets.
pub mod presets {
    use iced::Color;

    /// Material Design primary colors.
    pub const MATERIAL_PRIMARY: &[Color] = &[
        Color::from_rgb(0.957, 0.263, 0.212), // Red
        Color::from_rgb(0.914, 0.118, 0.388), // Pink
        Color::from_rgb(0.612, 0.153, 0.690), // Purple
        Color::from_rgb(0.404, 0.227, 0.718), // Deep Purple
        Color::from_rgb(0.247, 0.318, 0.710), // Indigo
        Color::from_rgb(0.129, 0.588, 0.953), // Blue
        Color::from_rgb(0.012, 0.663, 0.957), // Light Blue
        Color::from_rgb(0.000, 0.737, 0.831), // Cyan
        Color::from_rgb(0.000, 0.588, 0.533), // Teal
        Color::from_rgb(0.298, 0.686, 0.314), // Green
        Color::from_rgb(0.545, 0.765, 0.290), // Light Green
        Color::from_rgb(0.804, 0.863, 0.224), // Lime
        Color::from_rgb(1.000, 0.922, 0.231), // Yellow
        Color::from_rgb(1.000, 0.757, 0.027), // Amber
        Color::from_rgb(1.000, 0.596, 0.000), // Orange
        Color::from_rgb(1.000, 0.341, 0.133), // Deep Orange
    ];

    /// Grayscale colors.
    pub const GRAYSCALE: &[Color] = &[
        Color::BLACK,
        Color::from_rgb(0.13, 0.13, 0.13),
        Color::from_rgb(0.26, 0.26, 0.26),
        Color::from_rgb(0.38, 0.38, 0.38),
        Color::from_rgb(0.50, 0.50, 0.50),
        Color::from_rgb(0.62, 0.62, 0.62),
        Color::from_rgb(0.74, 0.74, 0.74),
        Color::from_rgb(0.87, 0.87, 0.87),
        Color::WHITE,
    ];

    /// Tailwind CSS colors (subset).
    pub const TAILWIND: &[Color] = &[
        Color::from_rgb(0.937, 0.267, 0.267), // red-500
        Color::from_rgb(0.976, 0.451, 0.086), // orange-500
        Color::from_rgb(0.980, 0.804, 0.082), // yellow-500
        Color::from_rgb(0.133, 0.773, 0.369), // green-500
        Color::from_rgb(0.059, 0.647, 0.914), // blue-500
        Color::from_rgb(0.392, 0.345, 0.839), // indigo-500
        Color::from_rgb(0.545, 0.310, 0.851), // purple-500
        Color::from_rgb(0.925, 0.282, 0.600), // pink-500
    ];
}
