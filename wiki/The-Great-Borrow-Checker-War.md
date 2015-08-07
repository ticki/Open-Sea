Big horrible realization: `CharacterCache`'s type `Texture` is *not* just used to retrieve the size of the image (i.e., it is not just an instance of `ImageSize`). It *must* match the `Texture` type of your `Graphics` instance in order to make text drawing calls with it.

To clarify, our `Graphics` instance is `GlGraphics`, whose texture is `opengl_graphics::Texture`.

So everything is mangled to shit.

:+1:

I'll work on it more later, wai.