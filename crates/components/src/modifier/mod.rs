use std::{any::Any, fmt::Debug, rc::Rc};

use rumpose_core::prelude::*;

mod combined;
mod content;
mod draw;
mod padding;

pub use self::{combined::CombinedModifier, content::ContentModifierExt, draw::*, padding::*};

pub trait ModifierElement: Any + Debug {
    #[track_caller]
    fn then<T: ModifierElement + 'static>(self, other: T) -> impl ModifierElement
    where
        Self: Sized,
    {
        CombinedModifier::new(self, other)
    }

    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>);
}

#[derive(Debug, Clone)]
pub struct Modifier;

impl ModifierElement for Modifier {
    #[track_caller]
    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>) {
        content(scope);
    }
}

#[track_caller]
pub fn modified<M: ModifierElement, C: Fn(Scope) + Clone + 'static>(
    scope: Scope,
    modifier: &M,
    content: C,
) {
    modifier.apply(scope, Rc::new(content));
}
