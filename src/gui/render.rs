use super::*;

use ggez::graphics::Rect;

/// Area which the rendering should take place in.
pub type RenderArea = Rect;
pub type DrawContext = Context;

/// Context used for rendering.
pub struct RenderContext<'a> {
    game_state: &'a World,
    draw_state: &'a mut Context,
}

impl<'a> RenderContext<'a> {
    /// Creates a new context.
    pub fn new(game_state: &'a World, draw_state: &'a mut DrawContext) -> Self {
        RenderContext {
            game_state,
            draw_state,
        }
    }

    /// Retrives the game state.
    pub fn game_state(&self) -> &World {
        self.game_state
    }

    /// Retrives the drawing context.
    pub fn draw_state(&mut self) -> &mut DrawContext {
        self.draw_state
    }
}
