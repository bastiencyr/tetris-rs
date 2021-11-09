use std::convert::TryInto;
use std::slice::{Iter, IterMut};

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
    pub fn get_case_borrow(&self, i: i32, j: i32) -> &Case {
        let index = j * crate::WIDTH + i;
        if index < 0 || index >= crate::WIDTH * crate::HEIGHT {
            print!(
                "Someone try to get an illegal value from the board. Request {}, {}",
                i, j
            );
            //return a default case even if the request case does not exist
            return &self.grid[0];
        }

        &self.grid[index as usize]
    }

    pub fn get_case_borrow_mut(&mut self, i: i32, j: i32) -> &mut Case {
        let index = j * crate::WIDTH + i;

        if index < 0 || index >= crate::WIDTH * crate::HEIGHT {
            print!(
                "Someone try to get an illegal value from the board. Request {}, {}",
                i, j
            );
            return &mut self.grid[0];
        }
        //here we use our iterator. It's not better but we use iterator
        self.into_iter().nth(index as usize).unwrap()
    }

    // update the board according a given piece
    pub fn update_board(&mut self, piece: &Piece) {
        for case in piece.data() {
            self.get_case_borrow_mut(case.x, case.y).set_empty(false);
        }
        //self.print_mat();
    }

    // remove full line from the board
    pub fn remove_full_lines(&mut self) {
        for (line, full_line) in self.get_full_lines().iter().enumerate() {
            if *full_line {
                self.remove_line(line as i32);
            }
        }
    }

    pub fn get_number_full_lines(&self) -> i32 {
        let mut nb = 0;
        for (_line, full_line) in self.get_full_lines().iter().enumerate() {
            if *full_line {
                nb += 1;
            }
        }
        nb
    }

    fn get_full_lines(&self) -> [bool; crate::HEIGHT as usize] {
        let mut v = [true; crate::HEIGHT as usize];
        for y in 0..crate::HEIGHT {
            for x in 0..crate::WIDTH {
                if self.get_case_borrow(x, y).empty() == true {
                    v[y as usize] = false;
                }
            }
        }
        v
    }

    fn remove_line(&mut self, line: i32) {
        for y in (1..line + 1).rev() {
            for x in 0..crate::WIDTH {
                let e = self.get_case_borrow(x, y - 1).empty();
                self.get_case_borrow_mut(x, y).set_empty(e);
            }
        }
        //on supprime la ligne la plus haute; y=0
        for x in 0..crate::WIDTH {
            self.get_case_borrow_mut(x, 0).set_empty(true);
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

    pub fn y(&self) -> i32 {
        self.coord.y()
    }

    pub fn x(&self) -> i32 {
        self.coord.x()
    }
}

//bancal -> à revoir
/*
impl  Iterator for Board {
    //on implémenter litérateur
    type Item =  Case;

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

 */

//TODO implement iterator over line and column

impl IntoIterator for Board {
    type Item = Case;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.into_iter()
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = &'a Case;
    type IntoIter = Iter<'a, Case>;

    fn into_iter(self) -> Iter<'a, Case> {
        self.grid.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut Case;
    type IntoIter = IterMut<'a, Case>;

    fn into_iter(self) -> IterMut<'a, Case> {
        self.grid.iter_mut()
    }
}