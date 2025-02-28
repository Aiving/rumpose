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
    let theme = scope.use_state(|| {
        ThemeBuilder::with_source(Argb::from_u32(0x4FF156))
            .build()
            .schemes
            .dark
    });

    column(
        scope,
        Modifier
            .background(theme.with(|value| value.surface.as_color()))
            .padding_all(8.0)
            .background(theme.with(|value| value.surface_container.as_color()))
            .padding(48., 24., 12., 18.)
            .border(
                1.0,
                theme.with(|value| value.outline.as_color()),
                RoundedShape::new_all(12.0),
            )
            .clip(RoundedShape::new_all(12.0))
            .background(theme.with(|value| value.surface_container_highest.as_color()))
            .padding_all(50.0)
            .fill_max_size(),
        move |scope| {
            text(
                scope,
                TextProps::new("helloo").color(theme.with(|value| value.primary.as_color())),
            );
            text(
                scope,
                TextProps::new("foo").color(theme.with(|value| value.secondary.as_color())),
            );
            text(
                scope,
                TextProps::new("baar").color(theme.with(|value| value.tertiary.as_color())),
            );
            text(
                scope,
                TextProps::new("world").color(theme.with(|value| value.on_surface.as_color())),
            );
        },
    );
}

trait AsColor {
    fn as_color(&self) -> Color;
}

impl AsColor for Argb {
    fn as_color(&self) -> Color {
        Color::from_argb(self.alpha, self.red, self.green, self.blue)
    }
}
