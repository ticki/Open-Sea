extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate noise;
extern crate num;
extern crate rustc_serialize;
extern crate time;

use piston::window::WindowSettings;
use piston::event::{Event, Events};
use piston::input::{Button, Input, Key};

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, GlGraphics};

use graphics::Image;
use graphics::rectangle::square;

// Why pub? Because that makes the docs work.
pub mod core;
pub mod math;
pub mod mapgen;
pub mod model;
pub mod renderer;
pub mod state;

use core::Config;
use state::State;

use renderer::{Renderer, TextureManager};


fn main() {
  let state = state::Game::new();

  let gl_context = OpenGL::_2_1;

  let config = match Config::load() {
    Ok(config) => config,
    Err(e) => {
      println!("Couldn't load config:");
      println!("  {:?}", e);
      return;
    }
  };

  let window_settings =
    WindowSettings::new(config.game_title().clone(),
                        config.window_size() ).exit_on_esc(true);
  let window = Window::new(gl_context, window_settings);

  let mut tex_mgr = TextureManager::new();

  let image = Image::new().rect(square(100.0, 10.0, 64.0));
  let path_to_stamos = "./data/graphics/stamos.png";
  let texture = match tex_mgr.get(path_to_stamos) {
    Ok(john) => john,
    Err(e) => {
      println!("Couldn't load John Stamos:");
      println!("  {:?}", e);
      return;
    }
  };

  // This is the object used for drawing
  let mut gl = GlGraphics::new(gl_context);
  let mut renderer = Renderer::new();

  for event in window.events() {
    match event {
      Event::Input(Input::Press(Button::Keyboard(Key::Return))) =>
        println!("Return pressed!"),

      Event::Input(Input::Release(Button::Keyboard(Key::Return))) =>
        println!("Return released!"),

      Event::Render(args) => {
        use graphics::*;
        gl.draw(args.viewport(), |c, gl| {
          clear([1.0, 1.0, 1.0, 1.0], gl);
          image.draw(texture, default_draw_state(), c.transform, gl);
          state.render(&args, gl, &mut renderer);
        });
      },

      _ => {}
    }
  }
}
