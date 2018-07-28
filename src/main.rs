extern crate opengl_graphics;
extern crate piston;
extern crate piston_window;
extern crate sdl2_window;

use piston::event_loop::*;
use piston_window::*;
use sdl2_window::Sdl2Window;

mod app;
mod board;

const WINDOW_TITLE: &'static str = "Sudoku Solver";
const WINDOW_SIZE: [u32; 2] = [800, 640];

fn main() {
  use opengl_graphics::GlGraphics;

  let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let mut a = app::App::new();

  let mut gl = GlGraphics::new(OpenGL::V3_2);
  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      a.render(&args, &mut gl);
    }
  }
}
