use std::rc::Rc;

use crate::{
    node::{Draw, Node, NodePhase, RenderPhase}, phase::RenderContext, Scope
};

#[track_caller]
pub fn draw<D: Fn(&RenderContext) + Clone + 'static, C: Fn(Scope) + Clone + 'static>(
    scope: Scope,
    draw: D,
    content: C,
) {
    let child_scope = scope.child();

    scope.create_node(
        child_scope,
        content,
        move || draw.clone(),
        move |draw, _| Node::new(NodePhase::Render(RenderPhase::new(draw))),
        move |node, draw, _| {
            if let NodePhase::Render(phase) = &mut node.phase {
                let draw: Draw = Rc::new(draw);

                if !Rc::ptr_eq(&phase.render, &draw) {
                    phase.render = draw;

                    node.mark_render_dirty();
                }
            }

        },
    );
}
