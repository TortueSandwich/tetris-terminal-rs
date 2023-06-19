use crate::utils::{
    direction::Direction,
    tetromino::{Forme, Tetromino},
};

use crate::param_const::cst;

use super::grid::Grille;
pub type Coord = (i16, i16);

#[derive(Clone)]
pub struct PolyominoPosition {
    pub org: Coord,
    pub polyomino: Tetromino,
}


fn co_est_invalide(c:(i16,i16)) -> bool {
    let (x,y) = c;
       x < 0
    || x >= cst::NB_LIGNE_GRILLE as i16
    || y < 0
    || y >= cst::NB_COLONNE_GRILLE as i16
}

impl PolyominoPosition {
    pub fn est_bougeable(&self, dir: Direction, g: &Grille) -> Option<PolyominoPosition> {
        let mut moke = self.clone();
        use Direction::*;

        let (row_delta, col_delta) = match dir {
            U => (-1, 0),
            L => (0, -1),
            D => (1, 0),
            R => (0, 1),
        };

        for c in self.to_coord() {
            let new_row = c.0 + row_delta;
            let new_col = c.1 + col_delta;
            let co = (new_row, new_col); 
            if co_est_invalide(co) {
                return None;
            }
            if g.est_rempli(co) {
                return None;
            }
        }

        moke.org.0 += row_delta;
        moke.org.1 += col_delta;
        Some(moke)
    }



    pub fn est_tournable(&mut self, dir: Direction, g: &Grille) -> Option<PolyominoPosition> {
        let mut moke = self.clone();
        if let Err(_) = moke.polyomino.rotate(dir) {
            return None;
        }
        for co in moke.to_coord() {
            if co_est_invalide(co) || g.est_rempli(co) {
                return None;
            }
        }
        Some(moke)
    }
}

impl From<Tetromino> for PolyominoPosition {
    fn from(value: Tetromino) -> Self {
        PolyominoPosition {
            org: (0, 3),
            polyomino: value,
        }
    }
}

impl PolyominoPosition {
    pub fn rand() -> Self {
        Self::from(Tetromino::rand())
    }

    pub fn to_coord(&self) -> Vec<Coord> {
        let it: Vec<Coord> = self.polyomino.to_coord();
        let modified_coords: Vec<Coord> = it
            .iter()
            .map(|&(x, y)| {
                (
                    x.wrapping_add(self.org.0.try_into().unwrap()),
                    y.wrapping_add(self.org.1.try_into().unwrap()),
                )
            })
            .collect();
        modified_coords
    }

    pub fn forme(&self) -> Forme {
        self.polyomino.forme()
    }

    pub fn code_ansi(&self) -> &str {
        self.polyomino.code_ansi()
    }
}
