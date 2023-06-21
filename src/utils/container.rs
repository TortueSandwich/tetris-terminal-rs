use std::io;
use crossterm::{
    cursor, queue,
    style::{Color, Print, SetBackgroundColor},
};

use crate::param_const::param;

#[allow(unused)]
#[derive(Debug)]
pub struct Container {
    pub co_x :(u16, u16),
    pub co_y :(u16, u16),
    couleur_bordure :Color,
    pub couleur_fond :Color,
}

impl Container {
    #[allow(unused)]
    pub const fn new(x:(u16, u16), y:(u16, u16), couleur_bordure: Color) -> Container {
        Container { co_x: x, co_y: y, couleur_bordure, couleur_fond: param::COULEUR_ }
    }
    #[allow(unused)]
    pub const fn default(x:(u16, u16), y:(u16, u16)) -> Container {
        Container { co_x: x, co_y: y, couleur_bordure: param::COULEUR_BORD, couleur_fond: param::COULEUR_ }
    }

    pub const fn one_tetro_holder(x :u16,y :u16) -> Container {
        Container::default((x, x+6), (y, y+6))
    }
}


impl Container {
    #[allow(unused)]
    fn draw_square(&self, xs:(u16, u16), ys:(u16, u16), couleur :Color) -> io::Result<()> {
        let mut w = io::stdout();
        queue!(w, SetBackgroundColor(couleur))?;
        for x in xs.0..xs.1 {
            for y in ys.0..ys.1 {
                queue!(
                    w,
                    cursor::MoveTo(x*2, y),
                    Print("  ".to_string()),
                )?;
            }
        }
        queue!(w, SetBackgroundColor(Color::Reset))?;
        Ok(())
    }

    #[allow(unused)]
    pub fn clear(&self) -> io::Result<()> {
        let mut xs = self.co_x; let mut ys = self.co_y;
        xs.0 += 1; xs.1 -= 1;
        ys.0 += 1; ys.1 -= 1;
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

    fn init(&self)-> io::Result<()> {
        self.get_container().draw()?;
        self.draw_inside()
    }
    fn update(&self)-> io::Result<()> {
        self.get_container().clear()?;
        self.draw_inside()
    }
    fn test(&self)-> io::Result<()> {
        self.get_container().draw()
    }
}