# Macroquad Canvas 2D

Macroquad Canvas 2D is a simple resolution-handling library that allows you to focus on making your game with a fixed resolution.

It is heavily inspired by [Push](https://github.com/Ulydev/push)

## How to use it

Import the library.

```rust
use macroquad_canvas_2d::*;
```

Create a new Canvas2D.

```rust
let canvas = Canvas2D::new(WIDTH as f32, HEIGHT as f32);
```

Draw!

```rust
loop {
  // Push canvas
  canvas.set_camera();
  {
    // Draw something inside the canvas
    // Clear background
    clear_background(WHITE);
    // Top left
    draw_rectangle(0.0, 0.0, 60.0, 60.0, RED);
    // Top right
    draw_rectangle(WIDTH as f32 - 60.0, 0.0, 60.0, 60.0, GRAY);
    // Bottom left
    draw_rectangle(0.0, HEIGHT as f32 - 60.0, 60.0, 60.0, GREEN);
    // Bottom right
    draw_rectangle(WIDTH as f32 - 60.0, HEIGHT as f32 - 60.0, 60.0, 60.0, BLUE);
  }
  // Pop canvas
  set_default_camera();

  // Draw canvas on screen
  canvas.draw_to_screen();

  next_frame().await
}

```

For more information check out the examples!

# TODO

- ✅ Function to transform canvas coordinates to screen coordinates.
- ✅ Mouse position, and transform.
- ✅ Camera movement and rotation.
  - ◻ Add camera constaint.
  - ◻ Add camera effects like shake.
- ◻ Simple post processing effects.
