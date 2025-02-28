use std::{
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use rumpose_layout::{Constraints, Rect2D};

use super::LayoutContext;
use crate::{
    Composer, ComposerExt, Recomposer, RumposeContext, RuntimeNode, Scope,
    node::{Node, NodeExt, NodePhase},
};

#[derive(Clone, Default)]
pub struct SubcomposeContext {
    recomposers: Rc<RefCell<HashMap<usize, Recomposer>>>,
}

impl SubcomposeContext {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    #[track_caller]
    pub fn render(&self) {
        for recomposer in (*self.recomposers.borrow_mut()).values_mut() {
            recomposer.with_composer_mut(|composer| composer.draw_all());
        }
    }

    #[track_caller]
    pub fn print_tree(&self) {
        for recomposer in (*self.recomposers.borrow()).values() {
            recomposer.print_tree();
        }
    }

    #[track_caller]
    pub fn compose<T>(
        &self,
        key: usize,
        content: impl Fn(Scope) + Clone + 'static,
        context: LayoutContext,
        constraints: Constraints,
        mut func: impl FnMut(LayoutContext, usize, &RuntimeNode) -> T,
    ) -> Vec<T> {
        let mut composer = self.recomposers.borrow_mut();

        let have_composer = composer.contains_key(&key);
        let recomposer = composer.entry(key).or_insert_with(move || {
            Composer::compose(
                |scope| {
                    scope.create_node(
                        scope.child(),
                        content.clone(),
                        || {},
                        |_, _| Node::new(NodePhase::Virtual),
                        |_, _, _| {},
                    );
                },
                RumposeContext {
                    root: constraints,
                    surface: RefCell::new(context.surface().clone()),
                    font_manager: context.font_manager().clone(),
                },
            )
        });

        if have_composer {
            recomposer.with_context_mut(|context| context.root = constraints);
            recomposer.recompose();
        }

        recomposer.with_composer_mut(|composer| {
            let root = composer.root_node_key();
            let mut results = Vec::new();

            for value in composer.nodes[root].children.clone() {
                composer.mark_dirty(value);
                
                results.push(func(
                    LayoutContext::new(composer),
                    value,
                    &composer.nodes[value],
                ))
            }

            results
        })
    }

    pub fn place(&self, key: usize, id: usize, x: f32, y: f32) {
        self.recomposers
            .borrow()
            .get(&key)
            .unwrap()
            .with_composer(|composer| {
                let node = &composer.nodes[id];
                let origin = node.get_area().origin;

                for &id in &node.children {
                    let offset = composer.nodes[id].get_area().origin - origin;

                    self.place(key, id, x + offset.x, y + offset.y);
                }

                node.place(x, y);
            })
    }

    pub fn place_relative(&self, key: usize, id: usize, x: f32, y: f32) {
        self.recomposers
            .borrow()
            .get(&key)
            .unwrap()
            .with_composer(|composer| {
                let node = &composer.nodes[id];

                for &id in &node.children {
                    self.place_relative(key, id, x, y);
                }

                node.place_relative(x, y);
            })
    }

    #[must_use]
    pub fn measure(&self, key: usize, id: usize, constraints: Constraints) -> Rect2D {
        self.recomposers
            .borrow()
            .get(&key)
            .unwrap()
            .with_composer(|composer| {
                let node = &composer.nodes[id];

                node.measure(LayoutContext::new(composer), constraints)
            })
    }
}
