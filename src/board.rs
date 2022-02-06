use std::convert::TryInto;
use std::slice::{Iter, IterMut};

use itertools::enumerate;
use sdl2::rect::Point;

use crate::piece::Piece;

#[derive(Debug, Clone)]
pub struct Board {
    grid: Vec<Case>,
}

impl Board {
    pub fn new() -> Board {
        let mut grid = Vec::new();
        for i in 0..(crate::WIDTH * crate::HEIGHT).try_into().unwrap() {
            grid.push(Case::new(i % crate::WIDTH, i / crate::WIDTH))
        }
        Board { grid }
    }

    fn rows_mut<'a>(&'a mut self) -> impl Iterator<Item=&'a mut [Case]> + 'a {
        self.grid.chunks_mut(crate::WIDTH as usize)
    }

    fn rows2<'a>(&'a self) -> impl Iterator<Item=&'a [Case]> + 'a {
        self.grid.chunks(crate::WIDTH as usize)
    }

    //return cols as iterators over cols
    pub fn rows(&self) -> Vec<&[Case]> {
        let mut a = vec![];
        for nb_line in 0..crate::HEIGHT {
            let start = (nb_line * crate::WIDTH) as usize;
            let end = ((nb_line + 1) * crate::WIDTH) as usize;
            a.push(&(self.grid)[start..end]);
        }
        a
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
        for (index, row) in enumerate(self.rows()) {
            for case in row {
                if case.empty() == true {
                    v[index] = false;
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

//a box represents a square object in the board
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

//#############################################//
//############### USELESS CODE ################//
//#############################################//

//Méthode intéressante mais on ne dispose d'aucune méthode comme nth.
//On a ici implémenter notre propre itérateur, c'est sûrement la méthode la plus
// efficace jusque là
/*
let mut iter_rows = Iter2RowsBoard{ board: self.borrow_mut(), count: 0 };
let mut row = iter_rows.next();
while row.is_some()  {
    for el in row.unwrap(){
        el.empty = false;
    }
    row = iter_rows.next();
}
*/
pub struct Iter2RowsBoard<'a> {
    board: &'a mut Board,
    count: usize,
}

impl Iter2RowsBoard<'_> {
    fn next(&mut self) -> Option<IterMut<'_, Case>> {
        let nb_line = self.count;
        let h = crate::HEIGHT as usize;
        let w = crate::WIDTH as usize;
        if nb_line < h {
            let start = nb_line * w;
            let end = (nb_line + 1) * w;
            self.count += 1;
            return Some(self.board.grid[start..end].iter_mut());
        } else {
            None
        }
    }
}

//Une solution très complexe qui n'apporte pas grand chose
pub struct IterRowsBoard<'a> {
    board: &'a mut [Case],
}

impl<'a> Iterator for IterRowsBoard<'a> {
    type Item = std::slice::IterMut<'a, Case>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.board.is_empty() {
            return None;
        }
        let tmp = std::mem::replace(&mut self.board, &mut []);
        //let tmp = std::mem::take(&mut self.board);
        let (head, tail) = tmp.split_at_mut(crate::WIDTH as usize);
        self.board = tail;
        Some(head.iter_mut())
    }
}

// implement iterators for Board
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

    fn into_iter(self) -> Self::IntoIter {
        self.grid.iter()
    }
}

impl<'a> IntoIterator for &'a mut Board {
    type Item = &'a mut Case;
    type IntoIter = IterMut<'a, Case>;

    fn into_iter(self) -> Self::IntoIter {
        self.grid.iter_mut()
    }
}

//remove_line -
//Une solution complexe mais indépendante de toute implémentation
/*
for y in (1..line+1).rev(){
    let b = self.rows2().nth((y-1) as usize).unwrap();
    let mut b_copy= vec![Case::new(0,0);b.len()];
    b_copy.clone_from_slice(b);

    let mut a  = self.rows_mut().nth(y as usize).unwrap();

    let iter = a.iter_mut().partition().zip(b_copy.iter());
    for (mut a1, b2) in iter{
        a1.empty = b2.empty;
    }

 */
//}
