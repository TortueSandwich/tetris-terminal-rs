mod game;
mod param_const;
mod systems {

    pub mod playground {
        pub mod data;
        pub mod display;
        pub mod grid;
        pub mod polyomino_position;
    }
    pub mod hold {
        pub mod data;
        pub mod display;
    }
    pub mod next {
        pub mod data;
        pub mod display;
    }
    
    pub mod mechanics {
        pub mod colision;
        pub mod randomness;
        pub mod rotation;
    }
    
    pub mod scoring {
        pub mod data;
        pub mod display;
    }
}
mod utils {
    pub mod container;
    pub mod coord;
    pub mod direction;
    pub mod show_srs;
    pub mod tetromino;
    pub mod writer;
}
mod affichage;
mod erreur;

//use crossterm::{style::SetAttribute, terminal};
// use std::io;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let mut game = game::Game::new();
    if let Err(e) = game.run() {
        let _ = game.quit_playground();
        panic!("{e}");
    }
}
