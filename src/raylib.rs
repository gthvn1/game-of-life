use std::{
    ffi::{c_void, CString},
    os::raw::{c_char, c_int},
};

#[repr(C)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

extern "C" {
    fn InitWindow(w: c_int, h: c_int, title: *const c_char) -> c_void;
    fn CloseWindow() -> c_void;
    fn SetTargetFPS(fps: c_int) -> c_void;
    fn BeginDrawing() -> c_void;
    fn EndDrawing() -> c_void;
    fn WindowShouldClose() -> c_int;
    fn ClearBackground(c: Color) -> c_void;
    fn DrawText(text: *const c_char, x: c_int, y: c_int, fs: c_int, c: Color) -> c_void;
}

pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
};

pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};

pub fn init_window(width: i32, height: i32, title: String) {
    let c_title = CString::new(title).unwrap();
    unsafe {
        InitWindow(
            width as c_int,
            height as c_int,
            c_title.as_ptr() as *const c_char,
        )
    };
}

pub fn close_window() {
    unsafe { CloseWindow() };
}

pub fn set_target_fps(fps: i32) {
    unsafe { SetTargetFPS(fps as c_int) };
}

pub fn window_should_close() -> bool {
    unsafe { WindowShouldClose() != 0 }
}

pub fn begin_drawing() {
    unsafe { BeginDrawing() };
}

pub fn end_drawing() {
    unsafe { EndDrawing() };
}

pub fn clear_background(color: Color) {
    unsafe { ClearBackground(color) };
}

pub fn draw_text(text: String, pos_x: i32, pos_y: i32, font_size: i32, color: Color) {
    let c_text = CString::new(text).unwrap();
    unsafe {
        DrawText(
            c_text.as_ptr() as *const c_char,
            pos_x as c_int,
            pos_y as c_int,
            font_size as c_int,
            color,
        )
    };
}
