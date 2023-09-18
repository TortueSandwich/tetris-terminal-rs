mod rotations_const;
use super::direction::Rotation;
use rand;
use rotations_const::{AbsCoord, RotationArray};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Forme { I, J, L, O, S, T, Z, }
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

    pub fn color(&self) -> crossterm::style::Color {
        use crate::param_const::param::*;
        match self {
            Forme::L => COULEUR_L,
            Forme::J => COULEUR_J,
            Forme::S => COULEUR_S,
            Forme::Z => COULEUR_Z,
            Forme::I => COULEUR_I,
            Forme::T => COULEUR_T,
            Forme::O => COULEUR_O,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tetromino {
    f: Forme,
    pub coordonne: RotationElement,
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

    pub fn color(&self) -> crossterm::style::Color {
        self.f.color()
    }

    pub fn with_init_rotation(&mut self) -> Tetromino {
        self.coordonne.current_value = 0;
        *self
    }
}

#[derive(Clone, Copy, Debug)]
pub struct RotationElement {
    pub current_value: usize,
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
    pub fn basic_rotation(&mut self, r: Rotation) -> Result<(), String> {
        use Rotation::*;
        match r {
            CounterClockWise => {
                let prev = self.coordonne.prev();
                self.coordonne = prev;
                Ok(())
            }
            ClockWise => {
                let next = self.coordonne.next();
                self.coordonne = next;
                Ok(())
            }
        }
    }

    pub fn set_default_pos(&mut self) {
        self.coordonne.current_value = 0;
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

    pub fn reset_rotation(&mut self) {
        self.coordonne.current_value = 0;
    }

    pub fn is_t(&self) -> bool {
        self.forme() == Forme::T
    }
}
