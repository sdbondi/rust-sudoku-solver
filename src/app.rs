extern crate piston;
extern crate piston_window;

use board::Board;
use opengl_graphics::GlGraphics;
use piston::input::*;

pub struct App {
  board: Board,
}

impl App {
  pub fn new() -> App {
    App {
      board: Board::new(),
    }
  }

  pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
    self.board.set_dims(args.width as f64, args.height as f64);
    self.board.render(&args.viewport(), gl);
  }
}
