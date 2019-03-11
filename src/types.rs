use std::cmp::min;
use std::collections::HashSet;
use std::rc::Rc;
extern crate ndarray;
// use ndarray::prelude::*;
use ndarray::{Array1, Array2};

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

    pub fn all_scores(&self, pics: &Pics) -> Array1<usize> {
        // let mut scores = Vec::with_capacity(pics.len());
        let mut scores = Array1::<usize>::zeros(pics.len());
        for (idx, pic) in pics.enumerate() {
            let score = self.score_with(&pic);
            scores[idx]
            // scores.push(score)
        }
        scores
    }

    fn idx(self) -> usize {
        match self {
            Pic::H { idx, tags } => idx.0,
            Pic::V { idx, tags } => idx.0,
            Pic::VV { idx, tags } => idx.0,
        }
    }
}

pub trait Score {
    fn score(&self) -> usize;
    fn scores_matrix(&self) -> Array2<usize>;
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
    fn scores_matrix(&self) -> Array2<usize> {
        let len = self.len();
        let mut scores = Array2::<usize>::zeros((len, len));
        let pics = self.clone();
        for (idx, row) in scores.genrows_mut().enumerate() {
            let pic_scores = self[idx].all_scores(&pics);
            row::from_vec(pic_scores);
        }
        scores
    }
}
