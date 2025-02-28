use std::cell::{Cell, RefMut};

use rumpose_engine::{FontCollection, Surface};
use rumpose_layout::Rect2D;

use crate::{Composer, RuntimeNode, node::NodeExt};

#[derive(Clone)]
pub struct RenderContext<'a> {
    node: &'a RuntimeNode,
    composer: &'a Composer,
    draw_content: Cell<bool>,
}

impl<'a> RenderContext<'a> {
    pub(crate) fn new(node: &'a RuntimeNode, composer: &'a Composer) -> Self {
        Self {
            node,
            composer,
            draw_content: Cell::new(true),
        }
    }

    #[must_use]
    pub fn area(&self) -> Rect2D {
        self.node.get_area()
    }

    #[must_use]
    pub const fn font_manager(&self) -> &FontCollection {
        &self.composer.context.font_manager
    }

    #[must_use]
    pub fn surface(&self) -> RefMut<Surface> {
        self.composer.context.surface.borrow_mut()
    }

    pub fn set_do_content_draw(&self, value: bool) {
        self.draw_content.set(value);
    }

    pub fn does_content_draw(&self) -> bool {
        self.draw_content.get()
    }

    pub fn draw_content(&self) {
        for &node in &self.node.children {
            let node = &self.composer.nodes[node];

            node.draw(RenderContext {
                node,
                composer: self.composer,
                draw_content: Cell::new(true),
            });
        }
    }
}
