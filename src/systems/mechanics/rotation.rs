// use super::super::playground::polyomino_position::PolyominoPosition;

use crate::systems::playground::grid::Grid;
use crate::systems::playground::polyomino_position::PolyominoPosition;
use crate::utils::direction::Rotation;
use crate::utils::tetromino::Forme;

impl PolyominoPosition {
    pub fn srs(&self, r: Rotation, g: &Grid) -> Option<Self> {
        let mut moke = self.clone();
        let co_org = self.org.clone();

        let data = match self.forme() {
            Forme::J | Forme::L | Forme::T | Forme::S | Forme::Z => &JLTSZ,
            Forme::I => &I,
            Forme::O => return None,
        };

        let m = match r {
            Rotation::ClockWise => 1,
            Rotation::CounterClockWise => -1,
        };
        let idx = match r {
            Rotation::ClockWise => moke.polyomino.coordonne.current_value,
            Rotation::CounterClockWise => (moke.polyomino.coordonne.current_value + 3) % 4,
        };

        if let Err(e) = moke.polyomino.basic_rotation(r) {
            panic!("Wtf, erreur ici (.srs()) ? : {e}");
        }

        let mut nb_test = 0;
        while nb_test < 5 && moke.collides_with(&g) {
            let (dx, dy) = data[idx][nb_test];
            moke.org.0 = co_org.0 + dx * m;
            moke.org.1 = co_org.1 + dy * m;
            nb_test += 1;
        }
        if moke.collides_with(&g) {
            return None;
        }
        return Some(moke);
    }
}

pub static JLTSZ: [[(i16, i16); 5]; 4] = [
    [(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)], // 0 >> 1 & 1 >> 0
    [(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],     // 1 >> 2 & 2 >> 1
    [(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],    // 2 >> 3 & 3 >> 2
    [(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],  // 3 >> 0 & 0 >> 3
]; // sens aiguilles
   // faire *-1 pour sens anti-h

pub static I: [[(i16, i16); 5]; 4] = [
    [(0, 0), (-2, 0), (1, 0), (-2, -1), (1, 2)],
    [(0, 0), (-1, 0), (2, 0), (-1, 2), (2, -1)],
    [(0, 0), (2, 0), (-1, 0), (2, 1), (-1, -2)],
    [(0, 0), (1, 0), (-2, 0), (1, -2), (-2, 1)],
];
