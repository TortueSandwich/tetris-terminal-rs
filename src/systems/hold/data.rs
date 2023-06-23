use crate::{utils::{tetromino::Tetromino, container::*}, erreur::{AlreadyHolding, NothingHolded}};
use std::mem;

pub struct Hold {
    pub(crate) holded_polyomino: Option<Tetromino>,
    pub contain: Container,
}

impl Hold {
    fn get_holded(&self) -> &Option<Tetromino> {
        &self.holded_polyomino
    }

    fn is_holding(&self) -> bool {
        self.get_holded().is_some()
    }
    fn is_none(&self) -> bool {
        self.get_holded().is_none()
    }

    fn set_holded(&mut self, t: Option<Tetromino>) -> Result<(), AlreadyHolding> {
        if self.is_holding() && t.is_some() {
            Err(AlreadyHolding)
        } else {
            self.holded_polyomino = t;
            Ok(())
        }
    }
    

    pub fn take_holded(&mut self) -> Result<Option<Tetromino>, NothingHolded> {
        if self.is_none() {
            Err(NothingHolded)
        } else  {
            let res = self.holded_polyomino.take();
            Ok(res)
        }
    }

    pub fn switch_with(&mut self, t: Option<Tetromino>) -> Option<Tetromino> {
        if self.is_holding() {
            mem::replace(&mut self.holded_polyomino, t)
        } else {
            self.set_holded(t);
            None
        }
    }
}