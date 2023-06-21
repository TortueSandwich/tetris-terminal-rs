use std::io;

#[allow(unused_imports)]
use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetAttribute, SetBackgroundColor},
    terminal,
    terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

use crate::{param_const::cst, utils::{container::{ContainTrait, Container}, writer::draw_tetromino}};

use super::data::Nexts;
impl ContainTrait for Nexts {
    fn get_container(&self) -> &Container {
        &self.contain
    }
    fn draw_inside(&self) -> io::Result<()> {
        let super_cont = self.get_container();
        for i in 0..cst::NB_PREVIEW as u16 {
            let tetro = &self.nexts[i as usize];
            let c = Container::one_tetro_holder(super_cont.co_x.0+1, super_cont.co_y.0 + i*4);

            draw_tetromino(tetro, &c)?;
        }
        Ok(())
    }
}