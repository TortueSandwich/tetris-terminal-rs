use crate::{
    erreur::EmptyVecAcces,
    param_const::{cst, param},
    systems::mechanics::randomness::PolyominoBag,
    utils::{container::*, tetromino::Tetromino},
};
use std::collections::VecDeque;

pub struct Nexts {
    nexts: Box<VecDeque<Tetromino>>,
    pub(crate) contain: Container,
}
impl Nexts {
    const MIN_CAPCITY: usize = 7 * cst::NB_BAG;
    const MAX_CAPACITY: usize = 7 * (cst::NB_BAG + 1);
    const NUM_TO_PREVIEW: usize = cst::NB_PREVIEW;
}

impl Nexts {
    fn add_next(&mut self, tetromino: Tetromino) {
        self.nexts.push_back(tetromino);
    }

    pub fn take_next(&mut self) -> Result<Tetromino, EmptyVecAcces> {
        if self.is_empty() {
            return Err(EmptyVecAcces);
        }

        if let Some(t) = self.nexts.pop_front() {
            let res = t.to_owned();
            self.fill_in();
            return Ok(res);
        } else {
            return Err(EmptyVecAcces);
        }
    }
    fn get_next(&self) -> Option<&Tetromino> {
        self.nexts.front()
    }

    pub fn get(&self, i: usize) -> Option<&Tetromino> {
        self.nexts.get(i)
    }
}

impl Nexts {
    pub fn new() -> Self {
        let mut res = Nexts {
            nexts: Box::new(VecDeque::with_capacity(Self::MAX_CAPACITY)),
            contain: param::CONTAINER_NEXT,
        };
        res.fill_in();
        res
    }

    fn add_bag(&mut self, pb: PolyominoBag) {
        self.nexts.extend(pb.0.iter().cloned());
    }

    fn is_empty(&self) -> bool {
        self.nexts.is_empty()
    }

    fn count_full_bag(&self) -> usize {
        self.nexts.len() / 7
    }

    fn need_to_fill(&self) -> bool {
        self.count_full_bag() < Self::MIN_CAPCITY
    }

    fn fill_in(&mut self) {
        while self.need_to_fill() {
            self.add_bag(PolyominoBag::new())
        }
    }
}
