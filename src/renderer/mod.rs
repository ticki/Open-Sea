use super::graphics::*;

pub mod view;

pub use view::*;

pub fn render(gl: Graphics, view: View) {
  view.render();
}
