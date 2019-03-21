use super::*;

/// A renderable component.
pub trait Component {
    fn render(&self, area: &mut RenderArea, ctx: &mut RenderContext);
}
