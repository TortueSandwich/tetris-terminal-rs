mod game;
mod param_const;
mod systems {
    mod ai;
    pub mod colision;
    pub mod grid;
    pub mod polyomino_position;
    pub mod randomness;
}
mod utils {
    pub mod direction;
    pub mod tetromino;
    pub mod container;
}
mod affichage;

//use crossterm::{style::SetAttribute, terminal};
use std::io;

fn main() {
    let mut game = game::Game::new();
    game.run();
    //crate::game::Game::new(stdout(), 10, 20).run();
}
