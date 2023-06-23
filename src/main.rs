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

}
mod utils {
    pub mod direction;
    pub mod tetromino;
    pub mod container;
    pub mod writer;
    pub mod coord;
    pub mod show_srs;
}
mod affichage;
mod erreur;

//use crossterm::{style::SetAttribute, terminal};
// use std::io;

use std::env;



fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}",args);
    

    let mut game = game::Game::new();
    if let Err(e) = game.run() {
        let _ = game.quit_playground();
        panic!("{e}");
    }
}
