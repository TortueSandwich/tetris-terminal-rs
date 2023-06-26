use crate::{utils::{container::Container, tetromino::Tetromino}, param_const::param};

use super::{grid::Grid, polyomino_position::PolyominoPosition};

pub struct Playground {
    pub current_polyomino: PolyominoPosition,
    pub grid: Grid,
    pub contain: Container,
}

impl Playground {
    pub fn new() -> Self {
        Playground {
            current_polyomino: PolyominoPosition::rand(),
            grid: Grid::new(),
            contain: param::CONTAINER_GRID,
        }
    }
    fn get_preview_polyomino(&self) -> PolyominoPosition {
        self.current_polyomino
            .get_preview_polyomino_position(&self.grid)
    }
    pub fn get_distance_preview_polyomino(&self) -> u16 {
        (self.get_preview_polyomino().org.0 - self.current_polyomino.org.0)
            .try_into()
            .unwrap()
    }

    pub fn set_polyomino(&mut self, t : Tetromino) {
        self.current_polyomino = PolyominoPosition::from(t);
    }
    pub fn get_polyomino(&self) -> Tetromino {
        self.current_polyomino.polyomino
    }

    pub fn place_on_grid(&mut self) -> Result<(), crate::erreur::InvalidCoordinatesError>{
       self.grid.pose_polyomino(&self.current_polyomino) 
    }
}
