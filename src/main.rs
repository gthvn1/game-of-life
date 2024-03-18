use rust_raylib::raylib_bindings::camera2d;
use rust_raylib::raylib_bindings::color;
use rust_raylib::raylib_bindings::keys::{KEY_LEFT, KEY_RIGHT};
use rust_raylib::raylib_bindings::rectangle::Rectangle;
use rust_raylib::raylib_bindings::vector2::Vector2;
use rust_raylib::raylib_bindings::{
    begin_drawing, begin_mode_2d, clear_background, close_window, draw_line, draw_rectangle,
    draw_rectangle_rec, draw_text, end_drawing, end_mode_2d, init_window, is_key_pressed,
    set_target_fps, window_should_close,
};

use rand::Rng;

fn main() {
    // Let's implement core_2d_camera.c (example)
    const SCREEN_WIDTH: i32 = 800;
    const SCREEN_HEIGHT: i32 = 450;
    const MAX_BUILDINGS: i32 = 100;

    init_window(SCREEN_WIDTH, SCREEN_HEIGHT, "core 2d example".to_string());

    let mut player: Rectangle = Rectangle::new(400.0, 280.0, 40.0, 40.0);
    let mut buildings: Vec<Rectangle> = Vec::new();
    let mut buildings_color: Vec<color::Color> = Vec::new();

    let mut spacing: i32 = 0;

    for _ in 0..MAX_BUILDINGS {
        let width = rand::thread_rng().gen_range(50..=200);
        let height = rand::thread_rng().gen_range(100..=100);
        let y: f32 = SCREEN_HEIGHT as f32 - 130.0 - height as f32;
        let x: f32 = -6000.0 + spacing as f32;

        buildings.push(Rectangle::new(width as f32, height as f32, x, y));
        buildings_color.push(color::Color::new(
            rand::thread_rng().gen_range(200..=240) as u8,
            rand::thread_rng().gen_range(200..=240) as u8,
            rand::thread_rng().gen_range(200..=250) as u8,
            255,
        ));

        spacing += width;
    }

    let mut camera = camera2d::Camera2D::new(
        Vector2::new(player.x + 20.0, player.y + 20.0),
        Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        0.0,
        1.0,
    );

    set_target_fps(60); // Set our game to run at 60 frames-per-second

    // Main game loop
    while !window_should_close()
    // Detect window close button or ESC key
    {
        // Player mouvement
        if is_key_pressed(KEY_RIGHT) {
            player.x += 2.0;
        } else if is_key_pressed(KEY_LEFT) {
            player.x -= 2.0;
        }

        // Camera target follows player
        camera.target = Vector2::new(player.x + 20.0, player.y + 20.0);

        // Skip camera rotation for now so begin drawing ...
        begin_drawing();

        clear_background(color::RAYWHITE);

        begin_mode_2d(camera);

        draw_rectangle(-6000, 320, 13000, 8000, color::DARKGRAY);

        for i in 0..MAX_BUILDINGS {
            draw_rectangle_rec(buildings[i as usize], buildings_color[i as usize]);
        }

        draw_rectangle_rec(player, color::RED);

        draw_line(
            camera.target.x as i32,
            -SCREEN_HEIGHT * 10,
            camera.target.x as i32,
            SCREEN_HEIGHT * 10,
            color::GREEN,
        );

        draw_line(
            -SCREEN_HEIGHT * 10,
            camera.target.y as i32,
            SCREEN_HEIGHT * 10,
            camera.target.y as i32,
            color::GREEN,
        );
        end_mode_2d();

        draw_text("SCREEN AREA".to_string(), 640, 10, 20, color::RED);
        draw_rectangle(0, 0, SCREEN_WIDTH, 5, color::RED);

        end_drawing();
    }

    // De-Initialization
    close_window(); // Close window and OpenGL context
}
