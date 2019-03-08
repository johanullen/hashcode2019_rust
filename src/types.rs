#[macro_use]
use matrix::prelude::Packed;
use std::cmp::min;
use std::collections::HashSet;
use std::rc::Rc;
extern crate matrix;

pub type Tags = HashSet<String>;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Pic {
    H { idx: (usize,), tags: Rc<Tags> },
    V { idx: (usize,), tags: Rc<Tags> },
    VV { idx: (usize, usize), tags: Rc<Tags> },
}
pub type Pics = Vec<Rc<Pic>>;

impl Pic {
    pub fn tags(&self) -> &Rc<Tags> {
        match self {
            Pic::H { idx, tags } => &tags,
            Pic::V { idx, tags } => &tags,
            Pic::VV { idx, tags } => &tags,
        }
    }

    pub fn score_with(&self, other: &Rc<Pic>) -> usize {
        let a = self.tags();
        let b = other.tags();
        let a_not_b = a.difference(&b).count();
        let a_and_b = a.intersection(&b).count();
        let b_not_a = b.difference(&a).count();
        min(a_not_b, min(a_and_b, b_not_a))
    }

    pub fn combine_with(&self, other: &Rc<Pic>) -> Rc<Pic> {
        match (self, &**other) {
            (Pic::V { idx: si, tags: st }, Pic::V { idx: oi, tags: ot }) => {
                let idx = (si.0, oi.0);
                let union: Tags = st.union(&ot).map(|x| x.clone()).collect();
                let tags = Rc::new(union);
                let pic = Pic::VV { idx, tags };
                Rc::new(pic)
            }
            (_, _) => panic!("not V"),
        }
    }

    pub fn all_scores(&self, pics: &Pics) -> Vec<usize> {
        let mut scores = Vec::with_capacity(pics.len());
        for pic in pics {
            let score = self.score_with(&pic);
            scores.push(score)
        }
        scores
    }
}

pub trait Score {
    fn score(&self) -> usize;
    fn score_matrix(&self) -> Packed;
}

impl Score for Pics {
    fn score(&self) -> usize {
        let pairs = self.windows(2);
        let mut sum: usize = 0;
        for pair in pairs {
            sum += pair[0].score_with(&pair[1]);
        }
        sum
    }
    fn scores_matrix(&self) -> Packed {
        let len = self.len();
        Packed::zero((len, len))
    }
}
