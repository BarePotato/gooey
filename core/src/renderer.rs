use euclid::{Length, Point2D, Rect, Scale, Size2D};
use stylecs::{palette::Srgba, Pixels, Points};

/// Implements drawing APIs.
pub trait Renderer {
    /// The size of the area being drawn.
    fn size(&self) -> Size2D<f32, Points>;
    /// The scale when converting between [`Points`] and [`Pixels`].
    fn scale(&self) -> Scale<f32, Points, Pixels>;

    /// Renders `text` at `baseline_origin` with `options`.
    fn render_text(&self, text: &str, baseline_origin: Point2D<f32, Points>, options: &TextOptions);
    /// Measures `text` using `options`.
    #[must_use]
    fn measure_text(&self, text: &str, options: &TextOptions) -> TextMetrics<Points>;

    /// Strokes the outline of `rect` using `options`.
    fn stroke_rect(&self, rect: &Rect<f32, Points>, options: StrokeOptions);
    /// Fills `rect` using `color`.
    fn fill_rect(&self, rect: &Rect<f32, Points>, color: Srgba);

    /// Draws a line between `point_a` and `point_b` using `options`.
    fn stroke_line(
        &self,
        point_a: Point2D<f32, Points>,
        point_b: Point2D<f32, Points>,
        options: StrokeOptions,
    );
}

/// Text rendering and measurement options.
pub struct TextOptions {
    /// The font family to use.
    pub font_family: Option<String>,
    /// The text size, in [`Points`].
    pub text_size: Length<f32, Points>,
}

impl Default for TextOptions {
    fn default() -> Self {
        Self {
            font_family: None,
            text_size: Length::new(13.),
        }
    }
}

/// Shape outline drawing options.
pub struct StrokeOptions {
    /// The color to stroke.
    pub color: Srgba,
    /// The width of each line segment.
    pub line_width: Length<f32, Points>,
}

/// A measurement of text.
pub struct TextMetrics<U> {
    /// The width of the text.
    pub width: Length<f32, U>,
    /// The height above the baseline.
    pub ascent: Length<f32, U>,
    /// The height below the baseline. Typically a negative number.
    pub descent: Length<f32, U>,
    /// The spacing expected between lines of text.
    pub line_gap: Length<f32, U>,
}

impl<U> TextMetrics<U> {
    /// The height of the rendered text. This is computed by combining
    /// [`ascent`](TextMetrics::ascent) and [`descent`](TextMetrics::descent).
    #[must_use]
    pub fn height(&self) -> Length<f32, U> {
        self.ascent - self.descent
    }

    /// The height of a line of text. This is computed by combining
    /// [`height()`](TextMetrics::height) and
    /// [`line_gap`](TextMetrics::line_gap)
    #[must_use]
    pub fn line_height(&self) -> Length<f32, U> {
        self.height() + self.line_gap
    }
}

impl<U, V> std::ops::Mul<Scale<f32, U, V>> for TextMetrics<U> {
    type Output = TextMetrics<V>;

    fn mul(self, rhs: Scale<f32, U, V>) -> Self::Output {
        TextMetrics {
            width: self.width * rhs,
            ascent: self.ascent * rhs,
            descent: self.descent * rhs,
            line_gap: self.line_gap * rhs,
        }
    }
}

impl<U, V> std::ops::Div<Scale<f32, U, V>> for TextMetrics<V> {
    type Output = TextMetrics<U>;

    fn div(self, rhs: Scale<f32, U, V>) -> Self::Output {
        TextMetrics {
            width: self.width / rhs,
            ascent: self.ascent / rhs,
            descent: self.descent / rhs,
            line_gap: self.line_gap / rhs,
        }
    }
}