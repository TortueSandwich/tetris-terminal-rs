use crate::utils::container::Container;

use super::{grid::Grid, polyomino_position::PolyominoPosition};

pub struct Playground {
    pub current_polyomino: PolyominoPosition,
    pub grid: Grid,
    pub contain: Container,
}

impl Playground {
    fn get_preview_polyomino(&self) -> PolyominoPosition {
        self.current_polyomino
            .get_preview_polyomino_position(&self.grid)
    }
    pub fn get_distance_preview_polyomino(&self) -> u16 {
        (self.get_preview_polyomino().org.0 - self.current_polyomino.org.0)
            .try_into()
            .unwrap()
    }
}
