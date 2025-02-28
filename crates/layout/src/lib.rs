use rumpose_geometry::{Point, Rect, Size};

pub type Point2D = Point<f32>;
pub type Size2D = Size<f32>;
pub type Rect2D = Rect<f32>;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Constraints {
    pub min: Size2D,
    pub max: Size2D,
}

impl Constraints {
    #[must_use]
    pub const fn new(min_width: f32, max_width: f32, min_height: f32, max_height: f32) -> Self {
        Self {
            min: Size2D::new(min_width.max(0.), min_height.max(0.)),
            max: Size2D::new(max_width.max(0.), max_height.max(0.)),
        }
    }

    #[must_use]
    pub const fn apply_width(&self, value: f32) -> f32 {
        value.clamp(self.min.width, self.max.width)
    }

    #[must_use]
    pub const fn apply_height(&self, value: f32) -> f32 {
        value.clamp(self.min.height, self.max.height)
    }

    #[must_use]
    pub const fn apply(&self, value: Size2D) -> Size2D {
        Size2D::new(
            self.apply_width(value.width),
            self.apply_height(value.height),
        )
    }

    #[must_use]
    pub fn offset(&self, horizontal: f32, vertical: f32) -> Self {
        Self::new(
            (self.min.width + horizontal).max(0.),
            if self.max.width == f32::INFINITY {
                self.max.width
            } else {
                self.max.width + horizontal
            },
            (self.min.height + vertical).max(0.),
            if self.max.height == f32::INFINITY {
                self.max.height
            } else {
                self.max.height + vertical
            },
        )
    }

    #[must_use]
    pub const fn has_bounded_width(&self) -> bool {
        self.max.width != f32::INFINITY
    }

    #[must_use]
    pub const fn has_bounded_height(&self) -> bool {
        self.max.height != f32::INFINITY
    }
}

impl Default for Constraints {
    fn default() -> Self {
        Self::new(0., f32::INFINITY, 0., f32::INFINITY)
    }
}
