
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
//use rand::random;

#[allow(unused_imports)]
use std::io::Stdout;
use std::time::{Duration, Instant};

const MAX_INTERVAL: u16 = 700;
const MIN_INTERVAL: u16 = 200;
const MAX_SPEED: u16 = 20;

use crate::{utils::{direction::Direction, container::ContainTrait, tetromino::{Tetromino, Forme}}, systems::{playground::{data::Playground, grid::Grid, polyomino_position::PolyominoPosition}, next::data::Nexts, hold::data::Hold, mechanics::{colision::est_valide, randomness::PolyominoBag}}, param_const::{param, cst}};
pub enum Commandes {
    Quit,
    Tourne(Direction),
    Bouge(Direction),
    Hold,
    Quick,
}


// #[derive(Debug)]
// original_terminal_size: (u16, u16),
pub struct Game {
    pub stdout: Stdout,
    pub grid_container: Playground,
    pub nexts_container: Nexts,
    pub hold_container: Hold,
    speed: u16,
    score: u16,
}

// RUNNING CODE
impl Game {
    pub fn get_containers(&self) -> Vec<&dyn ContainTrait>{
        vec![&self.grid_container, &self.nexts_container, &self.hold_container]
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
                            if let Some(tetro) = &self.hold_container.holded_polyomino {
                                let temp = self.grid_container.current_polyomino.polyomino.set_to_init_rotation();
                                self.grid_container.current_polyomino = PolyominoPosition::from(*tetro);
                                self.hold_container.holded_polyomino = Some(temp);
                            } else {
                                self.hold_container.holded_polyomino = Some(self.grid_container.current_polyomino.polyomino);
                                self.grid_container.current_polyomino = self.get_prochain();
                            }
                        },
                        Commandes::Quick => {
                            self.grid_container.current_polyomino.org.0 += self.grid_container.get_distance_preview_polyomino() as i16;
                            prec_mvnt = true;
                        },
                    };
                    _mwnt_auto = false;

                } else {
                    self.bouge_polyo(Direction::D);
                    _mwnt_auto = true;
                }

                
                match self.grid_container.current_polyomino.est_bougeable(Direction::D, &self.grid_container.grid){
                    None if prec_mvnt => {
                        self.grid_container.grid.pose_polyomino(&self.grid_container.current_polyomino);
                        self.ajoute_poly_queue();
                        self.grid_container.current_polyomino = self.get_prochain();
                        prec_mvnt = false;
                        if !est_valide(&self.grid_container.grid, &self.grid_container.current_polyomino) {
                            println!("Game Over !!");
                            done = true;
                            break;
                        }
                    },
                    None => { prec_mvnt = true; now = Instant::now();}
                    _ => {},
                };
                
                self.enleve_lignes();
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
        let c_grille = Playground {
            current_polyomino: PolyominoPosition::rand(),
            grid: Grid::new(),
            contain: param::CONTAINER_GRID,
        };

        let nexts: Box<Vec<Tetromino>> = Box::new(Vec::with_capacity(cst::NB_BAG+1));
        let c_nexts: Nexts = Nexts {
            nexts: nexts,
            contain: param::CONTAINER_NEXT,
        };

        let c_hold = Hold {
            holded_polyomino: None,
            contain: param::CONTAINER_HOLD,
        };


        //let original_terminal_size: (u16, u16) = size().unwrap();
        let mut init_game = Self {
            stdout: io::stdout(),
            //original_terminal_size: original_terminal_size,

            nexts_container: c_nexts ,
            hold_container: c_hold,
            grid_container: c_grille,
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
        self.nexts_container.nexts.len() % Forme::COUNT
    }

    fn get_prochain(&mut self) -> PolyominoPosition {
        let res_t: Tetromino = self.nexts_container.nexts.remove(0);
        PolyominoPosition::from(res_t)
    }
}

// PROCEDURES
impl Game {
    fn bouge_polyo(&mut self, dir: Direction) -> () {
        if let Some(p) = self.grid_container.current_polyomino.est_bougeable(dir, &self.grid_container.grid) {
            self.grid_container.current_polyomino = p;
        }
    }

    fn tourne_polyo(&mut self, dir: Direction) {
        if let Some(p) = self.grid_container.current_polyomino.est_tournable(dir, &self.grid_container.grid) {
            self.grid_container.current_polyomino = p;
        }
    }

    fn enleve_lignes(&mut self) {
        if let Some(t) = self.grid_container.grid.clear_lines() {
            self.score += t;
        }
    }

    fn ajoute_poly_queue(&mut self) {
        if self.get_num_bag() <= cst::NB_BAG {
            let bag: PolyominoBag = PolyominoBag::new();
            bag.into_iter().for_each(|a: Tetromino| self.nexts_container.nexts.push(a));
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
            KeyCode::Char(' ') => Some(Commandes::Quick),

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
