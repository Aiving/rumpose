use std::fs;

use material_colors::{color::Argb, theme::ThemeBuilder};
use rumpose::prelude::*;

fn main() {
    let mut recomposer =
        Composer::compose(|scope| app(scope.child()), RumposeContext::new(256, 256));

    {
        let start = std::time::Instant::now();

        recomposer.with_composer_mut(|composer| {
            composer.compute_layout();
            composer.draw_all();
        });

        println!(
            "First iteration of layout and rendering took an {:?}",
            start.elapsed()
        );
    }

    let start = std::time::Instant::now();
    let iterations = 10u8;

    for _ in 0..iterations {
        recomposer.recompose();

        recomposer.with_composer_mut(|composer| {
            composer.compute_layout();
            composer.draw_all();
        });
    }

    println!(
        "Next 10 iterations of layout and rendering took an ~{:?}\n",
        start.elapsed().div_f32(f32::from(iterations))
    );

    recomposer.print_tree_with(recomposer.root_node_key(), |node| node.unwrap().to_string());

    recomposer.with_composer_mut(|composer| {
        fs::write("./image.png", composer.context.encode()).expect("failed to write");
    });
}

#[track_caller]
fn app(scope: Scope) {
    let theme = scope.use_state(|| {
        ThemeBuilder::with_source(Argb::from_u32(0x4FF156))
            .build()
            .schemes
            .dark
    });

    column(
        scope,
        Modifier
            .fill_max_size()
            .background(theme.with(|theme| theme.background.as_color())),
        move |scope| {
            resize_width_column(scope, move |scope| {
                column(scope, Modifier.background(Color::RED), |scope| {
                    text(scope, TextProps::new("Hello"))
                });

                column(
                    scope,
                    Modifier.padding(0., 0., 8., 0.).background(Color::RED),
                    |scope| {
                        text(
                            scope,
                            TextProps::new("This is a long messsage \n and its longer"),
                        );
                    },
                );
            })
        },
    )
}

trait AsColor {
    fn as_color(&self) -> Color;
}

impl AsColor for Argb {
    fn as_color(&self) -> Color {
        Color::from_argb(self.alpha, self.red, self.green, self.blue)
    }
}

fn resize_width_column(scope: Scope, content: impl Fn(Scope) + Clone + 'static) {
    subcompose_layout(
        scope,
        move |_, layout_context, root_constraints, compose_context| {
            let sizes = compose_context.compose(
                1,
                content.clone(),
                layout_context,
                root_constraints,
                |layout_context, _, node| node.measure(layout_context, Constraints::default()),
            );

            let max_size = sizes
                .into_iter()
                .max_by(|x, y| x.partial_cmp(y).unwrap())
                .unwrap();

            let mut constraints = Constraints::default();

            constraints.min.width = max_size.size.width;

            let resized = compose_context.compose(
                1,
                content.clone(),
                layout_context,
                root_constraints,
                |layout_context, id, node| (id, node.measure(layout_context, constraints)),
            );

            let height = resized.iter().map(|v| v.1.size.height).sum();

            for (index, (id, _)) in resized.iter().enumerate() {
                compose_context.place_relative(
                    1,
                    *id,
                    0.,
                    resized.iter().take(index).map(|v| v.1.size.height).sum(),
                );
            }

            Rect2D::new(
                Point2D::default(),
                Size2D::new(constraints.max.width, height),
            )
        },
    );
}
