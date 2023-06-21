use std::io;

use crate::{utils::{container::{ContainTrait, Container}, writer::queue_draw}, param_const::{cst, param}};

use super::data::Playground;

impl ContainTrait for Playground {
    fn get_container(&self) -> &Container {
        &self.contain
    }
    fn draw_inside(&self) -> io::Result<()> {
        let g = &self.grid; // (20, 10) 'fin plutot (0-19,0-20)
        for y in 0..cst::NB_LIGNE_GRILLE {
            for x in 0..cst::NB_COLONNE_GRILLE {
                let cx = x + self.contain.co_x.0 + 1;
                let cy = y + self.contain.co_y.0 + 1;
                let case_color = g.get_couleur_case((y, x));
                queue_draw(cx, cy, case_color)?;
            }
        }
          

        let i = self.get_distance_preview_polyomino();

        for t in self.current_polyomino.to_coord() {
            let x = t.1 as u16 + 1 + self.contain.co_x.0 ;
            let y = t.0 as u16 + 1 + self.contain.co_y.0;
            queue_draw(x,y + i,param::COULEUR_PREVIEW)?;
            queue_draw(x       ,y,self.current_polyomino.code_ansi())?;
        }
        Ok(())
    }
}