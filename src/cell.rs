extern crate piston;
extern crate piston_window;

use opengl_graphics::GlGraphics;
use piston::event_loop::*;
use piston_window::*;
use std::cmp::max;

#[derive(Debug)]
enum UIState {
  Idle,
  Selected,
  Hovered,
}

pub struct Cell {
  x: i8,
  y: i8,
  width: f64,
  height: f64,
  number: u8,
  ui_state: UIState,
}

type RGBAColor = [f32; 4];

fn rgbtorgba(s: [f32; 3]) -> RGBAColor {
  [s[0], s[1], s[2], 1.0]
}

fn color_from_uistate(uistate: &UIState) -> RGBAColor {
  match uistate {
    UIState::Selected => rgbtorgba([0.3, 0.7, 0.1]),
    UIState::Hovered => rgbtorgba([0.5; 3]),
    UIState::Idle => rgbtorgba([1.0; 3]),
  }
}

impl Cell {
  pub fn new(x: i8, y: i8, width: f64, height: f64) -> Cell {
    Cell {
      x,
      y,
      width,
      height,
      number: 0,
      ui_state: UIState::Idle,
    }
  }

  pub fn render(&mut self, viewport: &Viewport, gl: &mut GlGraphics, cursor_pos: [f64; 2]) {
    let vx = viewport.rect[0] as f64;
    let vy = viewport.rect[1] as f64;
    let c = Context::new_viewport(*viewport).trans(vx, vy);

    let x = max(0, self.x - 1) as f64 * self.width - (self.x as f64);
    let y = max(0, self.y - 1) as f64 * self.height - (self.y as f64);

    if (cursor_pos[0] - vx > x && cursor_pos[0] - vx < x + self.width)
      && (cursor_pos[1] - vy > y && cursor_pos[1] - vy < y + self.height)
    {
      self.ui_state = UIState::Hovered;
    }

    let color: RGBAColor = color_from_uistate(&self.ui_state);

    Rectangle::new(color).draw(
      [x, y, self.width, self.height],
      &DrawState::default(),
      c.transform,
      gl,
    );
  }
}
