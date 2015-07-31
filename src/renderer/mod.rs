use opengl_graphics::GlGraphics;

use traits::View;


pub fn render<V: View>(gl: GlGraphics, view: V) {
  view.render(&gl);
}
