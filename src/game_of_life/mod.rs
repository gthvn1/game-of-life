// dynamic array
// just a vector of u8.
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub enum GofError {
    ReadFile,
    WrongWidth,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellState {
    Dead,
    Alive,
}

#[derive(Clone, Copy, Debug)]
struct Cell {
    state: CellState,
    neighbors: usize, // number of neighbors
}

const DEAD_CELL: Cell = Cell {
    state: CellState::Dead,
    neighbors: 0,
};

const ALIVE_CELL: Cell = Cell {
    state: CellState::Alive,
    neighbors: 0,
};

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
                    '@' => array.push(ALIVE_CELL),
                    _ => array.push(DEAD_CELL),
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
                if let Some(c) = self.get_cell(x.try_into().unwrap(), y.try_into().unwrap()) {
                    match c.state {
                        //CellState::Dead => print!(".{} ", c.neighbors),
                        //CellState::Alive => print!("@{} ", c.neighbors),
                        CellState::Dead => print!("."),
                        CellState::Alive => print!("@"),
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
        //
        // We will first upgrade the count of neighbors and then we will update the state of the
        // Cell.

        // Update the number of neighbors
        for y in 0..self.height {
            for x in 0..self.width {
                // We use i32 because neighbors can be out of grid (that means no neighbor).
                let x_i32: i32 = x.try_into().unwrap();
                let y_i32: i32 = y.try_into().unwrap();
                if let Some(c) = self.get_cell(x_i32, y_i32) {
                    self.set_cell(
                        x,
                        y,
                        Cell {
                            state: c.state,
                            neighbors: self.get_live_neighbors(x_i32, y_i32),
                        },
                    );
                } else {
                    unreachable!("at this point a cell must exist")
                }
            }
        }

        //println!("After computing neighbors...");
        //self.dump();

        // Update the state according to rules
        for y in 0..self.height {
            for x in 0..self.width {
                let x_i32: i32 = x.try_into().unwrap();
                let y_i32: i32 = y.try_into().unwrap();
                if let Some(c) = self.get_cell(x_i32, y_i32) {
                    match c.state {
                        CellState::Alive => {
                            if !(2..=3).contains(&c.neighbors) {
                                self.set_cell(x, y, DEAD_CELL);
                            } else {
                                self.set_cell(x, y, ALIVE_CELL);
                            }
                        }
                        CellState::Dead => {
                            if c.neighbors == 3 {
                                self.set_cell(x, y, ALIVE_CELL);
                            } else {
                                self.set_cell(x, y, DEAD_CELL);
                            }
                        }
                    }
                }
            }
        }

        //println!("Updating CellState done...");
    }

    pub fn get_size(&self) -> (i32, i32) {
        (
            self.width.try_into().unwrap(),
            self.height.try_into().unwrap(),
        )
    }

    pub fn get_state(&self, x: i32, y: i32) -> CellState {
        if let Some(cell) = self.get_cell(x, y) {
            cell.state
        } else {
            CellState::Dead
        }
    }

    fn get_live_neighbor(&self, x: i32, y: i32) -> usize {
        if let Some(c) = self.get_cell(x, y) {
            if c.state == CellState::Alive {
                1
            } else {
                0
            }
        } else {
            0
        }
    }

    fn get_live_neighbors(&self, x: i32, y: i32) -> usize {
        self.get_live_neighbor(x, y - 1)
            + self.get_live_neighbor(x + 1, y - 1)
            + self.get_live_neighbor(x + 1, y)
            + self.get_live_neighbor(x + 1, y + 1)
            + self.get_live_neighbor(x, y + 1)
            + self.get_live_neighbor(x - 1, y + 1)
            + self.get_live_neighbor(x - 1, y)
            + self.get_live_neighbor(x - 1, y - 1)
    }

    fn set_cell(&mut self, x: usize, y: usize, c: Cell) {
        assert!(y * self.width + x < self.width * self.height);
        self.vptr[y * self.width + x] = c;
    }

    fn get_cell(&self, x: i32, y: i32) -> Option<Cell> {
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
