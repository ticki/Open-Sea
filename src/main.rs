extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;
extern crate noise;

use piston::window::WindowSettings;
use piston::event::{ Event, Events };
use piston::input::{ Button, Input, Key };
use glutin_window::GlutinWindow as Window;
use opengl_graphics::OpenGL;

mod mapgen;
mod traits;
mod models;
mod renderer;


const TITLE: &'static str = "Open Sea";

fn main() {
    let gl_context = OpenGL::_2_1;

    let window = Window::new(gl_context,
                             WindowSettings::new(TITLE, [800, 600]).exit_on_esc(true));

    // This is the object used for drawing
    let mut gl = opengl_graphics::GlGraphics::new(gl_context);

    for event in window.events() {
        match event {
            Event::Input(Input::Press(Button::Keyboard(Key::Return))) =>
                println!("Return pressed!"),

            Event::Input(Input::Release(Button::Keyboard(Key::Return))) =>
                println!("Return released!"),

            Event::Render(args) => {
                use graphics::*;
                gl.draw(args.viewport(), |_, gl| {
                    clear([1.0, 1.0, 1.0, 1.0], gl);
                    renderer::render(&gl);
                });
            },

            _ => {}
        }
    }
}
