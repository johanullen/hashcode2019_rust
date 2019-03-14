use std::cmp::min;
use std::collections::HashSet;
extern crate ndarray;
use ndarray::{Array1, Array2, s};
use std::thread;

pub type Tags = HashSet<String>;
pub type ScoresMatrix = Array2<u8>;
pub type ScoresArray = Array1<u8>;

#[derive(Debug, Clone)]
pub struct Pic {
    pub tags: Tags,
    pub id: PicType,
}

#[derive(Debug, Clone, Copy)]
pub enum PicType {
    H { idx: (usize,) },
    V { idx: (usize,) },
    VV { idx: (usize, usize) },
}
pub type Pics = Vec<Pic>;

unsafe impl Send for PicType {}
unsafe impl Sync for PicType {}
unsafe impl Send for Pic {}
unsafe impl Sync for Pic {}

impl Pic {
    pub fn score_with(&self, other: &Pic) -> u8 {
        let a = &self.tags;
        let b = &other.tags;
        let a_not_b = a.difference(&b).count() as u8;
        let a_and_b = a.intersection(&b).count() as u8;
        let b_not_a = b.difference(&a).count() as u8;
        min(a_not_b, min(a_and_b, b_not_a))
    }

    pub fn combine_with(&self, other: &Pic) -> Pic {
        match (&self.id, &other.id) {
            (PicType::V { idx: si }, PicType::V { idx: oi }) => {
                let idx = (si.0, oi.0);
                let id = PicType::VV { idx };
                let tags: Tags = self.tags.union(&other.tags).map(|x| x.clone()).collect();
                // let tags = Rc::new(union);
                let pic = Pic { id, tags };
                pic
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
        let threads = 6;
        let len = self.len();
        let slice = len / threads + 1;
        let max_slice = min(threads-1, len)+1;
        let mut handles = vec![];
        let mut scores = ScoresMatrix::zeros((len, len));
        for tc in 0..max_slice {
            let start = tc*slice;
            let end = min(len, (tc+1)*slice);
            println!("{:?}-{:?}", start, end);
            let mut scores_slice = scores.slice_mut(s![start..end, ..]);
            let builder = thread::Builder::new().name(format!("slice {:?} to {:?} ", start, end));
            let pics = self.clone();
            let pics_slice = self[start..end].to_vec();
            let handle = unsafe { builder.spawn_unchecked(move || {
                for (idx, pic) in pics_slice.iter().enumerate() {
                // for (idx, pic) in self.iter().enumerate() {
                    let pic_scores = pic.all_scores(&pics);
                    // let idx = tc*slice + idx;
                    for (jdx, &score) in pic_scores.iter().enumerate() {
                        scores_slice[(idx, jdx)] = score;
                    }
                }
            }).unwrap()};
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        scores
    }
}
