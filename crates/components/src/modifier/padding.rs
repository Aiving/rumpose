use std::rc::Rc;

use rumpose_core::prelude::*;

use super::ModifierElement;

#[derive(Debug, Clone, Copy)]
enum FillDirection {
    Both,
    Horizontal,
    Vertical,
}

impl FillDirection {
    /// Returns `true` if the fill direction is [`Horizontal`].
    ///
    /// [`Horizontal`]: FillDirection::Horizontal
    #[must_use]
    const fn is_horizontal(self) -> bool {
        matches!(self, Self::Horizontal)
    }

    /// Returns `true` if the fill direction is [`Vertical`].
    ///
    /// [`Vertical`]: FillDirection::Vertical
    #[must_use]
    const fn is_vertical(self) -> bool {
        matches!(self, Self::Vertical)
    }
}

#[derive(Debug, Clone)]
pub struct FillModifier {
    direction: FillDirection,
    fraction: f32,
}

impl ModifierElement for FillModifier {
    #[track_caller]
    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>) {
        let &Self {
            direction,
            fraction,
        } = self;

        layout(
            scope,
            move |node, context, constraints| {
                let [min_width, max_width]: [f32; 2] =
                    if constraints.has_bounded_width() && !direction.is_vertical() {
                        let width = constraints.apply_width(constraints.max.width * fraction);

                        [width, width]
                    } else {
                        [constraints.min.width, constraints.max.width]
                    };

                let [min_height, max_height]: [f32; 2] =
                    if constraints.has_bounded_height() && !direction.is_horizontal() {
                        let height = constraints.apply_height(constraints.max.height * fraction);

                        [height, height]
                    } else {
                        [constraints.min.height, constraints.max.height]
                    };

                context.measure(
                    node.children[0],
                    Constraints::new(min_width, max_width, min_height, max_height),
                )
            },
            move |scope| content(scope),
        );
    }
}

#[derive(Debug, Clone)]
pub struct PaddingModifier {
    start: f32,
    end: f32,
    top: f32,
    bottom: f32,
}

impl ModifierElement for PaddingModifier {
    #[track_caller]
    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>) {
        let &Self {
            start,
            end,
            top,
            bottom,
        } = self;
        let horizontal = start + end;
        let vertical = top + bottom;

        layout(
            scope,
            move |node, context, constraints| {
                let mut area =
                    context.measure(node.children[0], constraints.offset(-horizontal, -vertical));

                context.place_relative(node.children[0], start, top);

                area = Size2D::new(
                    constraints.apply_width(area.width + horizontal),
                    constraints.apply_height(area.height + vertical),
                );

                area
            },
            move |scope: Scope| content(scope),
        );
    }
}

pub trait LayoutModifierExt {
    fn fill_max_size(self) -> impl ModifierElement;
    fn fill_max_width(self) -> impl ModifierElement;
    fn fill_max_height(self) -> impl ModifierElement;
    fn padding_all(self, value: f32) -> impl ModifierElement;
    fn padding(self, start: f32, end: f32, top: f32, bottom: f32) -> impl ModifierElement;
}

impl<T: ModifierElement> LayoutModifierExt for T {
    #[track_caller]
    fn padding_all(self, value: f32) -> impl ModifierElement {
        self.then(PaddingModifier {
            start: value,
            end: value,
            top: value,
            bottom: value,
        })
    }

    #[track_caller]
    fn padding(self, start: f32, end: f32, top: f32, bottom: f32) -> impl ModifierElement {
        self.then(PaddingModifier {
            start,
            end,
            top,
            bottom,
        })
    }

    #[track_caller]
    fn fill_max_size(self) -> impl ModifierElement {
        self.then(FillModifier {
            direction: FillDirection::Both,
            fraction: 1.,
        })
    }

    #[track_caller]
    fn fill_max_width(self) -> impl ModifierElement {
        self.then(FillModifier {
            direction: FillDirection::Horizontal,
            fraction: 1.,
        })
    }

    #[track_caller]
    fn fill_max_height(self) -> impl ModifierElement {
        self.then(FillModifier {
            direction: FillDirection::Vertical,
            fraction: 1.,
        })
    }
}
