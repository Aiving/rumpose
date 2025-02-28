mod measure;
mod measure_compose;
mod render;

use std::{
    cell::{Cell, RefCell},
    fmt,
};

use rumpose_layout::{Constraints, Point2D, Rect2D, Size2D};
use rumpose_runtime::ComposeNode;

pub use self::{
    measure::{Measure, MeasurementPhase},
    measure_compose::{MeasureCompose, MeasurementComposePhase},
    render::{Draw, RenderPhase},
};
use crate::{
    RumposeContext, RuntimeNode,
    phase::{LayoutContext, Measurable, RenderContext},
};

#[derive(Debug)]
pub enum NodePhase {
    Virtual, // Does nothing itself
    Measurement(MeasurementPhase),
    MeasurementCompose(MeasurementComposePhase),
    Render(RenderPhase),
}

#[derive(Debug)]
pub struct Node {
    pub area: RefCell<Rect2D>,
    pub phase: NodePhase,
    pub layout_dirty: Cell<bool>,
    pub render_dirty: Cell<bool>,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let area = self.area.borrow();

        write!(
            f,
            "Node({}) {}x{} at {}x{}",
            match &self.phase {
                NodePhase::Virtual => "virtual",
                NodePhase::Measurement(_) => "layout",
                NodePhase::MeasurementCompose(_) => "layout + subcompose",
                NodePhase::Render(_) => "render",
            },
            area.size.width,
            area.size.height,
            area.origin.x,
            area.origin.y
        )
    }
}

impl Node {
    pub fn new(phase: NodePhase) -> Self {
        Self {
            area: RefCell::default(),
            phase,
            layout_dirty: true.into(),
            render_dirty: true.into(),
        }
    }

    pub fn render(&self, context: &RenderContext) {
        if self.render_dirty.get() {
            if let NodePhase::Render(node) = &self.phase {
                (node.render)(context);
            }

            if let NodePhase::MeasurementCompose(node) = &self.phase {
                node.context.render();
            } else if context.does_content_draw() {
                context.draw_content();
            }

            self.render_dirty.set(false);
        }
    }

    pub fn mark_dirty(&self) {
        self.layout_dirty.set(true);
        self.render_dirty.set(true);
    }

    pub fn mark_render_dirty(&self) {
        self.render_dirty.set(true);
    }
}

impl ComposeNode for Node {
    type Context = RumposeContext;
}

impl Measurable for Node {
    fn measure(
        &self,
        node: &RuntimeNode,
        context: LayoutContext,
        constraints: Constraints,
    ) -> Size2D {
        if self.layout_dirty.get() {
            let size = match &self.phase {
                NodePhase::Virtual => Size2D::default(),
                NodePhase::Render(_) => context.measure(node.children[0], constraints),
                NodePhase::MeasurementCompose(measure_node) => {
                    measure_node.measure(node, context, constraints)
                }
                NodePhase::Measurement(measure_node) => {
                    measure_node.measure(node, context, constraints)
                }
            };

            self.area.borrow_mut().size = size;
            self.layout_dirty.set(false);

            size
        } else {
            self.area.borrow().size
        }
    }
}

pub trait NodeExt {
    fn get_area(&self) -> Rect2D;

    fn draw(&self, context: RenderContext);
    fn measure(&self, context: LayoutContext, constraints: Constraints) -> Size2D;

    fn place(&self, x: f32, y: f32);
    fn place_relative(&self, x: f32, y: f32);
}

impl NodeExt for crate::RuntimeNode {
    fn get_area(&self) -> Rect2D {
        self.data
            .as_ref()
            .map(|data| *data.area.borrow())
            .unwrap_or_default()
    }

    fn draw(&self, context: RenderContext) {
        self.data.as_ref().inspect(|value| value.render(&context));
    }

    fn measure(&self, context: LayoutContext, constraints: Constraints) -> Size2D {
        self.data
            .as_ref()
            .map(|value| value.measure(self, context, constraints))
            .unwrap_or_default()
    }

    fn place(&self, x: f32, y: f32) {
        let node = self.data.as_ref().unwrap();

        node.area.borrow_mut().origin = Point2D::new(x, y);
    }

    fn place_relative(&self, x: f32, y: f32) {
        let node = self.data.as_ref().unwrap();

        node.area.borrow_mut().origin += Point2D::new(x, y);
    }
}
