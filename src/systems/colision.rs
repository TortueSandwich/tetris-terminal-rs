use super::grid::Grille;
use super::polyomino_position::PolyominoPosition;
use crate::param_const::cst;

pub fn org_is_good(p: &PolyominoPosition) -> Result<(), ()> {
    // (0-19,0-20)
    let org: (i16, i16) = p.org;
    let coords: Vec<(i16, i16)> = p.to_coord();
    for coord in coords {
        if coord.0 < 0 || coord.1 < 0 {
            return Err(());
        }
        if coord.0 > cst::NB_LIGNE_GRILLE as i16 || coord.1 < cst::NB_COLONNE_GRILLE as i16 {
            return Err(());
        }
    }
    Ok(())
}

pub fn est_valide(g: &Grille, p: &PolyominoPosition) -> bool {
    for co in p.to_coord() {
        if g.est_rempli(co) {
            return false;
        }
    }
    true
}
