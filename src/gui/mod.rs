use ggez::event::EventHandler;
use ggez::{Context, GameResult};

/// Handles main GUI drawing.
pub struct GUI {}

impl GUI {
    /// Create a new GUI.
    pub fn new() -> Self {
        GUI {}
    }
}

impl EventHandler for GUI {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // TODO: Update code here...
        Ok(())
    }

    fn draw(&mut self, _ctx: &mut Context) -> GameResult<()> {
        // TODO: Draw code here...
        Ok(())
    }
}
