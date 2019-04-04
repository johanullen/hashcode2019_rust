use crate::pics_type::Pics;
use std::cmp::min;
use std::collections::HashSet;
extern crate ndarray;
use ndarray::Array1;
use std::hash::{Hash, Hasher};

pub type Tags = HashSet<String>;
pub type ScoresArray = Array1<u16>;

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
    pub fn score_with(&self, other: &Pic) -> u16 {
        let a = &self.tags;
        let b = &other.tags;
        let a_not_b = a.difference(&b).count() as u16;
        let a_and_b = a.intersection(&b).count() as u16;
        let b_not_a = b.difference(&a).count() as u16;
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
        // let mut scores = ScoresArray::zeros(pics.len());
        let mut scores = ScoresArray::zeros(100);
        for (idx, pic) in pics.into_iter().enumerate() {
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

    pub fn min_intersection(&self, pics: Pics) -> Pic {
        let mut min_pic = self.clone();
        let mut min = 9999;
        for pic in pics {
            let current = self.intersect_with(&pic).len();
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

    pub fn best_match(self, pics: &Pics) -> Pic {
        let mut best_score = 0;
        let mut best_other = self.clone();
        for other in pics {
            let other_score = self.score_with(&other);
            if best_other == self || other_score > best_score {
                best_score = other_score;
                best_other = other.clone();
            }
        }
        best_other
    }
}
