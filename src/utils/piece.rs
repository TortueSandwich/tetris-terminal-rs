#![allow(unused)]
use std::{pin::Pin, cell::Cell};



struct Polyomino<T, const DEGRE: usize ,const NB_OF_ROTATION : usize> 
where T : Clone
{
    all_data : [[T; DEGRE]; NB_OF_ROTATION],
    pos: Cell<usize>,
}



impl<T, const DEGRE:usize ,const NB_OF_ROTATION : usize> Polyomino<T,DEGRE,NB_OF_ROTATION>
where T : Clone {
    pub fn next(&self) -> [T; DEGRE] {
        let p = self.pos.get();
        self.pos.set((p + 1) % NB_OF_ROTATION);
        self.get()
    }
    pub fn back(&self) -> [T; DEGRE] {
        let p = self.pos.get();
        self.pos.set(if p > 0 { p - 1 } else { NB_OF_ROTATION - 1 });
        self.get()
    }
    fn get(&self) -> [T; DEGRE] {
        self.all_data[self.pos.get()].clone()
    }
    fn new(data : [[T; DEGRE]; NB_OF_ROTATION] ) -> Self {
        Polyomino { 
            all_data: data, 
            pos: 0.into(),
        }
    }
}

