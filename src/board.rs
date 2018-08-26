extern crate piston;
extern crate piston_window;

use cell::Cell;
use opengl_graphics::GlGraphics;
use piston_window::*;

const BOARD_SIZE: usize = 9;
const PADDING: f64 = 10.0;

pub struct Board {
  context: Option<Context>,
  width: f64,
  height: f64,
  board: [[u8; BOARD_SIZE]; BOARD_SIZE],
}

fn rgbtorgba(s: [f32; 3]) -> [f32; 4] {
  [s[0], s[1], s[2], 1.0]
}

impl Board {
  pub fn new() -> Board {
    Board {
      context: Option::None,
      width: 0.0,
      height: 0.0,
      board: [
        [0, 0, 0, 4, 0, 0, 2, 0, 0],
        [0, 0, 2, 0, 0, 0, 0, 1, 8],
        [5, 0, 6, 9, 0, 0, 0, 3, 0],
        [0, 6, 9, 0, 0, 0, 3, 0, 0],
        [0, 5, 0, 0, 0, 0, 0, 2, 1],
        [8, 0, 0, 1, 5, 7, 6, 0, 9],
        [0, 0, 0, 0, 3, 0, 9, 6, 0],
        [9, 0, 0, 6, 0, 2, 0, 5, 0],
        [0, 0, 0, 0, 0, 0, 7, 0, 2],
      ],
    }
  }

  pub fn set_dims(&mut self, width: f64, height: f64) {
    self.width = width;
    self.height = height;
    self.context = Option::from(Context::new_abs(width, height))
  }

  pub fn render(&mut self, viewport: &Viewport, gl: &mut GlGraphics) {
    if self.context.is_none() {
      panic!("Board dimensions have not been set!");
    }
    self.render_board(viewport, gl);
    self.render_cells(viewport, gl);
  }

  fn render_cells(&mut self, viewport: &Viewport, gl: &mut GlGraphics) {
    let block_size_x: f64 = ((self.width - (2.0 * PADDING)) / BOARD_SIZE as f64) as f64;
    let block_size_y: f64 = ((self.height - (2.0 * PADDING)) / BOARD_SIZE as f64) as f64;
    let mut cell: Cell = Cell::new(block_size_x, block_size_y);
    cell.render(viewport, gl)
  }

  fn render_board(&mut self, viewport: &Viewport, gl: &mut GlGraphics) {
    let color_black: [f32; 4] = rgbtorgba([0.0; 3]);

    gl.draw(*viewport, |_, gl| {
      clear([1.0; 4], gl);

      Rectangle::new_border(color_black, 1.0).draw(
        [
          PADDING,
          PADDING,
          self.width - (2.0 * PADDING),
          self.height - (2.0 * PADDING),
        ],
        &DrawState::default(),
        self.context.unwrap().transform,
        gl,
      );

      let block_size_x: f64 = ((self.width - (2.0 * PADDING)) / BOARD_SIZE as f64) as f64;
      let block_size_y: f64 = ((self.height - (2.0 * PADDING)) / BOARD_SIZE as f64) as f64;

      // Vertical lines
      for i in 1..10 {
        let thickness = if i % 3 == 0 { 1.0 } else { 0.5 };
        Line::new(color_black, thickness).draw(
          [
            PADDING + (block_size_x * i as f64),
            PADDING,
            PADDING + (block_size_x * i as f64),
            self.height - PADDING,
          ],
          &DrawState::default(),
          self.context.unwrap().transform,
          gl,
        );
      }

      // Horizontal lines
      for i in 1..10 {
        let thickness = if i % 3 == 0 { 1.0 } else { 0.5 };
        Line::new(color_black, thickness).draw(
          [
            PADDING,
            PADDING + (block_size_y * i as f64),
            self.width - PADDING,
            PADDING + (block_size_y * i as f64),
          ],
          &DrawState::default(),
          self.context.unwrap().transform,
          gl,
        );
      }
    })
  }
}
