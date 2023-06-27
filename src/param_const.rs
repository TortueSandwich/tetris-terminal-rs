pub mod cst {
    pub const NB_LIGNE_GRILLE: u16 = 20;
    pub const NB_COLONNE_GRILLE: u16 = 10;
    pub const NB_PREVIEW: usize = 4;
    pub const NB_BAG: usize = 2;
}

pub mod param {
    use crossterm::style::Color;

    use crate::utils::container::Container;

    use super::cst;

    pub const COULEUR_BORD: Color = Color::AnsiValue(107);

    pub const COULEUR_L: Color = Color::AnsiValue(75);
    pub const COULEUR_J: Color = Color::AnsiValue(191);
    pub const COULEUR_S: Color = Color::AnsiValue(118);
    pub const COULEUR_Z: Color = Color::AnsiValue(160);
    pub const COULEUR_I: Color = Color::AnsiValue(14);
    pub const COULEUR_T: Color = Color::AnsiValue(5);
    pub const COULEUR_O: Color = Color::AnsiValue(11);

    pub const COULEUR_PREVIEW: Color = Color::AnsiValue(235);
    pub const COULEUR_: Color = Color::AnsiValue(95);

    #[allow(unused)]
    pub const COULEUR_TEST: Color = Color::AnsiValue(1);

    pub const CONTAINER_HOLD: Container = Container::one_tetro_holder(0, 0);
    pub const CONTAINER_GRID: Container = Container::default((8, 20), (0, 22));
    pub const CONTAINER_NEXT: Container =
        Container::default((24, 24 + 6), (0, cst::NB_PREVIEW as u16 * 4 + 2));
    
    pub const CONTAINER_SCORE: Container = Container::default((0, 9), (8, 15));
}
