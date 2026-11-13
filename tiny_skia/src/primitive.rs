use crate::core::Rectangle;

#[derive(Debug, Clone, PartialEq)]
pub enum Primitive {
    /// A path filled with some paint.
    Fill {
        /// The path to fill.
        path: tiny_skia::Path,
        /// The paint to use.
        paint: tiny_skia::Paint<'static>,
        /// The fill rule to follow.
        rule: tiny_skia::FillRule,
        /// Logical canvas clip retained across frame merges.
        clip_bounds: Rectangle,
    },
    /// A path stroked with some paint.
    Stroke {
        /// The path to stroke.
        path: tiny_skia::Path,
        /// The paint to use.
        paint: tiny_skia::Paint<'static>,
        /// The stroke settings.
        stroke: tiny_skia::Stroke,
        /// Logical canvas clip retained across frame merges.
        clip_bounds: Rectangle,
    },
}

impl Primitive {
    /// Returns the visible bounds of the [`Primitive`].
    pub fn visible_bounds(&self) -> Rectangle {
        let (bounds, clip) = match self {
            Primitive::Fill {
                path, clip_bounds, ..
            } => {
                let b = path.bounds();
                (
                    Rectangle {
                        x: b.x(),
                        y: b.y(),
                        width: b.width(),
                        height: b.height(),
                    },
                    clip_bounds,
                )
            }
            Primitive::Stroke {
                path,
                stroke,
                clip_bounds,
                ..
            } => {
                let b = path.bounds();
                let padding = stroke.width / 2.0;
                (
                    Rectangle {
                        x: b.x() - padding,
                        y: b.y() - padding,
                        width: b.width() + 2.0 * padding,
                        height: b.height() + 2.0 * padding,
                    },
                    clip_bounds,
                )
            }
        };
        bounds.intersection(clip).unwrap_or_default()
    }
    pub(crate) fn clip_to(&mut self, bounds: Rectangle) {
        let (Self::Fill { clip_bounds, .. } | Self::Stroke { clip_bounds, .. }) = self;
        *clip_bounds = clip_bounds.intersection(&bounds).unwrap_or_default();
    }
}
