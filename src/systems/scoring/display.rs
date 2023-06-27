use std::io;

use crossterm::{queue, cursor::{MoveTo, MoveToNextLine}, style::Print};

use crate::utils::container::ContainTrait;

use super::data::ScoringSys;

impl ContainTrait for ScoringSys {
    fn get_container(&self) -> &crate::utils::container::Container {
        &self.container
    }

    fn draw_inside(&self) -> std::io::Result<()> {
        queue!(
            io::stdout(),
            MoveTo(self.get_container().co_x.0 + 2, self.get_container().co_y.0 + 1 ),
            Print(format!("score {:?}", self.get_score())), 
            MoveTo(self.get_container().co_x.0 + 2, self.get_container().co_y.0 + 3),
            Print(format!("{:?}", self.current_move )),
            MoveTo(self.get_container().co_x.0 + 2, self.get_container().co_y.0 + 5),
            Print(format!("{}", self.combo )),

        )
    }
}