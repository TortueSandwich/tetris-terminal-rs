// use std::usize;

// use crate::systems::Tetromino::PositionedPolyomino;

use std::ops::{Index, IndexMut};

use super::polyomino_position::{Coord, PolyominoPosition};
use crate::param_const::cst;
use crate::utils::tetromino::Forme;

#[derive(Clone, Copy, PartialEq)]
pub enum Case {
    Empty,
    Filled(Forme),
}

impl Case {
    pub fn color(&self) -> crossterm::style::Color {
        match self {
            Case::Filled(f) => f.color(),
            Case::Empty => crate::param_const::param::COULEUR_,
        }
    }
}


#[derive(Clone, Copy)]
pub struct Grid (
    pub [[Case; cst::NB_COLONNE_GRILLE as usize]; cst::NB_LIGNE_GRILLE as usize],
);

impl Index<usize> for Grid {
    type Output = [Case; cst::NB_COLONNE_GRILLE as usize];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<usize> for Grid {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}


impl Grid {
    // :)
    fn len(&self) -> usize {
        self.0.len()
    }
    pub fn new() -> Grid {
        Grid ([[Case::Empty; cst::NB_COLONNE_GRILLE as usize]; cst::NB_LIGNE_GRILLE as usize])
    }

    fn get_case(&self, co: Coord) -> &Case {
        &self[co.0 as usize][co.1 as usize]
    }

    fn get_case_mut(&mut self, co: Coord) -> &mut Case {
        &mut self[co.0 as usize][co.1 as usize]
    }

    pub fn get_couleur_case(&self, co: (u16,u16)) -> crossterm::style::Color {
        self.get_case((co.0 as i16, co.1 as i16)).color()
    }

    pub fn est_rempli(&self, co: Coord) -> bool {
        *self.get_case(co) != Case::Empty
    }
}

impl Grid {
    fn pose_case(&mut self, co: Coord, p: Forme) {
        *self.get_case_mut(co) = Case::Filled(p);
    }

    pub fn pose_polyomino(&mut self, p: &PolyominoPosition) {
        let positions: Vec<Coord> = p.to_coord();
        for pos in positions {
            self.pose_case(pos, p.forme());
        }
    }

    #[allow(unused)]
    pub fn est_posable(&self, p: &PolyominoPosition) -> Result<(), ()> {
        let positions: Vec<Coord> = p.to_coord();
        for pos in positions {
            if self.est_rempli(pos) {
                return Err(());
            }
        }
        Ok(())
    }
}

impl Grid {
    fn line_is_full(&self, row:usize) -> bool {
        self[row].iter().all(|&cell| cell != Case::Empty)
    }
    fn reset_line(&mut self, row:usize) {
        let new_row = [Case::Empty; cst::NB_COLONNE_GRILLE as usize];
        self[row] = new_row;
    }

    pub fn clear_lines(&mut self) -> Option<u16> {
        let mut num_cleared = 0;
        for row in (0..self.len()).rev() {
            if self.line_is_full(row) {
                num_cleared += 1;
            } else if num_cleared > 0 {
                self[row + num_cleared] = self[row];
            }
        }
        for row in 0..num_cleared {
            self.reset_line(row);
        }

        if num_cleared == 0 { None }
        else { Some(num_cleared as u16) }
    }
}





// impl std::fmt::Display for Grid {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for _ in 0..cst::NB_COLONNE_GRILLE + 2 {
//             write!(f, "\x1B[107m  \x1B[0m")?;
//         }
//         writeln!(f)?;
//         for y in &self.data {
//             write!(f, "\x1B[107m  \x1B[0m")?;
//             for x in y {
//                 write!(f, "{}", x)?;
//             }
//             write!(f, "\x1B[107m  \x1B[0m")?;
//             writeln!(f)?;
//         }
//         for _ in 0..cst::NB_COLONNE_GRILLE + 2 {
//             write!(f, "\x1B[107m  \x1B[0m")?;
//         }
//         writeln!(f)?;
//         Ok(())
//     }
// }
