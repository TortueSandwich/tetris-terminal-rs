use crate::affichage::{Grid, Holder, Nexts};
use crate::param_const::param;
use crate::utils::container::{Container, Div};
use crate::{param_const::cst, systems::colision};
use crate::systems::grid::Grille;
use std::io;
use std::io::Write;

#[allow(unused_imports)]
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    event::{poll, read, Event, KeyCode, KeyEvent, KeyModifiers},
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType, SetSize},
    ExecutableCommand,
};
use rand::random;

#[allow(unused_imports)]
use std::io::Stdout;
use std::time::{Duration, Instant};

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

use crate::utils::direction::Direction;
pub enum Commandes {
    Quit,
    Tourne(Direction),
    Bouge(Direction),
    Hold,
}

use crate::systems::polyomino_position::PolyominoPosition;
use crate::utils::tetromino::{Tetromino, Forme};

// #[derive(Debug)]
// original_terminal_size: (u16, u16),
pub struct Game {
    pub stdout: Stdout,
    pub g: Grid,
    pub nexts: Nexts,
    pub holded: Holder,
    speed: u16,
    score: u16,
}

// RUNNING CODE
impl Game {
    pub fn get_containers(&self) -> Vec<&dyn Div>{
        vec![&self.g, &self.nexts, &self.holded]
    }

    pub fn run(&mut self) -> io::Result<()> {
        



        let mut done = false;
        let mut _mwnt_auto = false;
        let mut prec_mvnt = false;
        self.init_playground()?;
        while !done {
            let interval = self.calculate_interval();
            let mut now = Instant::now();
            while now.elapsed() < interval {
                if let Some(command) = self.get_command(interval - now.elapsed()) {
                    prec_mvnt = false;
                    match command {
                        Commandes::Tourne(t) => match t {
                            Direction::R | Direction::L => self.tourne_polyo(t),
                            _ => unreachable!("Tourne à echoué"),
                        },
                        Commandes::Bouge(t) => self.bouge_polyo(t),
                        Commandes::Quit => {
                            done = true;
                            break;
                        },
                        Commandes::Hold => {
                            if let Some(tetro) = &self.holded.holded {
                                let temp = self.g.current_polyomino.polyomino.set_to_init_rotation();
                                self.g.current_polyomino = PolyominoPosition::from(*tetro);
                                self.holded.holded = Some(temp);
                            } else {
                                self.holded.holded = Some(self.g.current_polyomino.polyomino);
                                self.g.current_polyomino = self.get_prochain();
                            }
                        },
                    };
                    _mwnt_auto = false;

                } else {
                    self.bouge_polyo(Direction::D);
                    _mwnt_auto = true;
                }

                
                match self.g.current_polyomino.est_bougeable(Direction::D, &self.g.grid){
                    None if prec_mvnt => {
                        self.g.grid.pose_polyomino(&self.g.current_polyomino);
                        self.ajoute_poly_queue();
                        self.g.current_polyomino = self.get_prochain();
                        prec_mvnt = false;
                        if !colision::est_valide(&self.g.grid, &self.g.current_polyomino) {
                            println!("Game Over !!");
                            done = true;
                            break;
                        }
                    },
                    None => { prec_mvnt = true; now = Instant::now();}
                    _ => {},
                };
                
                for e in self.get_containers().into_iter() {
                    e.update()?;
                }
                self.stdout.flush()?;
            }
        }
        self.quit_playground()
    }
}



impl Game {
    pub fn new() -> Self {
        let c_grille = Grid{
            current_polyomino: PolyominoPosition::rand(),
            grid: Grille::new(),
            contain: param::CONTAINER_GRID,
        };

        let nexts: Box<Vec<Tetromino>> = Box::new(Vec::with_capacity(cst::NB_BAG+1));
        let c_nexts: Nexts = Nexts {
            nexts: nexts,
            contain: param::CONTAINER_NEXT,
        };

        let c_hold = Holder {
            holded: None,
            contain: param::CONTAINER_HOLD,
        };


        //let original_terminal_size: (u16, u16) = size().unwrap();
        let mut init_game = Self {
            stdout: io::stdout(),
            //original_terminal_size: original_terminal_size,

            nexts: c_nexts ,
            holded: c_hold,
            g: c_grille,
            speed: 0,
            score: 0,
            //containers: Box::new(vec![&g, &nexts, &holded]),
        };
        
        // init_game.containers.push(&init_game.g);
        // init_game.containers.push(&init_game.nexts);
        // init_game.containers.push(&init_game.holded);
        

        for _ in 0..cst::NB_BAG {
            init_game.ajoute_poly_queue();
        } 
        init_game.get_prochain();
        init_game
    }
}




// SPECIALS_GETTERS
impl Game {
    fn get_num_bag(&self) -> usize {
        self.nexts.nexts.len() % Forme::COUNT
    }

    fn get_prochain(&mut self) -> PolyominoPosition {
        let res_t: Tetromino = self.nexts.nexts.remove(0);
        PolyominoPosition::from(res_t)
    }
}

// PROCEDURES
impl Game {
    fn bouge_polyo(&mut self, dir: Direction) -> () {
        if let Some(p) = self.g.current_polyomino.est_bougeable(dir, &self.g.grid) {
            self.g.current_polyomino = p;
        }
    }

    fn tourne_polyo(&mut self, dir: Direction) {
        if let Some(p) = self.g.current_polyomino.est_tournable(dir, &self.g.grid) {
            self.g.current_polyomino = p;
        }
    }

    fn enleve_lignes(&mut self) {
        if let Some(t) = self.g.grid.clear_lines() {
            self.score += t;
        }
    }

    fn ajoute_poly_queue(&mut self) {
        use super::systems::randomness;
        if self.get_num_bag() <= cst::NB_BAG {
            let bag: randomness::PolyominoBag = randomness::new();
            bag.into_iter().for_each(|a: Tetromino| self.nexts.nexts.push(a));
        }
    }
}





// INTERACTION  H -> C
impl Game {
    fn get_command(&self, wait_for: Duration) -> Option<Commandes> {
        let key_event = self.wait_for_key_event(wait_for)?;

        match key_event.code {
            KeyCode::Down => Some(Commandes::Bouge(Direction::D)),
            KeyCode::Up => Some(Commandes::Bouge(Direction::U)),
            KeyCode::Right => Some(Commandes::Bouge(Direction::R)),
            KeyCode::Left => Some(Commandes::Bouge(Direction::L)),

            KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => Some(Commandes::Quit),
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if key_event.modifiers == KeyModifiers::CONTROL {
                    Some(Commandes::Quit)
                } else {
                    None
                }
            }
            KeyCode::Char('z') => Some(Commandes::Tourne(Direction::R)),
            KeyCode::Char('a') => Some(Commandes::Tourne(Direction::L)),
            KeyCode::Char('d') => Some(Commandes::Hold),
            _ => None,
        }
    }

    fn calculate_interval(&self) -> Duration {
        let speed = MAX_SPEED - self.speed;
        Duration::from_millis(
            (MIN_INTERVAL + (((MAX_INTERVAL - MIN_INTERVAL) / MAX_SPEED) * speed)) as u64,
        )
    }

    fn wait_for_key_event(&self, wait_for: Duration) -> Option<KeyEvent> {
        if poll(wait_for).ok()? {
            let event = read().ok()?;
            if let Event::Key(key_event) = event {
                return Some(key_event);
            }
        }
        None
    }
}
