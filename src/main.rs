extern crate rust_raylib;

use rust_raylib::raylib_bindings::color;
use rust_raylib::raylib_bindings::rectangle::Rectangle;
use rust_raylib::raylib_bindings::{
    begin_drawing, clear_background, close_window, draw_rectangle_rec, draw_text, end_drawing,
    init_window, set_target_fps, window_should_close,
};

fn main() {
    // Let's implement raylib example
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    let rec: Rectangle = Rectangle::new(0.0, 0.0, 40.0, 40.0);

    init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "raylib example".to_string());

    set_target_fps(60); // Set our game to run at 60 frames-per-second

    // Main game loop
    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Update
        // TODO: Update your variables here
        // Draw
        begin_drawing();

        clear_background(color::RAYWHITE);

        draw_rectangle_rec(rec, color::RED);

        draw_text(
            "Congrats! You created your first window!".to_string(),
            190,
            200,
            20,
            color::LIGHTGRAY,
        );

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
