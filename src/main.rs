extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate noise;

use std::path::Path;

use piston::window::WindowSettings;
use piston::event::{ Event, Events };
use piston::input::{ Button, Input, Key };

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{OpenGL, GlGraphics, Texture};

use graphics::Image;
use graphics::rectangle::square;

// Why pub? Because that makes the docs work.
pub mod mapgen;
pub mod core;
pub mod models;
pub mod renderer;

use core::*;

use renderer::Renderer;


const TITLE: &'static str = "Open Sea";

fn main() {
  let view = core::GameView::new();

  let gl_context = OpenGL::_2_1;

  let window = Window::new(gl_context,
               WindowSettings::new(TITLE, [800, 600]).exit_on_esc(true));

  // Create the image object and attach a square Rectangle object inside.
  let image = Image::new().rect(square(100.0, 10.0, 200.0));
  // A texture to use with the image
  let texture = Texture::from_path(Path::new("./assets/stamos.png")).unwrap();

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

          view.render(&args, gl, &mut renderer);

          image.draw(&texture, default_draw_state(), c.transform, gl);
        });
      },

      _ => {}
    }
  }
}
