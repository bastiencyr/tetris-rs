use sdl2::rect::Point;
use std::convert::TryInto;

#[derive(Debug, Clone)]
pub struct Board {
    grid: Vec<Box>, //grid is a  vector of boxes. Each box represents in the tetris.
    count: i32,
}

impl Board {
    pub fn new() -> Board {
        let mut grid = Vec::new();
        for i in 0..(crate::WIDTH * crate::HEIGHT).try_into().unwrap() {
            grid.push(Box::new(i % crate::WIDTH, i / crate::WIDTH))
        }
        Board { grid, count: 0 }
    }

    pub fn get_i_j(&self, i: i32, j: i32) -> &Box {
        let index = j * crate::WIDTH + i;
        if j<0 {
            print!("Someone try to get an illegal value from the board, {}, {}", i, j);
            return &self.grid[0];
        }
        if j>= crate::WIDTH*crate::HEIGHT {
            print!("Someone try to get an illegal value from he board, {}, {}", i, j);
            return &self.grid[0];
        }

        &self.grid[index as usize]
    }

    pub fn get_mut_ij(&mut self, i: i32, j: i32) -> &mut Box {
        let index = j * crate::WIDTH + i;
        &mut self.grid[index as usize]
    }

    pub fn update_board<T: crate::PieceModel>(&mut self, piece: &T) {
        for case in piece.get_points() {
            self.get_mut_ij(case.x, case.y).empty = false;
        }
    }
    /*
    pub fn full_line(&self)->Vec{
    let full_line = Vec::new();
    let mut full = false;
    //since we can iterate over lines or columns, we need two kind of iterators
    for line in &self.iterate_lines{
        for case in line{
        }
    }
    }
    */
    pub fn print_mat(&self) {
        for case in &self.grid {
            if case.newline() {
                print!("\n");
            }
            if case.empty() {
                print!("-{}-", 0);
            } else {
                print!("-{}-", 1);
            }
        }
        print!("\n");
    }
}
//a box represent a square object in the board
#[derive(Debug, Clone)]
pub struct Box {
    coord: Point,
    empty: bool,
}

impl Box {
    fn new(i: i32, j: i32) -> Box {
        Box {
            coord: Point::new(i, j),
            empty: true,
        }
    }

    pub fn empty(&self) -> bool {
        self.empty
    }

    pub fn set_empty(&mut self, empty: bool) {
        self.empty = empty;
    }

    pub fn newline(&self) -> bool {
        if self.coord.x == 0 {
            return true;
        }
        return false;
    }
}

//bancal -> à revoir
impl Iterator for Board {
    //on implémenter litérateur
    type Item = Box;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.count as usize;
        if self.count < self.grid.len().try_into().unwrap() {
            self.count += 1;
            Some(self.grid[index].clone())
        } else {
            self.count = 0;
            None // on renvoie None pour le dernier élément. Et y a pas une connerie de s'arrêter a lavat dernier comme en c++
        }
    }
}
