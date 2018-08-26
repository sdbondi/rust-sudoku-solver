extern crate piston;
extern crate piston_window;

use opengl_graphics::GlGraphics;
use piston_window::*;

#[derive(Debug)]
enum UIState {
  Idle,
  Selected,
  Hovered,
}

pub struct Cell {
  width: f64,
  height: f64,
  number: u8,
  ui_state: UIState,
}

fn rgbtorgba(s: [f32; 3]) -> [f32; 4] {
  [s[0], s[1], s[2], 1.0]
}

impl Cell {
  pub fn new(width: f64, height: f64) -> Cell {
    Cell {
      width,
      height,
      number: 0,
      ui_state: UIState::Idle,
    }
  }

  pub fn render(&mut self, viewport: &Viewport, gl: &mut GlGraphics) {
    let c = Context::new_viewport(*viewport);
    let color_black: [f32; 4] = rgbtorgba([0.0; 3]);

    Rectangle::new(color_black).draw(
      [0.0, 0.0, 50.0, 50.0],
      &DrawState::default(),
      c.transform,
      gl,
    );
  }
}
