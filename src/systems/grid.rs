// use std::usize;

// use crate::systems::Tetromino::PositionedPolyomino;

use super::polyomino_position::{Coord, PolyominoPosition};
use crate::param_const::cst;
use crate::utils::tetromino::Forme;

#[derive(Clone, Copy, PartialEq)]
pub enum Case {
    Empty,
    Filled(Forme),
}

impl Case {
    pub fn code_ansi(&self) -> &str {
        use crate::param_const::param::*;
        match self {
            Case::Filled(Forme::L) => COULEUR_L,
            Case::Filled(Forme::J) => COULEUR_J,
            Case::Filled(Forme::S) => COULEUR_S,
            Case::Filled(Forme::Z) => COULEUR_Z,
            Case::Filled(Forme::I) => COULEUR_I,
            Case::Filled(Forme::T) => COULEUR_T,
            Case::Filled(Forme::O) => COULEUR_O,
            Case::Empty => COULEUR_BORD,
        }
    }
}

// impl std::fmt::Display for Case {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let color: u8 = match self {
//             Case::Filled(Forme::L) => 44,
//             Case::Filled(Forme::J) => 43,
//             Case::Filled(Forme::S) => 42,
//             Case::Filled(Forme::Z) => 41,
//             Case::Filled(Forme::I) => 46,
//             Case::Filled(Forme::T) => 45,
//             Case::Filled(Forme::O) => 47,
//             Case::Empty => 0,
//         };
//         write!(f, "\x1B[{}m  \x1B[0m", color)?;
//         Ok(())
//     }
// }

#[derive(Clone, Copy)]
pub struct Grille {
    pub data: [[Case; cst::NB_COLONNE_GRILLE as usize]; cst::NB_LIGNE_GRILLE as usize],
}

impl Grille {
    pub fn new() -> Grille {
        Grille {
            data: [[Case::Empty; cst::NB_COLONNE_GRILLE as usize]; cst::NB_LIGNE_GRILLE as usize],
        }
    }

    pub fn get_case(&self, co: Coord) -> &Case {
        &self.data[co.0 as usize][co.1 as usize]
    }

    pub fn get_case_mut(&mut self, co: Coord) -> &mut Case {
        &mut self.data[co.0 as usize][co.1 as usize]
    }

    pub fn est_rempli(&self, co: Coord) -> bool {
        *self.get_case(co) != Case::Empty
    }
}

impl Grille {
    fn pose_case(&mut self, co: Coord, p: Forme) {
        *self.get_case_mut(co) = Case::Filled(p);
    }

    pub fn pose_polyomino(&mut self, p: &PolyominoPosition) {
        let positions: Vec<Coord> = p.to_coord();
        for pos in positions {
            self.pose_case(pos, p.forme());
        }
    }

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

impl Grille {
    fn line_is_full(&self, row:usize) -> bool {
        self.data[row].iter().all(|&cell| cell != Case::Empty)
    }
    fn reset_line(&mut self, row:usize) {
        let new_row = [Case::Empty; cst::NB_COLONNE_GRILLE as usize];
        self.data[row] = new_row;
    }

    pub fn clear_lines(&mut self) -> Option<u16> {
        let mut num_cleared = 0;
        for row in (0..self.data.len()).rev() {
            if self.line_is_full(row) {
                num_cleared += 1;
            } else if num_cleared > 0 {
                self.data[row + num_cleared] = self.data[row];
            }
        }
        for row in 0..num_cleared {
            self.reset_line(row);
        }

        if num_cleared == 0 { None }
        else { Some(num_cleared as u16) }
    }
}





// impl std::fmt::Display for Grille {
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
