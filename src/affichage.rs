use crossterm::QueueableCommand;
use std::io;
//use std::io::{stdout, Write};
#[allow(unused_imports)]
use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetAttribute, SetBackgroundColor},
    terminal,
    terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

use std::thread;
use std::time::Duration;
use std::io::Write;
use crate::{game::Game, utils::{tetromino::{Forme, Tetromino}, direction::Direction, container::{Container, Div}}, systems::{polyomino_position::PolyominoPosition, grid::Grille}};
use crate::param_const::cst;
use crate::param_const::param;

fn to_ansi_color(col: &str) -> Color {
    Color::parse_ansi(&*format!("5;{}", col)).unwrap()
}

// impl Game {
//     pub fn ajoute_container(&mut self, c: Container) {
//         self.containers.push(c);
//     }
// }

impl Game {
    pub fn init_playground(&mut self) -> Result<()> {
        let mut w = &self.stdout;
        execute!(w, EnterAlternateScreen)?;
        terminal::enable_raw_mode()?;
        queue!(
            w,
            ResetColor,
            terminal::Clear(ClearType::All),
            cursor::Hide,
            cursor::MoveTo(1, 1)
        )?;
        for e in self.get_containers().iter() {
            let _ = e.init();
        }
        let _ = self.stdout.flush();
        thread::sleep(Duration::from_secs(4));
        Ok(())
    }

    pub fn quit_playground(&self) -> io::Result<()>{
        //thread::sleep(Duration::from_secs(7));
        let mut w = &self.stdout;
        queue!(w, ResetColor, terminal::Clear(ClearType::All), cursor::Show)?;
        terminal::disable_raw_mode()?;
        execute!(w, LeaveAlternateScreen)
    }
    
}


pub struct Grid {
    pub current_polyomino: PolyominoPosition,
    pub grid: Grille,
    pub contain: Container,
}
impl Div for Grid {
    fn get_container(&self) -> &Container {
        &self.contain
    }
    fn draw_inside(&self) -> io::Result<()> {
        let mut w = io::stdout();
        let g = &self.grid; // (20, 10) 'fin plutot (0-19,0-20)
        for y in 0..cst::NB_LIGNE_GRILLE as i16 { // 0 -> 19
            for x in 0..cst::NB_COLONNE_GRILLE as i16 { // 0 -> 9
                let cx = x as u16 + self.contain.co_x.0 + 1;
                let cy = y as u16 + self.contain.co_y.0 + 1;
                let case_color = to_ansi_color(g.get_case((y,x)).code_ansi());
                queue!(
                    w,
                    cursor::MoveTo(cx*2, cy),
                    SetBackgroundColor(case_color),
                    Print("  "),
                    SetBackgroundColor(Color::Reset)
                )?;
            }
        }
          

        let mut moke = self.current_polyomino.clone();
        let mut i = 0;
        while let Some(t) = moke.est_bougeable(Direction::D, &self.grid) {
            moke = t;
            i+=1;
        }

        for t in self.current_polyomino.to_coord() {
            let x = (t.1 as u16 + 1 + self.contain.co_x.0 )*2;
            let y = t.0 as u16 + 1 + self.contain.co_y.0;
            queue!(
                w,
                //preview
                cursor::MoveTo(x, y+i),
                SetBackgroundColor(to_ansi_color(param::COULEUR_PREVIEW)),
                Print("  "),
                //SetBackgroundColor(Color::Reset),
                //polyo
                cursor::MoveTo(x, y),
                SetBackgroundColor(to_ansi_color(self.current_polyomino.code_ansi())),
                Print("  "),
                SetBackgroundColor(Color::Reset)
            )?;
        }

        
    
        
        Ok(())
    }
}

pub struct Nexts {
    pub nexts: Box<Vec<Tetromino>>,
    pub contain: Container,
}
impl Div for Nexts {
    fn get_container(&self) -> &Container {
        &self.contain
    }
    fn draw_inside(&self) -> io::Result<()> {
        let mut w = io::stdout();
        for i in 0..cst::NB_PREVIEW as u16 {
            let tetro = self.nexts[i as usize];
            for t in tetro.to_coord() {
                let mut x = (t.1 as u16 + 1 + param::CONTAINER_NEXT.co_x.0 + 1)*2 ;
                let y = t.0 as u16 + 1 + param::CONTAINER_NEXT.co_y.0 + 1;
                if tetro.forme() == Forme::I || tetro.forme() == Forme::O {x -= 1} 
                queue!(
                    w,
                    cursor::MoveTo(x, i*4 + y),
                    SetBackgroundColor(to_ansi_color(tetro.code_ansi())),
                    Print("  "),
                    SetBackgroundColor(Color::Reset)
                )?;
            }
        }
        Ok(())
    }
}



