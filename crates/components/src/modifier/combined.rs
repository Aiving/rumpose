use std::rc::Rc;

use rumpose_core::prelude::*;

use super::ModifierElement;

#[derive(Debug, Clone)]
pub struct CombinedModifier<L: ModifierElement, R: ModifierElement>(Rc<L>, Rc<R>);

impl<L: ModifierElement, R: ModifierElement> CombinedModifier<L, R> {
    pub fn new(l: L, r: R) -> Self {
        Self(Rc::new(l), Rc::new(r))
    }
}

impl<L: ModifierElement, R: ModifierElement + 'static> ModifierElement for CombinedModifier<L, R> {
    #[track_caller]
    fn apply(&self, scope: Scope, content: Rc<dyn Fn(Scope) + 'static>) {
        let (this, other) = (self.0.clone(), self.1.clone());

        this.apply(
            scope,
            Rc::new(move |scope| {
                other.apply(scope, content.clone());
            }),
        );
    }
}
