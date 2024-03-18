#[repr(C)]
#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

pub const LIGHTGRAY: Color = Color {
    r: 200,
    g: 200,
    b: 200,
    a: 255,
};

pub const DARKGRAY: Color = Color {
    r: 80,
    g: 80,
    b: 80,
    a: 255,
};

pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255,
};

pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255,
};

pub const RAYWHITE: Color = Color {
    r: 245,
    g: 245,
    b: 245,
    a: 255,
};

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }
}
