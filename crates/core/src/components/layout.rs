use std::rc::Rc;

use rumpose_layout::{Constraints, Size2D};

use crate::{
    node::{Measure, MeasureCompose, MeasurementComposePhase, MeasurementPhase, Node, NodePhase}, phase::{LayoutContext, SubcomposeContext}, RuntimeNode, Scope
};

#[track_caller]
pub fn subcompose_layout<
    M: Fn(&RuntimeNode, LayoutContext, Constraints, &SubcomposeContext) -> Size2D + Clone + 'static,
>(
    scope: Scope,
    measure: M,
) {
    let child_scope = scope.child::<()>();

    scope.create_node(
        child_scope,
        |_| {},
        move || measure.clone(),
        move |measure, _| {
            Node::new(NodePhase::MeasurementCompose(MeasurementComposePhase::new(
                measure,
            )))
        },
        move |node, measure, _| {
            if let NodePhase::MeasurementCompose(phase) = &mut node.phase {
                let measure: MeasureCompose = Rc::new(measure);

                if !Rc::ptr_eq(&phase.measurable, &measure) {
                    phase.measurable = measure;

                    node.mark_dirty();
                }
            }
        },
    );
}

#[track_caller]
pub fn layout<
    M: Fn(&RuntimeNode, LayoutContext, Constraints) -> Size2D + Clone + 'static,
    C: Fn(Scope) + Clone + 'static,
>(
    scope: Scope,
    measure: M,
    content: C,
) {
    let child_scope = scope.child();

    scope.create_node(
        child_scope,
        content,
        move || measure.clone(),
        move |measure, _| Node::new(NodePhase::Measurement(MeasurementPhase::new(measure))),
        move |node, measure, _| {
            if let NodePhase::Measurement(phase) = &mut node.phase {
                let measure: Measure = Rc::new(measure);

                if !Rc::ptr_eq(&phase.measurable, &measure) {
                    phase.measurable = measure;

                    node.mark_dirty();
                }
            }
        },
    );
}
