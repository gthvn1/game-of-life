// dynamic array
// just a vector of u8.
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum GofError {
    ReadFile,
    WrongWidth,
}

#[derive(Clone, Copy, Debug)]
enum Cell {
    Dead,
    Alive,
}

pub struct GameOfLife {
    vptr: Vec<Cell>,
    width: usize,
    height: usize,
}

impl GameOfLife {
    pub fn new(fname: &str) -> Result<GameOfLife, GofError> {
        let file = match File::open(fname) {
            Ok(v) => v,
            Err(_) => return Err(GofError::ReadFile),
        };

        let f = BufReader::new(file);
        let mut array: Vec<Cell> = Vec::new();
        let mut width = 0;
        let mut height = 0;

        // initialiazing the vector and keep its width and height.
        for line in f.lines() {
            let l = line.unwrap();
            if width != 0 {
                if width != l.len() {
                    return Err(GofError::WrongWidth);
                }
            } else {
                width = l.len();
            }
            for c in l.chars() {
                match c {
                    '@' => array.push(Cell::Alive),
                    _ => array.push(Cell::Dead),
                }
            }
            height += 1;
        }

        Ok(GameOfLife {
            vptr: array,
            width,
            height,
        })
    }

    pub fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(c) = self.get_idx(x.try_into().unwrap(), y.try_into().unwrap()) {
                    match c {
                        Cell::Dead => print!("."),
                        Cell::Alive => print!("@"),
                    }
                }
            }
            println!();
        }
    }

    pub fn update(&mut self) {
        // Every cell interacts with its eight neighbors.
        // Four rules:
        //     - Any live cell with fewer than two live neighbors dies, as if by underpopulation.
        //     - Any live cell with two or three live neighbors lives on to the next generation.
        //     - Any live cell with more than three live neighbors dies, as if by overpopulation.
        //     - Any dead cell with exactly three live neighbors becomes a live cell, as if by reproduction.
        for y in 0..self.height {
            for x in 0..self.width {
                let x_i32: i32 = x.try_into().unwrap();
                let y_i32: i32 = y.try_into().unwrap();
                let neighbors = self.get_live_neighbors(x_i32, y_i32);
                match self.get_idx(x_i32, y_i32) {
                    Some(Cell::Alive) => {
                        if !(2..=3).contains(&neighbors) {
                            self.set_idx(x, y, Cell::Dead)
                        }
                    }
                    Some(Cell::Dead) => {
                        if neighbors == 3 {
                            self.set_idx(x, y, Cell::Alive)
                        }
                    }
                    _ => {} // nothing to do
                }
            }
        }
    }

    fn get_live_neighbor(&self, x: i32, y: i32) -> usize {
        if let Some(Cell::Alive) = self.get_idx(x, y) {
            1
        } else {
            0
        }
    }

    fn get_live_neighbors(&self, x: i32, y: i32) -> usize {
        //    0 1 2 3 4
        //  0 x x x x x
        //  1 x x x x x
        //  2 x x x x x
        //  3 x x x x x
        //
        //
        //
        //      (x - 1, y - 1) - (x , y - 1 ) - (x + 1, y - 1)
        //      (x - 1, y)     -    (x , y)   - (x + 1, y)
        //      (x - 1, y + 1) - (x , y + 1 ) - (x + 1, y + 1)

        //
        let up = self.get_live_neighbor(x, y + 1);
        let up_right = self.get_live_neighbor(x + 1, y - 1);
        let right = self.get_live_neighbor(x + 1, y);
        let down_right = self.get_live_neighbor(x + 1, y + 1);
        let down = self.get_live_neighbor(x, y + 1);
        let down_left = self.get_live_neighbor(x - 1, y + 1);
        let left = self.get_live_neighbor(x - 1, y);
        let up_left = self.get_live_neighbor(x - 1, y - 1);

        up + up_left + left + down_left + down + down_right + right + up_right
    }

    fn set_idx(&mut self, x: usize, y: usize, c: Cell) {
        assert!(y * self.width + x < self.width * self.height);
        self.vptr[y * self.width + x] = c;
    }

    fn get_idx(&self, x: i32, y: i32) -> Option<Cell> {
        if x < 0
            || x >= self.width.try_into().unwrap()
            || y < 0
            || y >= self.height.try_into().unwrap()
        {
            None
        } else {
            Some(self.vptr[y as usize * self.width + x as usize])
        }
    }
}
