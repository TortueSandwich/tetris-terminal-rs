use core::num;
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

use crate::{
    param_const::{cst, param},
    systems::{
        hold::data::Hold,
        mechanics::{colision::est_valide, randomness::PolyominoBag},
        next::data::Nexts,
        playground::{data::Playground, grid::Grid, polyomino_position::PolyominoPosition}, scoring::data::ScoringSys,
    },
    utils::{
        direction::{Direction, Rotation},
        tetromino::{Forme, Tetromino},
    },
};
pub enum Commandes {
    Quit,
    Tourne(Rotation),
    Bouge(Direction),
    Hold,
    InstantDrop,
}

// #[derive(Debug)]
// original_terminal_size: (u16, u16),
pub struct Game {
    pub stdout: Stdout,
    pub grid_container: Playground,
    pub nexts_container: Nexts,
    pub hold_container: Hold,
    pub score_sys: ScoringSys,
    speed: u16,
}

// RUNNING CODE
impl Game {
    pub fn run(&mut self) -> io::Result<()> {
        self.init_playground()?;

        let mut game_is_done: bool = false;



        let mut last_move_was_instant_drop = false;
        let mut has_wait_before_placing = false;

        while !game_is_done {
            let interval = self.calculate_interval();
            let mut now = Instant::now();


            while now.elapsed() < interval {
                // command handler
                if let Some(command) = self.get_command(interval - now.elapsed()) {
                    has_wait_before_placing = false;

                    match command {
                        Commandes::Tourne(r) => self.perform_rotation(r),
                        Commandes::Bouge(d) => self.perform_movement(d),
                        Commandes::Quit => {
                            game_is_done = true;
                            break;
                        },
                        Commandes::Hold => self.perform_hold(),
                        Commandes::InstantDrop => {
                            //self.score_sys.add_score(self.grid_container.get_distance_preview_polyomino() as u128 * 2);
                            self.perform_instant_drop();
                            last_move_was_instant_drop = true;
                        }
                    };
                } else {
                    // self.score_sys.add_score(1);
                    self.perform_movement(Direction::D);
                }

                // Placing
                let cannot_move_down: bool = self.cannot_go_lower();
                if cannot_move_down {                    
                    if has_wait_before_placing || last_move_was_instant_drop {

                        self.score_sys.check_tspin(&self.grid_container.current_polyomino, &self.grid_container.grid);
                        let _ = self.perform_place();
                        if self.is_game_over() {
                            game_is_done = true;
                            self.perform_game_over();
                            break;
                        }
                    } else {
                        has_wait_before_placing = true;
                        now = Instant::now();
                    }
                };

                last_move_was_instant_drop = false;
                
                self.perform_clear_and_scoring();
                self.update()?;
            }
            
        }
        self.quit_playground()
    }
}

impl Game {
    pub fn new() -> Self {
        let c_grille = Playground::new();
        let c_nexts = Nexts::new();
        let c_hold = Hold::new();

        let mut init_game = Self {
            stdout: io::stdout(),
            nexts_container: c_nexts,
            hold_container: c_hold,
            grid_container: c_grille,
            speed: 1,
            score_sys: ScoringSys::new(),
            //containers: Box::new(vec![&g, &nexts, &holded]),
        };

        init_game.next_tetromino();
        init_game
    }
}

impl Game {
    fn perform_rotation(&mut self, r : Rotation) {
        if let Some(p) = self
            .grid_container
            .current_polyomino
            .est_tournable(r, &self.grid_container.grid)
        {
            // if p.polyomino.is_t() {
            //     self.score_sys.check_tspin(&p, &self.grid_container.grid);
            // }
            self.grid_container.current_polyomino = p;
        }
    }

    fn perform_movement(&mut self, dir: Direction) -> () {
        if let Some(p) = self
            .grid_container
            .current_polyomino
            .est_bougeable(dir, &self.grid_container.grid)
        {
            self.grid_container.current_polyomino = p;
        }
    }

    fn perform_hold(&mut self) {
        let curr_p = self.grid_container.get_polyomino();
        if let Some(holded_p) = self.hold_container.switch_with(Some(curr_p)){
            self.grid_container.set_polyomino(holded_p);
        } else {
            self.next_tetromino();
        }
    }

    fn perform_instant_drop(&mut self) {
        let preview_polyo_y = self.grid_container.get_distance_preview_polyomino();
        self.grid_container.current_polyomino.org.0 += preview_polyo_y as i16;
    }

    fn perform_place(&mut self) -> Result<(), crate::erreur::InvalidCoordinatesError>{
        self.grid_container.place_on_grid()?;
        self.next_tetromino();
        Ok(())
    }
    fn perform_game_over(&self) {
        
    }

    fn perform_clear_and_scoring(&mut self) {
        let num_line_cleared: u16 = self.grid_container.grid.clear_lines();
        if num_line_cleared != 0 {
            let scr_value: u128 = self.score_sys.move_score(num_line_cleared, self.speed);

            self.score_sys.add_score(scr_value);
        }
    }
}

impl Game {
    fn cannot_go_lower(&self) -> bool {
        self.grid_container.current_polyomino.est_bougeable(Direction::D, &self.grid_container.grid).is_none()
    }

    fn is_game_over(&self) -> bool {
        !est_valide(
            &self.grid_container.grid,
            &self.grid_container.current_polyomino,
        )
    }
}

// SPECIALS_GETTERS
impl Game {
    // fn get_num_bag(&self) -> usize {
    //     self.nexts_container.nexts.len() % Forme::COUNT
    // }

    fn next_tetromino(&mut self) {
        let t = match self.nexts_container.take_next(){
            Err(e) => panic!("{e}"),
            Ok(t) => PolyominoPosition::from(t),
        };
        self.grid_container.set_polyomino(t.polyomino);
    }
}

// PROCEDURES
impl Game {




    // fn ajoute_poly_queue(&mut self) {
    //     if self.get_num_bag() <= cst::NB_BAG {
    //         let bag: PolyominoBag = PolyominoBag::new();
    //         bag.into_iter()
    //             .for_each(|a: Tetromino| self.nexts_container.nexts.push(a));
    //     }
    // }
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
            KeyCode::Char('z') => Some(Commandes::Tourne(Rotation::ClockWise)),
            KeyCode::Char('a') => Some(Commandes::Tourne(Rotation::CounterClockWise)),
            KeyCode::Char('d') => Some(Commandes::Hold),
            KeyCode::Char(' ') => Some(Commandes::InstantDrop),

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
