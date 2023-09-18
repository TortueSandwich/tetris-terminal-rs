use crossterm::{
    cursor::{self, MoveTo}, queue,
    style::{Color, Print, SetBackgroundColor, ResetColor}, Command, terminal::{EnterAlternateScreen, self, LeaveAlternateScreen},
};
use std::io;

use crate::param_const::param;

#[allow(unused)]
#[derive(Debug)]
pub struct Container {
    pub co_x: (u16, u16),
    pub co_y: (u16, u16),
    couleur_bordure: Color,
    pub couleur_fond: Color,
}

impl Container {
    #[allow(unused)]
    pub const fn new(x: (u16, u16), y: (u16, u16), couleur_bordure: Color) -> Container {
        Container {
            co_x: x,
            co_y: y,
            couleur_bordure,
            couleur_fond: param::COULEUR_,
        }
    }
    #[allow(unused)]
    pub const fn default(x: (u16, u16), y: (u16, u16)) -> Container {
        Container {
            co_x: x,
            co_y: y,
            couleur_bordure: param::COULEUR_BORD,
            couleur_fond: param::COULEUR_,
        }
    }

    pub const fn one_tetro_holder(x: u16, y: u16) -> Container {
        Container::default((x, x + 6), (y, y + 6))
    }
}

impl Container {
    #[allow(unused)]
    fn draw_square(&self, xs: (u16, u16), ys: (u16, u16), couleur: Color) -> io::Result<()> {
        let mut w = io::stdout();
        queue!(w, SetBackgroundColor(couleur))?;
        for x in xs.0..xs.1 {
            for y in ys.0..ys.1 {
                queue!(w, cursor::MoveTo(x * 2, y), Print("  ".to_string()),)?;
            }
        }
        queue!(w, SetBackgroundColor(Color::Reset))?;
        Ok(())
    }

    #[allow(unused)]
    pub fn clear(&self) -> io::Result<()> {
        let mut xs = self.co_x;
        let mut ys = self.co_y;
        xs.0 += 1;
        xs.1 -= 1;
        ys.0 += 1;
        ys.1 -= 1;
        self.draw_square(xs, ys, self.couleur_fond)?;
        Ok(())
    }

    #[allow(unused)]
    pub fn draw(&self) -> io::Result<()> {
        self.draw_square(self.co_x, self.co_y, self.couleur_bordure)?;
        self.clear()?;
        Ok(())
    }
}

pub trait ContainTrait {
    fn draw_inside(&self) -> io::Result<()>;
    fn get_container(&self) -> &Container;

    fn init(&self) -> io::Result<()> {
        self.get_container().draw()?;
        self.draw_inside()
    }
    fn update(&self) -> io::Result<()> {
        self.get_container().clear()?;
        self.draw_inside()
    }
    fn test(&self) -> io::Result<()> {
        self.get_container().draw()
    }
}





pub struct DrawSquareAtRawCoord(pub u16, pub u16, pub Color);
impl Command for DrawSquareAtRawCoord {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let x = self.0;
        let y = self.1;
        MoveTo(x, y).write_ansi(f)?;
        DrawSquare(self.2).write_ansi(f)?;
        Ok(())
    }
}

pub struct DrawSquareAt(pub u16, pub u16, pub Color);
impl Command for DrawSquareAt {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        let x = self.0;
        let y = self.1;
        MoveTo(x * 2, y).write_ansi(f)?;
        DrawSquare(self.2).write_ansi(f)?;
        Ok(())
    }
}

pub struct DrawSquare(pub Color);
impl Command for DrawSquare {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        SetBackgroundColor(self.0).write_ansi(f)?;
        Print("  ").write_ansi(f)?;
        ResetColor.write_ansi(f)?;
        Ok(())
    }
}

pub struct CreateTerminal;
impl Command for CreateTerminal {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        //terminal::enable_raw_mode().unwrap();
        EnterAlternateScreen.write_ansi(f)?;
        ResetColor.write_ansi(f)?;
        terminal::Clear(terminal::ClearType::All).write_ansi(f)?;
        Ok(())
    }
}

pub struct CloseTerminal;
impl Command for CloseTerminal {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        ResetColor.write_ansi(f)?;
        terminal::Clear(terminal::ClearType::All).write_ansi(f)?;
        LeaveAlternateScreen.write_ansi(f)?;
        terminal::disable_raw_mode().map_err(|_| std::fmt::Error)?;
        Ok(())
    }
}

pub struct ClearTerminal;
impl Command for ClearTerminal {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        ResetColor.write_ansi(f)?;
        terminal::Clear(terminal::ClearType::All).write_ansi(f)?;
        LeaveAlternateScreen.write_ansi(f)?;

        Ok(())
    }
}

pub struct PrintAt<T: std::fmt::Display>(pub u16, pub u16, pub T);
impl<T: std::fmt::Display> Command for PrintAt<T> {
    fn write_ansi(&self, f: &mut impl std::fmt::Write) -> std::fmt::Result {
        MoveTo(self.0, self.1).write_ansi(f)?;
        Print(&self.2).write_ansi(f)?;
        Ok(())
    }
}