use ggez::{event::*, *};
pub use log::{debug, info, warn};

use crate::world::World;
mod component;
mod render;
mod view;

use self::render::{RenderArea, RenderContext};
use self::view::StateMachine;

/// UI user interaction events.
pub enum Event {
    MouseDown {
        button: MouseButton,
        x: i32,
        y: i32,
    },
    MouseUp {
        button: MouseButton,
        x: i32,
        y: i32,
    },
    MouseMove {
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    },
    MouseWheel {
        x: i32,
        y: i32,
    },
    KeyDown {
        code: Keycode,
        repeat: bool,
    },
    KeyUp {
        code: Keycode,
        repeat: bool,
    },
}

/// UI handler for dispatching events and holding main state.
pub struct GUI {
    game_state: World,
    states: StateMachine,
    frames: u64,
}

impl GUI {
    /// Create a new UI.
    pub fn new(game_state: World) -> Self {
        GUI {
            frames: 0,
            game_state,
            states: StateMachine::default(),
        }
    }

    /// Start the UI, i.e the event loop and rendering.
    pub fn start(&mut self) {
        let c = conf::Conf::new();
        let ctx = &mut Context::load_from_conf("prospero", "holmgr", c).unwrap();
        event::run(ctx, self).unwrap();
    }
}

impl event::EventHandler for GUI {
    fn mouse_button_down_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.states.handle_event(Event::MouseDown { button, x, y });
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        self.states.handle_event(Event::MouseUp { button, x, y });
    }

    fn mouse_motion_event(
        &mut self,
        _ctx: &mut Context,
        _state: MouseState,
        x: i32,
        y: i32,
        xrel: i32,
        yrel: i32,
    ) {
        self.states
            .handle_event(Event::MouseMove { x, y, xrel, yrel });
    }

    fn mouse_wheel_event(&mut self, _ctx: &mut Context, x: i32, y: i32) {
        self.states.handle_event(Event::MouseWheel { x, y });
    }

    fn key_down_event(&mut self, _ctx: &mut Context, code: Keycode, _keymod: Mod, repeat: bool) {
        self.states.handle_event(Event::KeyDown { code, repeat });
    }

    fn key_up_event(&mut self, _ctx: &mut Context, code: Keycode, _keymod: Mod, repeat: bool) {
        self.states.handle_event(Event::KeyUp { code, repeat });
    }

    /// Update the UI.
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    /// Draw the UI.
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::present(ctx);
        if self.frames % 100 == 0 {
            debug!("FPS: {:.1}", timer::get_fps(ctx));
        }
        self.frames += 1;

        // Render the current view, if any.
        if let Some(view) = self.states.current() {
            let mut render_ctx = RenderContext::new(&self.game_state, ctx);
            view.render(&mut render_ctx);
        }

        timer::yield_now();
        Ok(())
    }
}
