pub mod cst {
    pub const NB_LIGNE_GRILLE: u16 = 20;
    pub const NB_COLONNE_GRILLE: u16 = 10;
    pub const NB_PREVIEW: u16 = 7;
    pub const NB_BAG: usize = 2;
}

use crate::utils::container::Container;
pub mod param {
    use crossterm::style;

    use crate::utils::container::Container;

    use super::cst;

    pub const COULEUR_BORD: &str = "107";

    pub const COULEUR_L: &str = "75";
    pub const COULEUR_J: &str = "191";
    pub const COULEUR_S: &str = "118";
    pub const COULEUR_Z: &str = "9";
    pub const COULEUR_I: &str = "14";
    pub const COULEUR_T: &str = "5";
    pub const COULEUR_O: &str = "11";
    
    pub const COULEUR_PREVIEW :&str = "235";
    pub const COULEUR_: &str = "0";

    pub const COULEUR_TEST :&str = "1";


    pub const x_grille:u16 = 8;
    pub const y_grille:u16 = 0;
    pub const POSITION_GRILLE : ((u16,u16),(u16, u16)) = ((x_grille*2,x_grille+super::cst::NB_COLONNE_GRILLE+1),(y_grille,y_grille+super::cst::NB_LIGNE_GRILLE+1)); 

    pub const CONTAINER_HOLD:Container = Container::default((0, 6), (0, 6));
    pub const CONTAINER_GRID:Container = Container::default((8, 20), (0, 22));
    pub const CONTAINER_NEXT:Container = Container::default((24, 24 + 7), (0, cst::NB_PREVIEW*4 + 2));

    const x_hold:u16 = 0;
    const y_hold:u16 = 0;
    pub const POSITION_HOLD : ((u16,u16),(u16, u16)) = ((x_hold*2,x_hold*2+4+1),(y_hold,y_hold+4+1)); 
    
    pub const x_next:u16 = 12;
    pub const y_next:u16 = 0;
    pub const POSITION_NEXT : ((u16,u16),(u16, u16)) = ((x_next*2*2,x_next*2+4+1+1),(y_next,y_next+4*cst::NB_PREVIEW+1)); 

}
