use crate::pic_type::{Pic, PicType};
use std::cmp::min;
use std::collections::HashSet;
extern crate ndarray;
use ndarray::{s, Array2};

pub type PicsVec = Vec<Pic>;
pub type PicSet = HashSet<Pic>;

pub struct Pics<Iterator<Item=Pic>>;

pub type ScoresMatrix = Array2<u16>;

pub trait PicsFn<Pics> {
    fn filter(&self, types: Vec<&str>) -> PicsVec;
}

impl PicsFn for PicsVec {
    fn filter(&self, types: Vec<&str>) -> PicsVec {
        self.iter()
            .filter(|x| match x.source {
                PicType::H(_) => types.contains(&"H"),
                PicType::V(_) => types.contains(&"V"),
                PicType::VV(_, _) => types.contains(&"VV"),
            })
            .cloned()
            .collect()
    }
}

pub trait PicsVecFn {
    fn reindex(&mut self);
    fn score(&self) -> u16;
    fn scores_matrix(&self) -> ScoresMatrix;
}

impl PicsVecFn for PicsVec {
    fn reindex(&mut self) {
        for (idx, pic) in self.iter().enumerate() {
            pic.id = idx;
        }
    }

    fn score(&self) -> u16 {
        let pairs = self.windows(2);
        let mut sum: u16 = 0;
        for pair in pairs {
            sum += pair[0].score_with(&pair[1]);
        }
        sum
    }
    fn scores_matrix(&self) -> ScoresMatrix {
        let threads = 6;
        let len = self.len();
        let slice = len / threads + 1;
        let max_slice = min(threads - 1, len) + 1;
        let mut handles = vec![];
        let mut scores = ScoresMatrix::zeros((len, len));
        for tc in 0..max_slice {
            let start = tc * slice;
            let end = min(len, (tc + 1) * slice);
            let mut scores_slice = scores.slice_mut(s![start..end, ..]);
            let builder = thread::Builder::new().name(format!("slice {:?} to {:?} ", start, end));
            let pics = self.clone();
            let pics_slice = self[start..end].to_vec();
            let handle = unsafe {
                builder
                    .spawn_unchecked(move || {
                        for (idx, pic) in pics_slice.iter().enumerate() {
                            let pic_scores = pic.all_scores(&pics);
                            for (jdx, &score) in pic_scores.iter().enumerate() {
                                scores_slice[(idx, jdx)] = score;
                            }
                        }
                    })
                    .unwrap()
            };
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        scores
    }
}
