use crate::utils::{tetromino::Tetromino, container::*};

pub struct Nexts {
    pub nexts: Box<Vec<Tetromino>>,
    pub contain: Container,
}