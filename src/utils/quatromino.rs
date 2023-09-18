use std::cell::Cell;

use super::coord::Coord;

struct Polyomino {
    data : [(u16, u16); 4],
}
struct RotationClass {
    data : [Polyomino; 4],
}



// impl std::ops::Add<(u16, u16)> for VecCoord {
//     type Output = VecCoord;
//     fn add(self, rhs: (u16, u16)) -> Self::Output {
//         self.data.into_iter().map(|(x,y)| (x + rhs.0, y + rhs.1))
//     }
// }
