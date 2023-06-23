use crate::utils::{
    direction::{Direction, Rotation},
    tetromino::{Forme, Tetromino},
};

use super::grid::Grid;
pub type Coord = (i16, i16);

#[derive(Clone, Debug)]
pub struct PolyominoPosition {
    pub org: Coord,
    pub polyomino: Tetromino,
}

impl PolyominoPosition {
    pub fn collides_with(&self, grid: &Grid) -> bool {
        let positions = self.to_coord();
        for pos in positions {
            if let Some(case) = grid.get_case(pos.1 as usize, pos.0 as usize).ok() {
                if case.is_filled() {
                    return true;
                }
            } else {
                return true;
            }
        }
        false
    }
}

impl PolyominoPosition {
    pub fn est_bougeable(&self, dir: Direction, g: &Grid) -> Option<PolyominoPosition> {
        let mut moke = self.clone();
        use Direction::*;

        let (row_delta, col_delta) = match dir {
            U => (-1, 0),
            L => (0, -1),
            D => (1, 0),
            R => (0, 1),
        };

        for c in self.to_coord() {
            let new_row = (c.0 + row_delta) as usize;
            let new_col = (c.1 + col_delta) as usize;
            if new_row >= Grid::Y_LEN as usize || new_col >= Grid::X_LEN as usize {
                return None;
            }
            if let Some(case) = g.get_case(new_col, new_row).ok() {
                if case.is_filled() {
                    return None;
                }
            } else {
                return None;
            }
        }

        moke.org.0 += row_delta;
        moke.org.1 += col_delta;
        Some(moke)
    }

    pub fn est_tournable(&mut self, r: Rotation, g: &Grid) -> Option<PolyominoPosition> {
        self.srs(r, g)
    }
}

impl From<Tetromino> for PolyominoPosition {
    fn from(value: Tetromino) -> Self {
        // let mut v = value.clone();
        // v.coordonne.current_value = 0;
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

    pub fn color(&self) -> crossterm::style::Color {
        self.polyomino.color()
    }
}

impl PolyominoPosition {
    pub fn get_preview_polyomino_position(&self, g: &Grid) -> PolyominoPosition {
        let mut moke = self.clone();
        while let Some(t) = moke.est_bougeable(Direction::D, g) {
            moke = t;
        }
        moke
    }
}
