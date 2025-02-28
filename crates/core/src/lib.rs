use std::cell::RefCell;

use node::NodeExt;
use phase::{LayoutContext, RenderContext};
use rumpose_engine::{
    EncodedImageFormat, FontCollection, FontManager, Image, Surface, create_surface,
};
use rumpose_layout::Constraints;

mod components;
mod node;
mod phase;

pub mod prelude {
    pub use rumpose_engine::*;
    pub use rumpose_layout::*;

    pub use crate::{
        Composer, ComposerExt, Recomposer, RumposeContext, RuntimeNode, Scope, components::*,
        node::*, phase::*,
    };
}

pub type RuntimeNode = rumpose_runtime::Node<node::Node>;
pub type Composer = rumpose_runtime::Composer<node::Node>;
pub type Recomposer = rumpose_runtime::Recomposer<(), node::Node>;
pub type Scope = rumpose_runtime::Scope<(), node::Node>;
pub type State<T> = rumpose_runtime::State<T, node::Node>;

pub struct RumposeContext {
    root: Constraints,
    surface: RefCell<Surface>,
    font_manager: FontCollection,
}

impl RumposeContext {
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            root: Constraints::new(0., width as f32, 0., height as f32),
            surface: RefCell::new(create_surface((i32::from(width), i32::from(height))).unwrap()),
            font_manager: {
                let mut collection = FontCollection::new();

                collection.set_default_font_manager(FontManager::new(), "Arial");

                collection
            },
        }
    }

    pub fn snapshot(&self) -> Image {
        self.surface.borrow_mut().image_snapshot()
    }

    pub fn encode(&self) -> Vec<u8> {
        self.snapshot()
            .encode(None, EncodedImageFormat::PNG, 100)
            .map(|data| data.to_vec())
            .unwrap()
    }
}

pub trait ComposerExt {
    fn compute_layout(&self);
    fn draw_all(&mut self);
    fn mark_dirty(&self, id: usize);
}

impl ComposerExt for Composer {
    #[track_caller]
    fn mark_dirty(&self, id: usize) {
        let node = &self.nodes[id];

        node.data.as_ref().inspect(|data| data.mark_dirty());

        for &id in &node.children {
            self.mark_dirty(id);
        }
    }

    #[track_caller]
    fn compute_layout(&self) {
        let root_node = self.root_node_key();
        let node = &self.nodes[root_node];

        node.measure(LayoutContext::new(self), self.context.root);
    }

    #[track_caller]
    fn draw_all(&mut self) {
        let root_node = self.root_node_key();

        let node = &self.nodes[root_node];

        node.draw(RenderContext::new(node, self));
    }
}
