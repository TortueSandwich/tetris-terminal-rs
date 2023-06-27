use crate::{utils::{direction::Rotation, container::Container}, systems::playground::{polyomino_position::PolyominoPosition, grid::Grid}, param_const::param::CONTAINER_SCORE};



#[derive(PartialEq, Clone, Debug)]
pub enum MoveStyle { 
    Nothing,
    Single, MiniTspin,
    MiniTspinSingle,
    Double,
    Tspin, MiniTspinDouble,
    Triple,
    B2BMiniTspinDouble,
    Tetris, TspinSingle,
    B2BTspinSingle, B2BTetris, TspinDouble,
    TspinTriple,
    B2BTspinDouble,
    B2BTspinTriple,
}

impl MoveStyle {
    pub fn score(&self) -> u128 {
        match *self {
            MoveStyle::Nothing => 0,
            MoveStyle::Single | MoveStyle::MiniTspin => 100,
            MoveStyle::MiniTspinSingle => 200,
            MoveStyle::Double => 300,
            MoveStyle::Tspin | MoveStyle::MiniTspinDouble => 400,
            MoveStyle::Triple => 500,
            MoveStyle::B2BMiniTspinDouble => 600,
            MoveStyle::Tetris | MoveStyle::TspinSingle => 800,
            MoveStyle::B2BTspinSingle | MoveStyle::B2BTetris | MoveStyle::TspinDouble => 1200,
            MoveStyle::TspinTriple => 1600,
            MoveStyle::B2BTspinDouble => 1800,
            MoveStyle::B2BTspinTriple => 2400,
        }
    }
}

#[derive(PartialEq, Clone)]
enum TspinType {NotSpinned, MiniTspin, Tspin}
impl TspinType {
    fn add_turn(&mut self) {
        let new_tspin = match *self {
            TspinType::NotSpinned => TspinType::MiniTspin,
            TspinType::MiniTspin => TspinType::Tspin,
            _ => self.clone(),
        };
        *self = new_tspin;
    }
}

pub struct ScoringSys {
    current_score : u128,
    pub current_move : MoveStyle,
    last_move : MoveStyle,
    tspin : TspinType,
    pub combo :bool,
    pub container :Container,
}

impl ScoringSys {
    pub fn new() -> Self {
        ScoringSys { current_score:0, current_move: MoveStyle::Nothing, last_move: MoveStyle::Nothing, tspin: TspinType::NotSpinned, combo: false, container:CONTAINER_SCORE}
    }

    pub fn get_score(&self) -> u128 {
        self.current_score
    }



    pub fn check_tspin(&mut self, p: &PolyominoPosition, g :&Grid) {

        if p.est_bougeable(crate::utils::direction::Direction::U, g).is_none()  {
            if p.est_tournable(Rotation::ClockWise, g).is_some() {
                self.tspin = TspinType::Tspin;
            }
            self.tspin = TspinType::MiniTspin;
        } else {
            self.tspin = TspinType::NotSpinned;
        }
    }

    pub fn add_score(&mut self, val : u128) {
        self.current_score += val;
    }

    pub fn move_score(&mut self, nb_line_cleared: u16, level : u16) -> u128 {
        self.set_current_move(nb_line_cleared);
        let level: u128 = level as u128;
        let mut res_value = self.current_move.score();
        if self.combo {
            res_value += 50; 
        }
        res_value * level
    }

    pub fn set_current_move(&mut self, nb_line_cleared: u16) {
        self.last_move = self.current_move.clone();
        self.current_move = match nb_line_cleared {
            0 => {
                match self.tspin {
                    TspinType::NotSpinned => self.current_move.clone(),
                    TspinType::MiniTspin => MoveStyle::MiniTspin,
                    TspinType::Tspin => MoveStyle::Tspin,
                }
            },
            1 => {
                match self.tspin {
                    TspinType::NotSpinned => MoveStyle::Single,
                    TspinType::MiniTspin => MoveStyle::MiniTspinSingle,
                    TspinType::Tspin => MoveStyle::TspinSingle,
                }
            },
            2 => {
                match self.tspin {
                    TspinType::NotSpinned => MoveStyle::Double,
                    TspinType::MiniTspin => MoveStyle::MiniTspinDouble,
                    TspinType::Tspin => MoveStyle::TspinDouble,
                }
            },
            3 => {
                match self.tspin {
                    TspinType::NotSpinned => MoveStyle::Triple,
                    TspinType::MiniTspin => unreachable!(),
                    TspinType::Tspin => MoveStyle::TspinTriple,
                }
            },
            4 => {
                MoveStyle::Tetris
            },
            _ => unreachable!()
        };
        self.combo = self.last_move == self.current_move && self.current_move != MoveStyle::Nothing;
        if self.combo {
            self.current_move = match self.current_move {
                MoveStyle::MiniTspinDouble => {
                    self.combo = false;
                    MoveStyle::B2BMiniTspinDouble
                }
                MoveStyle::TspinSingle => {
                    self.combo = false;
                    MoveStyle::B2BTspinSingle
                }
                MoveStyle::Tetris => {
                    self.combo = false;
                    MoveStyle::B2BTetris
                }
                MoveStyle::TspinDouble => {
                    self.combo = false;
                    MoveStyle::B2BTspinDouble
                }
                MoveStyle::TspinTriple => {
                    self.combo = false;
                    MoveStyle::B2BTspinTriple
                }
                _ => self.current_move.clone(),
            }
        }
        self.tspin = TspinType::NotSpinned;
    }

    
}