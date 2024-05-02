# Game of Life

Just play with [raylib](https://www.raylib.com/) and [rust](https://www.rust-lang.org/).
Why not writing a Game of Life in Rust with Raylib bindings :)

# Description of steps

## Linking with Hello C

To be able to use raylib we first need to be able to link our rust code with
C code. So the first step is to call a C function that is implemented in `clib/`.

*Note*: code hase been removed but it has been tagged **hello** before doing so.

## Linking with Raylib

- **Important**: To run it you need to create a directory `raylib/` and copy the `libraylib.a`.
Raylib can easly be compiled from [github:raylib](https://github.com/raysan5/raylib).
Otherwise you can modify `build.rs` to fit your needs.

- run: `cargo run -- --fname examples/input.gol`

## Steps
- [x] Start by implementing [Basic Window](https://www.raylib.com/examples/core/loader.html?name=core_basic_window)
- [x] [2D camera](https://www.raylib.com/examples/core/loader.html?name=core_2d_camera) example
- [x] wrote game of life

## Screenshots

<img align="center" src="https://github.com/gthvn1/rusty-gof/blob/master/screenshots/more_complex.png">
