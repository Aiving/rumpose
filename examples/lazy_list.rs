use std::fs;

use material_colors::{color::Argb, theme::ThemeBuilder};
use rumpose::prelude::*;

trait AsColor {
    fn as_color(&self) -> Color;
}

impl AsColor for Argb {
    fn as_color(&self) -> Color {
        Color::from_argb(self.alpha, self.red, self.green, self.blue)
    }
}

fn main() {
    let mut recomposer =
        Composer::compose(|scope| app(scope.child()), RumposeContext::new(256, 1024));

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
        })
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

fn app(scope: Scope) {
    lazy_column(scope, 1000, move |scope, index| {
        let theme = scope.use_state(|| {
            ThemeBuilder::with_source(Argb::from_u32(0x4FF156))
                .build()
                .schemes
                .dark
        });

        column(
            scope,
            Modifier.background(theme.with(|theme| theme.primary.as_color())),
            move |scope| {
                text(
                    scope,
                    TextProps::new(format!("Hello {index}"))
                        .color(theme.with(|theme| theme.on_primary.as_color())),
                )
            },
        );
    })
}

fn lazy_column(
    scope: Scope,
    items_count: usize,
    item_builder: impl Fn(Scope, usize) + Clone + 'static,
) {
    subcompose_layout(
        scope,
        move |_, layout_context, root_constraints, compose_context| {
            let mut height = 0.;
            let mut width = root_constraints.min.width;
            let mut composed_items = 0u16;

            for item in 0..items_count {
                let item_builder = item_builder.clone();

                let areas = compose_context.compose(
                    item,
                    move |scope| item_builder(scope, item),
                    layout_context,
                    root_constraints,
                    |layout_context, id, node| {
                        (id, node.measure(layout_context, Constraints::default()))
                    },
                );

                for (id, area) in areas {
                    compose_context.place_relative(item, id, 0.0, height);

                    width = width.max(area.width);
                    height += area.height;
                }

                composed_items += 1;

                if height >= root_constraints.max.height {
                    println!("Composed {composed_items} items");

                    break;
                }
            }

            println!(
                "Column height: composed = {height}px, real = {}px\n",
                (height / (f32::from(composed_items))) * (items_count as f32)
            );

            root_constraints.apply(Size2D::new(width, height))
        },
    );
}
