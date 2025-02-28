use std::{fmt::Debug, rc::Rc};

use rumpose_core::prelude::*;

use super::ModifierElement;

#[derive(Clone)]
pub struct DrawModifier {
    draw: Rc<dyn Fn(&RenderContext)>,
}

impl Debug for DrawModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DrawModifier")
            .field("draw", &Rc::as_ptr(&self.draw))
            .finish()
    }
}

impl ModifierElement for DrawModifier {
    #[track_caller]
    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>) {
        let draw = self.draw.clone();

        rumpose_core::prelude::draw(
            scope,
            move |context| draw(context),
            move |scope| content(scope),
        );
    }
}

#[derive(Debug, Clone)]
pub struct ClipModifier(Rc<dyn Shape>);

impl ModifierElement for ClipModifier {
    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>) {
        let shape = self.0.clone();

        rumpose_core::prelude::draw(
            scope,
            move |context| {
                context.set_do_content_draw(false);

                let path = shape.create(context.area());

                {
                    let mut surface = context.surface();
                    let canvas = surface.canvas();

                    canvas.save();
                    canvas.clip_path(&path, Some(ClipOp::Intersect), Some(true));
                }

                context.draw_content();

                context.surface().canvas().restore();
            },
            move |scope| content(scope),
        );
    }
}

pub trait Shape: Debug {
    fn create(&self, area: Rect2D) -> Path;
}

#[derive(Debug)]
pub struct CircleShape;

impl Shape for CircleShape {
    fn create(&self, area: Rect2D) -> Path {
        Path::circle(
            area.origin + area.size.center(),
            area.size.width.min(area.size.height) / 2.,
            None,
        )
    }
}

#[derive(Debug)]
pub struct RoundedShape {
    top_start: f32,
    top_end: f32,
    bottom_start: f32,
    bottom_end: f32,
}

impl RoundedShape {
    #[must_use]
    pub const fn new(top_start: f32, top_end: f32, bottom_start: f32, bottom_end: f32) -> Self {
        Self {
            top_start,
            top_end,
            bottom_start,
            bottom_end,
        }
    }

    #[must_use]
    pub const fn new_all(value: f32) -> Self {
        Self {
            top_start: value,
            top_end: value,
            bottom_start: value,
            bottom_end: value,
        }
    }
}

impl Shape for RoundedShape {
    fn create(&self, area: Rect2D) -> Path {
        let &Self {
            top_start,
            top_end,
            bottom_start,
            bottom_end,
        } = self;

        if top_start + top_end + bottom_start + bottom_end == 0.0 {
            Path::rect(Rect::from(area), None)
        } else {
            Path::rrect(
                RRect::new_rect_radii(Rect::from(area), &[
                    Point::new(self.top_start, self.top_start),
                    Point::new(self.top_end, self.top_end),
                    Point::new(self.bottom_start, self.bottom_start),
                    Point::new(self.bottom_end, self.bottom_end),
                ]),
                None,
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct BorderModifier(f32, Color, Rc<dyn Shape>);

impl ModifierElement for BorderModifier {
    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>) {
        let Self(width, color, shape) = self.clone();

        rumpose_core::prelude::draw(
            scope,
            move |context| {
                let path = shape.create({
                    let mut area = context.area();

                    area.origin += width / 2.;
                    area.size -= width;

                    area
                });

                let mut paint = Paint::default();

                paint.set_anti_alias(true);
                paint.set_color(color);
                paint.set_stroke(true);
                paint.set_stroke_width(width);

                context.set_do_content_draw(false);
                context.draw_content();
                context.surface().canvas().draw_path(&path, &paint);
            },
            move |scope| {
                let content = content.clone();

                layout(
                    scope,
                    move |node, context, constraints| {
                        let mut size = context.measure(
                            node.children[0],
                            constraints.offset(-(width * 2.), -(width * 2.)),
                        );

                        context.place_relative(node.children[0], width, width);

                        size += width * 2.;

                        size
                    },
                    move |scope| content(scope),
                );
            },
        );
    }
}

pub trait DrawModifierExt {
    fn draw<F: Fn(&RenderContext) + 'static>(self, func: F) -> impl ModifierElement;
    fn clip<S: Shape + 'static>(self, shape: S) -> impl ModifierElement;
    fn border<S: Shape + 'static>(self, width: f32, color: Color, shape: S)
    -> impl ModifierElement;
}

impl<T: ModifierElement> DrawModifierExt for T {
    #[track_caller]
    fn draw<F: Fn(&RenderContext) + 'static>(self, draw: F) -> impl ModifierElement {
        self.then(DrawModifier {
            draw: Rc::new(draw),
        })
    }

    #[track_caller]
    fn clip<S: Shape + 'static>(self, shape: S) -> impl ModifierElement {
        self.then(ClipModifier(Rc::new(shape)))
    }

    #[track_caller]
    fn border<S: Shape + 'static>(
        self,
        width: f32,
        color: Color,
        shape: S,
    ) -> impl ModifierElement {
        self.then(BorderModifier(width, color, Rc::new(shape)))
    }
}
