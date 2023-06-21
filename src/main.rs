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
}
mod affichage;

//use crossterm::{style::SetAttribute, terminal};
// use std::io;

fn main() {
    let mut game = game::Game::new();
    let _ = game.run();
    //crate::game::Game::new(stdout(), 10, 20).run();
}
