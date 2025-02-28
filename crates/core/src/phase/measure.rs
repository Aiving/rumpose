use std::cell::Ref;

use rumpose_engine::{FontCollection, Surface};
use rumpose_layout::{Constraints, Rect2D};

use crate::{Composer, ComposerExt, RuntimeNode, node::NodeExt};

#[derive(Clone, Copy)]
pub struct LayoutContext<'a> {
    composer: &'a Composer,
}

impl<'a> LayoutContext<'a> {
    pub(crate) const fn new(composer: &'a Composer) -> Self {
        Self { composer }
    }

    pub(crate) fn surface(&self) -> Ref<Surface> {
        self.composer.context.surface.borrow()
    }

    #[must_use]
    pub const fn font_manager(&self) -> &FontCollection {
        &self.composer.context.font_manager
    }

    pub fn mark_dirty(&self, id: usize) {
        self.composer.mark_dirty(id);
    }

    pub fn place(&self, id: usize, x: f32, y: f32) {
        let node = &self.composer.nodes[id];
        let origin = node.get_area().origin;

        for &id in &node.children {
            let offset = self.composer.nodes[id].get_area().origin - origin;

            self.place(id, x + offset.x, y + offset.y);
        }

        node.place(x, y);
    }

    pub fn place_relative(&self, id: usize, x: f32, y: f32) {
        let node = &self.composer.nodes[id];

        for &id in &node.children {
            self.place_relative(id, x, y);
        }

        node.place_relative(x, y);
    }

    #[must_use]
    pub fn measure(&self, id: usize, constraints: Constraints) -> Rect2D {
        let node = &self.composer.nodes[id];

        node.measure(*self, constraints)
    }
}

pub trait Measurable {
    fn measure(
        &self,
        node: &RuntimeNode,
        context: LayoutContext,
        constraints: Constraints,
    ) -> Rect2D;
}
