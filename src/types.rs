use std::cmp::min;
use std::collections::HashSet;
use std::rc::Rc;
extern crate ndarray;
use ndarray::{Array1, Array2};

pub type Tags = HashSet<String>;
pub type ScoresMatrix = Array2<u8>;
pub type ScoresArray = Array1<u8>;

#[derive(Debug, Clone)]
pub struct Pic {
    pub tags: Rc<Tags>,
    pub id: PicType,
}

#[derive(Debug, Clone)]
pub enum PicType {
    H { idx: (usize,) },
    V { idx: (usize,) },
    VV { idx: (usize, usize) },
}
pub type Pics = Vec<Rc<Pic>>;

impl Pic {
    // pub fn tags(&self) -> &Rc<Tags> {
    //     match self {
    //         Pic::H { idx, tags } => &tags,
    //         Pic::V { idx, tags } => &tags,
    //         Pic::VV { idx, tags } => &tags,
    //     }
    // }

    pub fn score_with(&self, other: &Rc<Pic>) -> u8 {
        let a = &self.tags;
        let b = &other.tags;
        let a_not_b = a.difference(&b).count() as u8;
        let a_and_b = a.intersection(&b).count() as u8;
        let b_not_a = b.difference(&a).count() as u8;
        min(a_not_b, min(a_and_b, b_not_a))
    }

    pub fn combine_with(&self, other: &Rc<Pic>) -> Rc<Pic> {
        match (&self.id, &other.id) {
            (PicType::V { idx: si }, PicType::V { idx: oi }) => {
                let idx = (si.0, oi.0);
                let id = PicType::VV { idx };
                let union: Tags = self.tags.union(&other.tags).map(|x| x.clone()).collect();
                let tags = Rc::new(union);
                let pic = Pic { id, tags };
                Rc::new(pic)
            }
            (_, _) => panic!("not V"),
        }
    }

    pub fn all_scores(&self, pics: &Pics) -> ScoresArray {
        let mut scores = ScoresArray::zeros(pics.len());
        for (idx, pic) in pics.iter().enumerate() {
            let score = self.score_with(&pic);
            scores[idx] = score;
        }
        scores
    }

    fn idx(self) -> usize {
        match self.id {
            PicType::H { idx } => idx.0,
            PicType::V { idx } => idx.0,
            PicType::VV { idx } => idx.0,
        }
    }
}

pub trait Score {
    fn score(&self) -> u8;
    fn scores_matrix(&self) -> ScoresMatrix;
}

impl Score for Pics {
    fn score(&self) -> u8 {
        let pairs = self.windows(2);
        let mut sum: u8 = 0;
        for pair in pairs {
            sum += pair[0].score_with(&pair[1]);
        }
        sum
    }
    fn scores_matrix(&self) -> ScoresMatrix {
        let len = self.len();
        let mut scores = ScoresMatrix::zeros((len, len));
        let pics = self.clone();
        for (idx, pic) in self.iter().enumerate() {
            let pic_scores = pic.all_scores(&pics);
            for (jdx, &score) in pic_scores.iter().enumerate() {
                scores[(idx, jdx)] = score;
            }
        }
        scores
    }
}
