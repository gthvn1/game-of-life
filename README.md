# Game of Life?

Just play with [raylib](https://www.raylib.com/) and [rust](https://www.rust-lang.org/).
Game of Life maybe be a goal, will see.

# Description of steps

## Linking with Hello C

To be able to use raylib we first need to be able to link our rust code with
C code. So the first step is to call a C function that is implemented in `clib/`.

*Note*: code hase been removed but before removing the example it has been tagged
with **hello**.

## Linking with Raylib

To run it you need to create a directory `rayluib/` and copy the `libraylib.a`.
It can easly be compiled from [github:raylib](https://github.com/raysan5/raylib).
Otherwise you can modify `builld.sh` to fit your needs.

- [x] Start by implementing [Basic Window](https://www.raylib.com/examples/core/loader.html?name=core_basic_window)
- [ ] Draw a rectangle and move it...
