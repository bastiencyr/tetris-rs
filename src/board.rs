use std::convert::TryInto;

use sdl2::rect::Point;

use crate::piece::Piece;

#[derive(Debug, Clone)]
pub struct Board {
    grid: Vec<Case>,
    //grid is a  vector of boxes. Each box represents in the tetris.
    count: i32,
}

impl Board {
    pub fn new() -> Board {
        let mut grid = Vec::new();
        for i in 0..(crate::WIDTH * crate::HEIGHT).try_into().unwrap() {
            grid.push(Case::new(i % crate::WIDTH, i / crate::WIDTH))
        }
        Board { grid, count: 0 }
    }

    // j = ligne (y), i = colonne (x)
    pub fn get_i_j(&self, i: i32, j: i32) -> &Case {
        let index = j * crate::WIDTH + i;
        if index < 0 {
            print!(
                "Someone try to get an illegal value from the board, {}, {}",
                i, j
            );
            return &self.grid[0];
        }
        if index >= crate::WIDTH * crate::HEIGHT {
            print!(
                "Someone try to get an illegal value from he board, {}, {}",
                i, j
            );
            return &self.grid[0];
        }

        &self.grid[index as usize]
    }

    pub fn get_mut_ij(&mut self, i: i32, j: i32) -> &mut Case {
        let index = j * crate::WIDTH + i;
        &mut self.grid[index as usize]
    }

    pub fn update_board(&mut self, piece: &Piece) {
        for case in piece.data() {
            self.get_mut_ij(case.x, case.y).set_empty(false);
        }
        for (line, full_line) in self.get_full_lines().iter().enumerate() {
            if *full_line {
                print!("Remove LINE {}", line);
                self.remove_line(line as i32);
            }
        }
        self.print_mat();
    }

    pub fn get_full_lines(&self) -> [bool; crate::HEIGHT as usize] {
        let mut v = [true; crate::HEIGHT as usize];
        for y in 0..crate::HEIGHT {
            for x in 0..crate::WIDTH {
                if self.get_i_j(x, y).empty() == true {
                    v[y as usize] = false;
                }
            }
        }
        v
    }

    pub fn remove_line(&mut self, line: i32) {
        for y in (1..line + 1).rev() {
            println!("{}", line);
            for x in 0..crate::WIDTH {
                let e = self.get_i_j(x, y - 1).empty();
                self.get_mut_ij(x, y).set_empty(e);
            }
        }
        //on supprime la ligne la plus haute; y=0
        for x in 0..crate::WIDTH {
            self.get_mut_ij(x, 0).set_empty(true);
        }
    }

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
pub struct Case {
    coord: Point,
    empty: bool,
}

impl Case {
    fn new(i: i32, j: i32) -> Case {
        Case {
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
    type Item = Case;

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
