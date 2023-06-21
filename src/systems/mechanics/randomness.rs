
use rand::seq::SliceRandom;

use crate::utils::tetromino::{Tetromino, Forme};


pub struct PolyominoBag([Tetromino; 7]);

impl PolyominoBag {
    pub fn new() -> PolyominoBag {
        let mut formes = [Forme::I, Forme::J, Forme::L, Forme::O, Forme::S, Forme::T, Forme::Z];
        formes.shuffle(&mut rand::thread_rng());
    
        let bag: [Tetromino; 7] = [
            Tetromino::from(formes[0]),
            Tetromino::from(formes[1]),
            Tetromino::from(formes[2]),
            Tetromino::from(formes[3]),
            Tetromino::from(formes[4]),
            Tetromino::from(formes[5]),
            Tetromino::from(formes[6]),
        ];
    
        PolyominoBag(bag)
    }
}

impl IntoIterator for PolyominoBag {
    type Item = Tetromino;
    type IntoIter = std::vec::IntoIter<Tetromino>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.to_vec().into_iter()
    }
}

// pub struct PolyominoBag {
//     bag: [Tetromino; 7],
// }

// impl PolyominoBag {
//     pub fn new() -> PolyominoBag {
//         let formes = [Forme::I, Forme::J, Forme::L, Forme::O, Forme::S, Forme::T, Forme::Z,];
//         formes.shuffle(&mut rand::thread_rng());
//         PolyominoBag{bag:formes}
//     }

//     pub fn get_Tetro(&mut self) -> Option<Tetromino>{
//         if self.head < Forme::COUNT {
//             let res = self.bag[self.head].clone();
//             self.head += 1;
//             Some(res)
//         } else {
//             None
//         }
//     }

//     pub fn is_empty(&self) -> bool {
//         self.head < Forme::COUNT
//     }
// }