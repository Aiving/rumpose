use rumpose_core::prelude::*;

use super::{DrawModifierExt, ModifierElement};

pub trait ContentModifierExt {
    fn background(self, color: Color) -> impl ModifierElement;
}

impl<T: ModifierElement> ContentModifierExt for T {
    #[track_caller]
    fn background(self, color: Color) -> impl ModifierElement {
        self.draw(move |context| {
            let area = context.area();

            let mut paint = Paint::default();

            paint.set_anti_alias(true);
            paint.set_color(color);

            context
                .surface()
                .canvas()
                .draw_rect(Rect::from(area), &paint);
        })
    }
}
