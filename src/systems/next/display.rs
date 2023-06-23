use std::{arch::x86_64::_mm_test_all_zeros, io};

#[allow(unused_imports)]
use crossterm::{
    cursor, execute, queue,
    style::{Color, Print, ResetColor, SetAttribute, SetBackgroundColor},
    terminal,
    terminal::{ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    Result,
};

use crate::{
    param_const::cst,
    utils::{
        container::{ContainTrait, Container},
        writer::draw_tetromino,
    },
};

use super::data::Nexts;
impl ContainTrait for Nexts {
    fn get_container(&self) -> &Container {
        &self.contain
    }
    fn draw_inside(&self) -> io::Result<()> {
        let super_cont = self.get_container();
        for i in 0..cst::NB_PREVIEW as u16 {
            let tetro = self.get(i as usize);
            if let Some(t) = tetro {
                let c = Container::one_tetro_holder(super_cont.co_x.0, super_cont.co_y.0 + i * 4);
                draw_tetromino(t, &c)?;
            } else {
                panic!("Could not read {i}e element");
            }
        }
        Ok(())
    }
}
