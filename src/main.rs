use rust_raylib::raylib_bindings::{
    begin_drawing, clear_background, close_window, draw_rectangle, end_drawing, init_window,
    set_target_fps, window_should_close,
};
use rust_raylib::raylib_bindings::{color, draw_line};

use rust_raylib::game_of_life::{CellState, GameOfLife};

#[allow(unreachable_code)]
fn main() {
    let mut gof = GameOfLife::new("input.gol").unwrap();
    let (game_width, game_height) = gof.get_size();

    let cell_width: i32 = 10;
    let cell_height: i32 = 10;

    let screen_width: i32 = cell_width * game_width;
    let screen_height: i32 = cell_height * game_height;

    println!("GOF {} x {}", game_width, game_height);
    println!("screen {} x {}", screen_width, screen_height);

    init_window(screen_width, screen_height, "Game Of Life".to_string());

    set_target_fps(10); // Set our game to run at 10 frames-per-second

    // Main game loop
    // Detect window close button or ESC key
    while !window_should_close() {
        // Drawing
        // --------------------------------------------------------------------
        begin_drawing();

        clear_background(color::BLACK);

        let step = cell_width;
        let mut i = step;

        while i < screen_width {
            draw_line(i, 0, i, screen_height, color::DARKGRAY); // vertical line
            draw_line(0, i, screen_width, i, color::DARKGRAY); // horizontal line
            i = i + step;
        }

        // Draw cell
        for y in 0..game_height {
            for x in 0..game_width {
                if gof.get_state(x, y) == CellState::Alive {
                    draw_rectangle(
                        x * cell_width,
                        y * cell_height,
                        cell_width,
                        cell_height,
                        color::GREEN,
                    );
                }
            }
        }

        end_drawing();

        gof.update();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
