extern crate piston;
extern crate piston_window;
extern crate sdl2_window;

use piston::event_loop::{EventSettings, Events};
use piston_window::{PistonWindow, WindowSettings};
use sdl2_window::Sdl2Window;

mod board;
mod renderer;

const WINDOW_TITLE: &'static str = "Sudoku Solver";
const WINDOW_SIZE: [u32; 2] = [800, 640];

fn main() {
  let mut window: PistonWindow<Sdl2Window> = WindowSettings::new(WINDOW_TITLE, WINDOW_SIZE)
    .exit_on_esc(true)
    .build()
    .unwrap();

  let board = board::new();
  let renderer = renderer::new();

  let mut events = Events::new(EventSettings::new());
  while let Some(e) = events.next(&mut window) {
    if let Some(args) = e.render_args() {
      renderer.render_ui(args)
    }
  }
}
