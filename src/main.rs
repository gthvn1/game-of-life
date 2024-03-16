mod hello;
mod raylib;

use crate::raylib::{
    begin_drawing, clear_background, close_window, draw_text, end_drawing, init_window,
    set_target_fps, window_should_close, LIGHTGRAY, RAYWHITE,
};

fn main() {
    let x: i32 = hello::get_number();
    println!("Got {x}!");
    hello::say_hello("Sailor".to_string());

    // Let's implement the Basic Window Raylib example...
    //Initialization
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    init_window(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        "raylib [core] example - basic window".to_string(),
    );

    set_target_fps(60); // Set our game to run at 60 frames-per-second

    // Main game loop
    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Update
        // TODO: Update your variables here
        // Draw
        begin_drawing();

        clear_background(RAYWHITE);

        draw_text(
            "Congrats! You created your first window!".to_string(),
            190,
            200,
            20,
            LIGHTGRAY,
        );

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
