use std::{fmt, rc::Rc};

use rumpose_layout::{Constraints, Size2D};

use crate::{
    RuntimeNode,
    phase::{LayoutContext, Measurable, SubcomposeContext},
};

pub type MeasureCompose =
    Rc<dyn Fn(&RuntimeNode, LayoutContext, Constraints, &SubcomposeContext) -> Size2D>;

#[derive(Clone)]
pub struct MeasurementComposePhase {
    pub measurable: MeasureCompose,
    pub(crate) context: SubcomposeContext,
}

impl Measurable for MeasurementComposePhase {
    #[track_caller]
    fn measure(
        &self,
        node: &RuntimeNode,
        context: LayoutContext,
        constraints: Constraints,
    ) -> Size2D {
        (self.measurable)(node, context, constraints, &self.context)
    }
}

impl MeasurementComposePhase {
    pub fn new<
        F: Fn(&RuntimeNode, LayoutContext, Constraints, &SubcomposeContext) -> Size2D + 'static,
    >(
        measurable: F,
    ) -> Self {
        Self {
            measurable: Rc::new(measurable),
            context: SubcomposeContext::new(),
        }
    }
}

impl fmt::Debug for MeasurementComposePhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeasureContainerNode")
            .finish_non_exhaustive()
    }
}
