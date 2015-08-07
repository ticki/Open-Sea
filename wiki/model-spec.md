# Model File Specification

This is the specification for model files. (`data/models/*.json`) All units are
tiles (16x16 pixels) unless specified otherwise.



# Keys

#### name

This is an arbitrary name for the model. Conventionally, this should match the
filename (without ".json".)


#### sprite_data

This list contains the data for the sprites of the model.

Key          | Default?        | Type        | Description
-------------|-----------------|-------------|------------
`cut_from`   | `[0,0]`         | `[u32,u32]` | Where from the image to cut the graphic
`cut_offset` | `[0,0]`         | `[i8,i8]`   | The *pixel* offset of the graphic in the source image
`file`       | `"tileset.png"` | `string`    | The path to the image, relative to `data/graphics`
`frame`      | `0`             | `u32`       | The `frame` the graphic belongs to
`pin_offset` | `[0,0]`         | `[i8,i8]`   | The *pixel* offset of the graphic on the model
`pin_to`     | `[0,0]`         | `[i32,i32]` | Where the graphic should be drawn relative to the model
`size`       | `[1,1]`         | `[u32,u32]` | The size of the rectangle cut from the image
`sprite`     | `"default"`     | `string`    | The sprite of the model that this graphic belongs to
`with`       | N/A             | `structure` | A set of graphic structures containing different defaults

This is a bit complex, so check out the examples.


#### occupied_tiles

The elements of this list take one of these two forms:

- `{ "individual": [ [3,4], [4,4], [5,4] ] }`
- `{ "rectangle": { "start": [3,4], "size": [3,1] } }`

The two examples are equivalent.



# Examples

#### Rock

A model for a 1x1 tile rock.

```json5
// Contents of the file data/models/rock.json
{
  // The name of the model
  "name": "Rock",

  // The sprite is the first from the left, second from the top of the
  // default tileset
  "sprite_data": [
    { "cut_from": [0,1] }
  ],

  // The rock occupies one tile where it is positioned
  "occupied_tiles": [
    { "individual": [ [0,0] ] }
  ]
}
```


#### Tree

A model for a 2x2 tile tree. Its "collision box" is just the bottom two tiles.

```json5
// Contents of the file data/models/tree.json
{
  "name": "Tree",

  // The sprite is in a contiguous block in the default tileset
  // (It starts at the 5th from the left, 3rd from the top tile)
  "sprite_data": [
    {
      "cut_from": [4,2],
      "size": [2,2]
    }
  ],

  // The tree only occupies its bottom two tiles
  // (so entities can sit behind the tree)
  "occupied_tiles": [
    { "individual": [ [0,1], [1,1] ] }
  ]
}
```


#### House

A model for a 5x4 tile house. Its "collision box" is the bottom 5x3 tiles.

```json5
// Contents of the file data/models/house_a.json
{
  "name": "House A",

  // The sprite is made up of the building, the door, and the window
  // They are in contiguous blocks in the file data/graphics/house.png
  "sprite_data": [
    {
      "with": {
        "defaults": { "file": "house.png" },
        // All of these items will cut from house.png by default
        "data": [
          { // the building
            "cut_from": [0,0],
            "size": [5,4]
          },
          { // the window
            "cut_from": [5,0],
            "pin_to": [3,2]
          },
          { // the door
            "cut_from": [5,1],
            "size": [1,2],
            "pin_to": [1,2]
          }
        ]
      }
    }
  ],

  "occupied_tiles": [
    { "rectangle": { "start": [0,1], "size": [5,3] } }
  ]
}
```


#### Flower

A model for a 1x1 flower tile. It doesn't collide; it's just decoration for the
grass. It's animated so it looks like the flowers blow in the wind.

```json5
// Contents of the file data/models/flower.json
{
  "name": "Flower",

  // The sprite is made up of the grass and the flowers. It's animated.
  "sprite_data": [
    {
      "with": {
        // The default for "frame" is already 0. This just makes it look
        // consistent with the next "with" block.
        "defaults": { "frame": 0 },
        "data": [
          { "cut_from": [1,0] }, // grass
          { "cut_from": [2,0] }  // flowers (first frame)
        ]
      },
      "with": {
        "defaults": { "frame": 1 },
        "data": [
          { "cut_from": [1,0] }, // grass
          { "cut_from": [3,0] }  // flowers (second frame)
        ]
      }
    }
  ],

  // Does not occupy space
  "occupied_tiles": []
}
```


#### Wall Lamp

A model for a 1x1 light fixture. It doesn't occupy space or collide; it is
fixed on a wall. It can be switched on and off, however.

```json5
// Contents of the file data/models/wall_lamp.json
{
  "name": "Wall Lamp",

  // When the lamp is switched on, it changes sprites. (That functionality
  // is not encoded here, but when the model is loaded, the game logic can
  // access the "off" sprite and the "on" sprite.) The sprite is in the
  // file data/graphics/light.png.
  "sprite_data": [
    {
      "with": {
        "defaults": { "file": "light.png" },
        "data": [
          {
            "sprite": "off",
            "cut_from": [0,0]
          },
          {
            "sprite": "on",
            "cut_from": [1,0]
          }
        ]
      }
    }
  ],

  // Does not occupy space
  "occupied_tiles": []
}
```
