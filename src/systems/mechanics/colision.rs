use crate::{systems::playground::{polyomino_position::PolyominoPosition, grid::Grid}};

// pub fn org_is_good(p: &PolyominoPosition) -> Result<(), ()> {
//     // (0-19,0-20)
//     let org: (i16, i16) = p.org;
//     let coords: Vec<(i16, i16)> = p.to_coord();
//     for coord in coords {
//         if coord.0 < 0 || coord.1 < 0 {
//             return Err(());
//         }
//         if coord.0 > cst::NB_LIGNE_GRILLE as i16 || coord.1 < cst::NB_COLONNE_GRILLE as i16 {
//             return Err(());
//         }
//     }
//     Ok(())
// }



pub fn est_valide(g: &Grid, p: &PolyominoPosition) -> bool {
    for co in p.to_coord() {
        if g.is_filled(co.1 as usize, co.0 as usize).unwrap() {
            return false;
        }
    }
    true
}
