use crate::utils::{tetromino::Tetromino, container::*};

pub struct Hold {
    pub holded_polyomino: Option<Tetromino>,
    pub contain: Container,
}

#[allow(unused)]
impl Hold {
    fn is_holding(&self) -> bool {
        self.holded_polyomino.is_some()
    }
    fn get_holded(&self) -> Option<Tetromino> {
        self.holded_polyomino
    }

    pub fn switch_with(&mut self, t: Tetromino) -> Option<Tetromino> {
        if let Some(res) = self.holded_polyomino {
            self.holded_polyomino = Some(t);
            Some(res)
        } else {
            self.holded_polyomino = Some(t);
            None
        }
    }
}