use crate::{utils::{container::*,  writer::draw_tetromino}};
use super::data::Hold;

impl ContainTrait for Hold {
    fn draw_inside(&self) -> std::io::Result<()> {
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