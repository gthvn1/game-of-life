use rust_raylib::raylib_bindings::color;
use rust_raylib::raylib_bindings::{
    begin_drawing, clear_background, close_window, draw_rectangle, end_drawing, init_window,
    set_target_fps, window_should_close,
};

use rust_raylib::game_of_life::GameOfLife;

#[allow(unreachable_code)]
fn main() {
    println!("We want to read the initial state from file");

    let mut gof = GameOfLife::new("input.gol").unwrap();
    println!("Initial state");
    gof.dump();

    // Update once and check if it is ok
    for _ in 0..10 {
        println!();
        gof.update();
        gof.dump();
    }

    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;

    init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "Game Of Life".to_string());

    set_target_fps(60); // Set our game to run at 60 frames-per-second

    // Main game loop
    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Drawing
        // --------------------------------------------------------------------
        begin_drawing();

        clear_background(color::BLACK);

        draw_rectangle(0, 0, SCREEN_WIDTH, 5, color::RED);

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
