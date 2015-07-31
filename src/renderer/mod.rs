use super::graphics::*;
use super::traits::View;

pub fn render(gl: Graphics, view: View) {
  view.render();
}
