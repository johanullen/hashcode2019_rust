use std::rc::Rc;
use std::collections::HashSet;
use std::cmp::min;

pub type Tags = HashSet<String>;
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Pic {
    H{idx:(usize,), tags:Rc<Tags>},
    V{idx:(usize,), tags:Rc<Tags>},
    VV{idx:(usize, usize), tags:Rc<Tags>},
}
pub type Pics = Vec<Rc<Pic>>;

impl Pic {
    pub fn tags(&self) -> &Rc<Tags> {
        match self {
            Pic::H{idx, tags} => &tags,
            Pic::V{idx, tags} => &tags,
            Pic::VV{idx, tags} => &tags,
        }
    }

    pub fn score_with(&self, other:&Rc<Pic>) -> usize {
        let a = self.tags();
        let b = other.tags();
        let a_not_b = a.difference(&b).count();
        let a_and_b = a.intersection(&b).count();
        let b_not_a = b.difference(&a).count();
        min(a_not_b, min(a_and_b, b_not_a))
    }

    pub fn combine_with(&self, other:&Rc<Pic>) -> Rc<Pic> {
        match (self, &**other) {
            (Pic::V{idx: si, tags: st}, Pic::V{idx: oi, tags: ot}) => {
                let idx = (si.0, oi.0);
                let union: Tags = st.union(&ot).map(|x| x.clone()).collect();
                let tags = Rc::new(union);
                let pic = Pic::VV{idx, tags};
                Rc::new(pic)
            },
            (_, _) => panic!("not V")
        }
    }
}