use std::rc::Rc;

use rumpose_core::prelude::*;

#[derive(Default, Clone)]
pub struct TextProps {
    content: Rc<String>,
    color: Option<Color>,
    font_size: Option<f32>,
    // font_style: FontStyle,
    // font_weight: Option<FontWeight>,
    // font_family: FontFamily,
    letter_spacing: Option<f32>,
    // text_decoration: TextDecoration,
    // text_align: TextAlign,
    line_height: Option<f32>,
    // overflow: TextOverflow,
    // soft_wrap: bool,
    // max_lines: usize,
    // min_lines: usize,
}

impl TextProps {
    #[must_use]
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: Rc::new(content.into()),
            ..Default::default()
        }
    }

    #[must_use]
    pub const fn color(mut self, color: Color) -> Self {
        self.color = Some(color);

        self
    }

    #[must_use]
    pub const fn font_size(mut self, value: f32) -> Self {
        self.font_size = Some(value);

        self
    }

    #[must_use]
    pub const fn letter_spacing(mut self, value: f32) -> Self {
        self.letter_spacing = Some(value);

        self
    }

    #[must_use]
    pub const fn line_height(mut self, value: f32) -> Self {
        self.line_height = Some(value);

        self
    }
}

#[track_caller]
#[allow(clippy::missing_panics_doc)]
pub fn text(scope: Scope, props: TextProps) {
    let content = props.content.clone();

    draw(
        scope,
        move |context| {
            let area = context.area();

            let mut style = TextStyle::new();

            if let Some(color) = props.color {
                style.set_color(color);
            }

            if let Some(value) = props.line_height {
                style.set_height_override(true).set_height(value);
            }

            if let Some(value) = props.letter_spacing {
                style.set_letter_spacing(value);
            }

            if let Some(value) = props.font_size {
                style.set_font_size(value);
            }

            let mut paragraph_style = ParagraphStyle::new();

            paragraph_style.set_text_style(&style);

            let mut paragraph = ParagraphBuilder::new(&paragraph_style, context.font_manager())
                .add_text(content.as_str())
                .build();

            paragraph.layout(context.area().size.width + 1.);

            paragraph.paint(
                context.surface().canvas(),
                Point::new(area.origin.x, area.origin.y),
            );
        },
        move |scope| {
            let content = props.content.clone();

            layout(
                scope,
                move |_, context, constraints| {
                    let mut style = TextStyle::new();

                    if let Some(value) = props.line_height {
                        style.set_height_override(true).set_height(value);
                    }

                    if let Some(value) = props.letter_spacing {
                        style.set_letter_spacing(value);
                    }

                    if let Some(value) = props.font_size {
                        style.set_font_size(value);
                    }

                    let mut paragraph_style = ParagraphStyle::new();

                    paragraph_style.set_text_style(&style);

                    let mut paragraph =
                        ParagraphBuilder::new(&paragraph_style, context.font_manager())
                            .add_text(content.as_str())
                            .build();

                    paragraph.layout(constraints.max.width);

                    constraints.apply(Size2D::new(paragraph.longest_line(), paragraph.height()))
                },
                |_| {},
            );
        },
    );
}
