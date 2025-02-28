use std::{fmt, rc::Rc};

use rumpose_layout::{Constraints, Size2D};

use crate::{
    RuntimeNode,
    phase::{LayoutContext, Measurable},
};

pub type Measure = Rc<dyn Fn(&RuntimeNode, LayoutContext, Constraints) -> Size2D>;

#[derive(Clone)]
pub struct MeasurementPhase {
    pub measurable: Measure,
}

impl Measurable for MeasurementPhase {
    fn measure(
        &self,
        node: &RuntimeNode,
        scope: LayoutContext,
        constraints: Constraints,
    ) -> Size2D {
        (self.measurable)(node, scope, constraints)
    }
}

impl MeasurementPhase {
    pub fn new<F: Fn(&RuntimeNode, LayoutContext, Constraints) -> Size2D + 'static>(
        measurable: F,
    ) -> Self {
        Self {
            measurable: Rc::new(measurable),
        }
    }
}

impl fmt::Debug for MeasurementPhase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MeasureContainerNode")
            .finish_non_exhaustive()
    }
}
