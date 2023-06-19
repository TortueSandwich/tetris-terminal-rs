mod rotations_const;
use super::direction::Direction;
use rand;
use rotations_const::{AbsCoord, RotationArray};

#[derive(Clone, Copy, PartialEq)]
pub enum Forme {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}
impl Forme {
    pub const COUNT: usize = 7;

    fn rand() -> Forme {
        use Forme::*;
        match rand::random::<usize>() % Forme::COUNT {
            0 => L,
            1 => J,
            2 => S,
            3 => Z,
            4 => I,
            5 => T,
            6 => O,
            _ => unreachable!(
                "Impossible, Verifiez que Forme::COUNT soit bien correct. Valeur actuelle {}",
                Forme::COUNT
            ),
        }
    }
}

#[derive(Clone, Copy)]
pub struct Tetromino {
    f: Forme,
    coordonne: RotationElement,
}

impl From<Forme> for Tetromino {
    fn from(value: Forme) -> Self {
        Tetromino {
            f: value,
            coordonne: RotationElement::from(value),
        }
    }
}

impl Tetromino {
    pub fn rand() -> Self {
        let f: Forme = Forme::rand();
        Tetromino::from(f)
    }

    pub fn code_ansi(&self) -> &str {
        use crate::param_const::param::*;
        match self.f {
            Forme::L => COULEUR_L,
            Forme::J => COULEUR_J,
            Forme::S => COULEUR_S,
            Forme::Z => COULEUR_Z,
            Forme::I => COULEUR_I,
            Forme::T => COULEUR_T,
            Forme::O => COULEUR_O,
        }
    }

    pub fn set_to_init_rotation(&mut self) -> Tetromino {
        self.coordonne.current_value = 0;
        *self
    }
}

#[derive(Clone, Copy)]
struct RotationElement {
    current_value: usize,
    arr_rotation: &'static RotationArray,
}

impl From<&'static RotationArray> for RotationElement {
    fn from(arr: &'static RotationArray) -> Self {
        RotationElement {
            current_value: 0,
            arr_rotation: arr,
        }
    }
}
impl From<Forme> for RotationElement {
    fn from(value: Forme) -> Self {
        use rotations_const as rotation_array;
        let arr: &'static RotationArray = match value {
            Forme::I => &rotation_array::I,
            Forme::J => &rotation_array::J,
            Forme::L => &rotation_array::L,
            Forme::O => &rotation_array::O,
            Forme::S => &rotation_array::S,
            Forme::T => &rotation_array::T,
            Forme::Z => &rotation_array::Z,
        };
        RotationElement::from(arr)
    }
}

impl RotationElement {
    fn to_vec(&self) -> Vec<AbsCoord> {
        self.arr_rotation[self.current_value].to_vec()
    }
    fn next(&self) -> RotationElement {
        let mut res = self.clone();
        res.current_value = (res.current_value + 1) % 4;
        res
    }
    fn prev(&self) -> RotationElement {
        let mut res = self.clone();
        res.current_value = (res.current_value + 3) % 4;
        res
    }
}

impl Tetromino {
    pub fn rotate(&mut self, dir: Direction) -> Result<(), String> {
        use Direction::*;
        match dir {
            L => {
                let prev = self.coordonne.prev();
                self.coordonne = prev;
                Ok(())
            }
            R => {
                let next = self.coordonne.next();
                self.coordonne = next;
                Ok(())
            }
            _ => Err(format!(
                "La direction entrée n'est pas acceptée lors d'une rotation : {:?}",
                dir
            )),
        }
    }

    pub fn to_coord(&self) -> Vec<(i16, i16)> {
        self.coordonne
            .to_vec()
            .iter()
            .map(|c| (c.0 as i16, c.1 as i16))
            .collect()
    }

    pub fn forme(&self) -> Forme {
        self.f
    }
}
