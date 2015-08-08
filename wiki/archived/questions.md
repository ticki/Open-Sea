### Hey wai,

I have some questions about the code.



#### Requests for Documentation

The following is a list of items for which I need clearer documentation/explanation. I know much of this is probably terminology from another engine/framework/etc., but not all of it is clear to me.

- `View`
- `View::start()` and `View::end()` (What are these supposed to do? Also, should they have empty default implementations? You defined them that way.)

:: They're events which should be called when the view changes. The reason for the empty default method is that not every view necessarily will have any contents for this event.

- `GameView` (Especially in constrast to `View`)

:: This is just a temporary naming for a view which is in game (not a menu etc.)

- `models` (Is this module for entities/game objects?)

:: A model is just a game object.

You don't have to document them yourself; we'll can always discuss them tomorrow and I'll write the docs!

:: Sorry for the bad docs...


#### Design Choices We Should Discuss

- I think we should use `f32` or `f64` for positions, otherwise the finest movement granularity we can achieve is one coordinate/pixel per frame (without storing extra data or scaling world coordinates, etc.)

:: The i64 positions are for the grid position. Therefore the movement will probably be stored in a different field.

- Currently, the `traits` module seems to just be storing a lot of commonly used functionality. I vote that we refactor it into a `core` module where we can put things until they should be organized out.

:: I'm okay with that!

- Getters and setters are a little archaic. Will things be modifying object coordinates/directions directly? Could these items just be made private? This one really merits a discussion about the design of the movement system.

:: Let's discuss this in the chat.



#### Talk to you tomorrow