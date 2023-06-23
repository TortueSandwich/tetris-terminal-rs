// use std::usize;

// use crate::systems::Tetromino::PositionedPolyomino;

use std::ops::{Index, IndexMut};

use super::polyomino_position::{Coord, PolyominoPosition};
use crate::erreur::InvalidCoordinatesError;
use crate::param_const::cst;
use crate::utils::tetromino::Forme;
use crate::utils::writer::quit_quit;

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
    pub fn is_filled(&self) -> bool {
        *self != Case::Empty
    }
}


#[derive(Clone, Copy)]
pub struct Grid (
    pub [[Case; Self::X_LEN as usize]; Self::Y_LEN as usize],
);

#[allow(unused)]
impl Grid {
    pub const X_LEN : u16 = cst::NB_COLONNE_GRILLE;
    pub const Y_LEN : u16 = cst::NB_LIGNE_GRILLE;
}

impl Index<usize> for Grid {
    type Output = [Case; Self::X_LEN as usize];
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
    fn validate_coordinates(x: usize, y: usize) -> Result<(), InvalidCoordinatesError> {
        if x >= Self::X_LEN as usize || y >= Self::Y_LEN as usize {
            return Err(InvalidCoordinatesError);
        }
        Ok(())
    }
    
    pub fn new() -> Grid {
        Grid([[Case::Empty; Self::X_LEN as usize]; Self::Y_LEN as usize])
    }
    
    pub fn get_case(&self, x: usize, y: usize) -> Result<&Case, InvalidCoordinatesError> {
        Self::validate_coordinates(x, y)?;
        Ok(&self.0[y][x])
    }
    
    pub fn get_case_mut(&mut self, x: usize, y: usize) -> Result<&mut Case, InvalidCoordinatesError> {
        Self::validate_coordinates(x, y)?;
        Ok(&mut self.0[y][x])
    }
    
    
    pub fn get_couleur_case(&self, x: usize, y: usize) -> Result<crossterm::style::Color, InvalidCoordinatesError> {
        match self.get_case(x, y) {
            Ok(case) => Ok(case.color()),
            Err(err) => Err(err),
        }
    }
    
    pub fn is_filled(&self, x: usize, y: usize) -> Result<bool, InvalidCoordinatesError> {
        match self.get_case(x, y) {
            Ok(case) => Ok(case.is_filled()),
            Err(err) => Err(err),
        }
    }
    
}


impl Grid {
    fn pose_case(&mut self, x: usize, y: usize, p: Forme) -> Result<(), InvalidCoordinatesError> {
        Self::validate_coordinates(x, y)?;
        *self.get_case_mut(x, y)? = Case::Filled(p);
        Ok(())
    }
    
    pub fn pose_polyomino(&mut self, p: &PolyominoPosition) -> Result<(), InvalidCoordinatesError> {
        let positions: Vec<Coord> = p.to_coord();
        for pos in positions {
            if let Err(e) = self.pose_case(pos.1 as usize, pos.0 as usize, p.forme()) {
                quit_quit();
                panic!("Erreur lors de la pose du polyomino {p:?} :\n{e}");
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
        let new_row = [Case::Empty; Self::X_LEN as usize];
        self[row] = new_row;
    }

    pub fn clear_lines(&mut self) -> Option<u16> { // OPtion ???
        let mut num_cleared:usize = 0;
        for row in (0..Self::Y_LEN as usize).rev() {
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
        else { Some(num_cleared.try_into().unwrap()) }
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