pub struct Holder {
    pub holded: Option<Tetromino>,
    pub contain: Container,
}

impl Div for Holder {
    fn draw_inside(&self) -> io::Result<()> {
        let mut w = io::stdout();
        match &self.holded {
            None => Ok(()),
            Some(tetro) => {
                for t in tetro.to_coord() {
                    let mut x = (t.1 as u16 + 1 + param::CONTAINER_HOLD.co_x.0 + 1)*2;
                    if tetro.forme() == Forme::I || tetro.forme() == Forme::O { x -= 1} 
                    let y = t.0 as u16 + 1 + param::CONTAINER_HOLD.co_x.0 + 1;
                    queue!(
                        w,
                        cursor::MoveTo(x, y),
                        SetBackgroundColor(to_ansi_color(tetro.code_ansi())),
                        Print("  "),
                        SetBackgroundColor(Color::Reset)
                    )?;
                }
                Ok(())
            }
        }
    }
    fn get_container(&self) -> &Container {
        &self.contain
    }
}



// // GRILLE
// impl Game {
//     fn draw_borders(&self) -> io::Result<()> {
//         let mut w = &self.stdout;
//         let col_bord = to_ansi_color(param::COULEUR_BORD);
//         w.queue(SetBackgroundColor(col_bord))?;
//         self.draw_square(param::POSITION_GRILLE.0, param::POSITION_GRILLE.1)?;
//         w.queue(SetBackgroundColor(Color::Reset))?;
//         Ok(())
//     }

//     pub fn draw_grid(&self) -> io::Result<()> {
//         let mut w = &self.stdout;
//         let g = &self.g; // (20, 10) 'fin plutot (0-19,0-20)
//         let xs = param::POSITION_GRILLE.0;
//         for x in 0..cst::NB_COLONNE_GRILLE as i16 {
//             // 0 -> 9
//             for y in 0..cst::NB_LIGNE_GRILLE as i16 {
//                 // 0 -> 19
//                 let cx = (1 + x as u16)*2 + param::POSITION_GRILLE.0.0;
//                 let cy = 1 + y as u16 + param::POSITION_GRILLE.1.0;
//                 let case_color = to_ansi_color(g.get_case((y, x)).code_ansi());
//                 queue!(
//                     w,
//                     cursor::MoveTo(cx, cy),
//                     SetBackgroundColor(case_color),
//                     Print("  "),
//                     SetBackgroundColor(Color::Reset)
//                 )?;
//             }
//         }
//         Ok(())
//     }

//     pub fn draw_polyo(&self) -> io::Result<()> {
//         let mut w = &self.stdout;
//         for t in self.polyomino_current.to_coord() {
//             let x = (t.1 as u16 + 1)*2 + param::POSITION_GRILLE.0.0;
//             let y = t.0 as u16 + 1 + param::POSITION_GRILLE.1.0;
//             queue!(
//                 w,
//                 cursor::MoveTo(x, y),
//                 SetBackgroundColor(to_ansi_color(self.polyomino_current.code_ansi())),
//                 Print("  "),
//                 SetBackgroundColor(Color::Reset)
//             )?;
//         }
//         Ok(())
//     }

//     pub fn draw_prev(&self) -> io::Result<()> {
//         let mut w = &self.stdout;
//         let mut moke = self.polyomino_current.clone();
//         while true {
//             if let Some(t) = moke.est_bougeable(Direction::D, &self.g) {
//                 moke = t;
//             } else {
//                 break;
//             }
//         }
//         for t in moke.to_coord() {
//             let x = (t.1 as u16 + 1)*2 + param::POSITION_GRILLE.0.0;
//             let y = t.0 as u16 + 1 + param::POSITION_GRILLE.1.0;
//             queue!(
//                 w,
//                 cursor::MoveTo(x, y),
//                 SetBackgroundColor(to_ansi_color(param::COULEUR_PREVIEW)),
//                 Print("  "),
//                 SetBackgroundColor(Color::Reset)
//             )?;
//         }
//         Ok(())
//     }
// }


