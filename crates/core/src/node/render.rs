use std::{fmt, rc::Rc};

use rumpose_layout::{Constraints, Size2D};

use super::NodeExt;
use crate::{
    RuntimeNode,
    phase::{LayoutContext, Measurable, RenderContext},
};

pub type Draw = Rc<dyn Fn(&RenderContext)>;

#[derive(Clone)]
pub struct RenderPhase {
    pub(crate) render: Draw,
}

impl RenderPhase {
    pub fn new<F: Fn(&RenderContext) + 'static>(render: F) -> Self {
        Self {
            render: Rc::new(render),
        }
    }
}

impl Measurable for RenderPhase {
    fn measure(
        &self,
        node: &RuntimeNode,
        scope: LayoutContext,
        constraints: Constraints,
    ) -> Size2D {
        node.measure(scope, constraints)
    }
}

impl fmt::Debug for RenderPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DrawNode").finish_non_exhaustive()
    }
}
