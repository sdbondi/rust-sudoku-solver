extern crate piston_window;
extern crate piston;

use piston_window::*;

struct Renderer {}

pub fn new() -> Renderer {
  Renderer {}
}

impl Renderer {
  pub fn render_ui(&mut self, args RenderArgs) {
    clear([1.0; 4], args);
    rectangle(
      [1.0, 0.0, 0.0, 1.0], // red
      [0.0, 0.0, 100.0, 100.0],
      c.transform,
      g,
    );
  }
}
