use std::cmp::min;
use std::collections::HashSet;
extern crate ndarray;
use ndarray::{s, Array1, Array2};
use std::hash::{Hash, Hasher};
use std::thread;

pub type Tags = HashSet<String>;
pub type ScoresMatrix = Array2<u8>;
pub type ScoresArray = Array1<u8>;

#[derive(Debug, Clone)]
pub struct Pic {
    pub tags: Tags,
    pub id: usize,
    pub source: PicType,
}

#[derive(Debug, Clone, Copy)]
pub enum PicType {
    H(usize),
    V(usize),
    VV(usize, usize),
}
pub type Pics = Vec<Pic>;

unsafe impl Send for PicType {}
unsafe impl Sync for PicType {}
unsafe impl Send for Pic {}
unsafe impl Sync for Pic {}

impl Eq for Pic {}
impl PartialEq for Pic {
    fn eq(&self, other: &Pic) -> bool {
        match (self.source, other.source) {
            (PicType::H(sid), PicType::H(oid)) => sid == oid,
            (PicType::V(sid), PicType::V(oid)) => sid == oid,
            (PicType::VV(sid1, sid2), PicType::VV(oid1, oid2)) => sid1 == oid1 && sid2 == oid2,
            (_, _) => false,
        }
    }
}
impl Hash for Pic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.source {
            PicType::H(id) => id.hash(state),
            PicType::V(id) => id.hash(state),
            PicType::VV(id1, id2) => {
                id1.hash(state);
                id2.hash(state)
            }
        }
    }
}

impl Pic {
    pub fn score_with(&self, other: &Pic) -> u8 {
        let a = &self.tags;
        let b = &other.tags;
        let a_not_b = a.difference(&b).count() as u8;
        let a_and_b = a.intersection(&b).count() as u8;
        let b_not_a = b.difference(&a).count() as u8;
        min(a_not_b, min(a_and_b, b_not_a))
    }

    pub fn combine_with(&self, other: &Pic, new_id: usize) -> Pic {
        match (&self.source, &other.source) {
            (PicType::V(_), PicType::V(_)) => {
                let source = PicType::VV(self.id, self.id);
                let tags: Tags = self.tags.union(&other.tags).map(|x| x.clone()).collect();
                let pic = Pic {
                    id: new_id,
                    tags,
                    source,
                };
                pic
            }
            (a, b) => panic!(format!(
                "only `PicType::V` Pics can be combined, not `{:?}` and `{:?}`",
                a, b
            )),
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

    pub fn intersect_with(&self, other: &Pic) -> Tags {
        let mut new_tags = Tags::new();
        for tag in self.tags.intersection(&other.tags) {
            new_tags.insert(tag.clone());
        }
        new_tags
    }

    pub fn min_intersection(&self, pics: &Pics) -> Pic {
        let mut min_pic = pics[0].clone();
        let mut min = self.intersect_with(&min_pic).len();
        for pic in pics {
            let current = self.intersect_with(pic).len();
            if current < min {
                min = current;
                min_pic = pic.clone();
            }
        }
        min_pic
    }

    pub fn union_with(&self, other: &Pic) -> Tags {
        let mut new_tags = Tags::new();
        for tag in self.tags.union(&other.tags) {
            new_tags.insert(tag.clone());
        }
        new_tags
    }

    pub fn source(&self) -> usize {
        match self.source {
            PicType::H(_) => panic!("use Pic.source() only for merging PicType::V, not PicType::H"),
            PicType::V(id) => id,
            PicType::VV(_, _) => {
                panic!("use Pic.source() only for merging PicType::V, not PicType::VV")
            }
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

pub trait PicsFn {
    fn reindex(&mut self);
    fn filter(&self, types: Vec<&str>) -> Pics;
}

impl PicsFn for Pics {
    fn reindex(&mut self) {
        for (idx, pic) in self.iter_mut().enumerate() {
            pic.id = idx;
        }
    }

    fn filter(&self, types: Vec<&str>) -> Pics {
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
