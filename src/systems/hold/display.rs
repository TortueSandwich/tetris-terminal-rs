use crate::{utils::{container::*,  writer::draw_tetromino}};
use std::io;
use super::data::Hold;
#[allow(unused_imports)]
use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetAttribute, SetBackgroundColor},
    terminal,
    terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};
impl ContainTrait for Hold {
    fn draw_inside(&self) -> io::Result<()> {
        match &self.holded_polyomino {
            None => Ok(()),
            Some(tetro) => {
                draw_tetromino(tetro, self.get_container())?;
                Ok(())
            }
        }
    }
    fn get_container(&self) -> &Container {
        &self.contain
    }
}