// // UTILS
// impl Game {
//     fn draw_square(&self, xs:(u16, u16), ys:(u16, u16),) -> io::Result<()> {
//         let mut w = &self.stdout;
//         let col_bord = to_ansi_color(param::COULEUR_TEST);
//         w.queue(SetBackgroundColor(col_bord))?;
//         for x in (xs.0..(xs.1) * 2).step_by(2) {
//             queue!(
//                 w,
//                 cursor::MoveTo(x, ys.0),
//                 Print("  ".to_string()),
//                 cursor::MoveTo(x, ys.1),
//                 Print("  ".to_string())
//             )?;
//         }
//         for y in ys.0..ys.1+1 {
//             queue!(
//                 w,
//                 cursor::MoveTo(xs.0, y),
//                 Print("  ".to_string()),
//                 cursor::MoveTo(xs.1*2, y),
//                 Print("  ".to_string())
//             )?;
//         }
//         w.queue(SetBackgroundColor(Color::Reset))?;
//         Ok(())
//     }
// }


// // NEXTS
// impl Game {
//     pub fn draw_nexts_window(&self) -> io::Result<()> {
//         self.draw_square(param::POSITION_NEXT.0, param::POSITION_NEXT.1)
//     }

//     pub fn draw_nexts(&self) -> io::Result<()>{
//         self.clear_nexts();
//         let mut w = &self.stdout;
//         for i in 0..7 as u16 {
//             let tetro = self.nexts[i as usize];
//             for t in tetro.to_coord() {
//                 let mut x = (t.1 as u16 + 1)*2 + param::POSITION_NEXT.0.0 + 2;
//                 if tetro.forme() == Forme::I { x -= 1} 
//                 let y = t.0 as u16 + 1 + param::POSITION_NEXT.1.0 + 1;
//                 queue!(
//                     w,
//                     cursor::MoveTo(x, i*4 + y),
//                     SetBackgroundColor(to_ansi_color(tetro.code_ansi())),
//                     Print("  "),
//                     SetBackgroundColor(Color::Reset)
//                 )?;
//             }
//         }
//         Ok(())
//     }

//     pub fn clear_nexts(&self) -> io::Result<()> {
//         let mut w = &self.stdout;
//         for x in param::POSITION_NEXT.0.0*2..param::POSITION_NEXT.0.1*2 {
//             for y in param::POSITION_NEXT.1.0..param::POSITION_NEXT.1.1 {
//                 queue!(
//                     w,
//                     cursor::MoveTo(x*2 + param::POSITION_NEXT.0.0 + 2, y + param::POSITION_NEXT.1.0 + 1),
//                     SetBackgroundColor(to_ansi_color(param::COULEUR_)),
//                     Print("  "),
//                     SetBackgroundColor(Color::Reset)
//                 )?; 
//             }
//         }
//         Ok(())
//     }
// }


// // HOLD
// impl Game {
//     pub fn draw_hold(&self) -> io::Result<()> {
//         let c_hold = Container::default((0,5),(0,5));
//         c_hold.draw()?;

//         self.draw_square(param::POSITION_HOLD.0, param::POSITION_HOLD.1)?;
//         Ok(())
//     }
//     pub fn draw_holded(&self) -> io::Result<()> {
//         self.clean_holded()?;
//         let mut w = &self.stdout;
//         match &self.holded {
//             None => Ok(()),
//             Some(tetro) => {
//                 for t in tetro.to_coord() {
//                     let mut x = (t.1 as u16 + 1)*2 + param::POSITION_HOLD.0.0 + 1;
//                     if tetro.forme() == Forme::I { x -= 1} 
//                     let y = t.0 as u16 + 1 + param::POSITION_HOLD.1.0 + 1;
//                     queue!(
//                         w,
//                         cursor::MoveTo(x, y),
//                         SetBackgroundColor(to_ansi_color(tetro.code_ansi())),
//                         Print("  "),
//                         SetBackgroundColor(Color::Reset)
//                     )?;
//                 }
//                 Ok(())
//             }
//         }
//     }
//     pub fn clean_holded(&self) -> io::Result<()> {
//         let mut w = &self.stdout;
//         for x in 0..4 {
//             for y in 0..4 {
//                 queue!(
//                     w,
//                     cursor::MoveTo(x*2 + param::POSITION_HOLD.0.0 + 2, y + param::POSITION_HOLD.1.0 + 1),
//                     SetBackgroundColor(to_ansi_color(param::COULEUR_)),
//                     Print("  "),
//                     SetBackgroundColor(Color::Reset)
//                 )?; 
//             }
//         }
//         Ok(())
//     }
// }
