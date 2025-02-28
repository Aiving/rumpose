use rumpose_core::prelude::*;

use crate::{ModifierElement, modified};

#[track_caller]
pub fn container<C: Fn(Scope) + Clone + 'static>(
    scope: Scope,
    modifier: impl ModifierElement,
    content: C,
) {
    modified(scope, &modifier, move |scope| {
        layout(scope, |_, _, constraints| constraints.min, content.clone())
    });
}

#[track_caller]
pub fn column<C: Fn(Scope) + Clone + 'static>(
    scope: Scope,
    modifier: impl ModifierElement,
    content: C,
) {
    modified(scope, &modifier, move |scope| {
        layout(
            scope,
            |node, context, constraints| {
                let mut size = Size2D::default();

                for &id in &node.children {
                    let children_area = context.measure(id, Constraints::default());

                    context.place_relative(id, 0.0, size.height);

                    size.height += children_area.height;
                    size.width = size.width.max(children_area.width);
                }

                constraints.apply(size)
            },
            content.clone(),
        )
    });
}
