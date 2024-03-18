// dynamic array
// just a vector of u8.
#[derive(Clone, Copy, Debug)]
pub enum Cell {
    Dead,
    Alive,
}

pub struct GameOfLife<'a> {
    vptr: &'a Vec<Cell>,
    width: usize,
    height: usize,
}

impl<'a> GameOfLife<'a> {
    pub fn new(vptr: &'a Vec<Cell>, width: usize, height: usize) -> GameOfLife {
        GameOfLife {
            vptr,
            width,
            height,
        }
    }

    pub fn dump(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}x{}->{:?}  ", x, y, self.get_idx(x, y));
            }
            println!();
        }
    }

    pub fn update(&mut self) {
        unimplemented!()
    }

    fn get_idx(&self, x: usize, y: usize) -> Cell {
        assert!(y * self.width + x < self.width * self.height);
        self.vptr[y * self.width + x]
    }
}
