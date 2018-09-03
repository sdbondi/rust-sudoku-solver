extern crate piston;
extern crate piston_window;

use board::Board;
use opengl_graphics::GlGraphics;
use piston::input::*;

pub struct App {
  board: Board,
  cursor_position: CursorPosition,
}

type CursorPosition = [f64; 2];

impl App {
  pub fn new() -> App {
    App {
      board: Board::new(),
      cursor_position: [0.0, 0.0],
    }
  }

  pub fn set_mouse_cursor_position(&mut self, x: f64, y: f64) {
    self.cursor_position = [x, y];
  }

  pub fn render(&mut self, args: &RenderArgs, gl: &mut GlGraphics) {
    self.board.set_dims(args.width as f64, args.height as f64);
    self.board.set_padding(10.0);
    self
      .board
      .render(&args.viewport(), gl, self.cursor_position);
  }
}
