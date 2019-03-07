use std::collections::HashSet;
use std::ops::Add;
// use std::clone::Clone;

pub type Tags = HashSet<String>;
#[derive(Debug, Clone)]
pub enum Pic {
    H{idx:(usize,), tags:Tags},
    V{idx:(usize,), tags:Tags},
    VV{idx:(usize, usize), tags:Tags},
}
pub type Pair = (Pic, Pic);
pub type Pics = Vec<Pic>;
pub enum Slides {
    ONE(Pic),
    MANY(Pics),
}


impl Add for Slides {
    type Output = Pics;
    fn add (self,  rhs:Slides) -> Pics {
        match (self, rhs) {
            (Slides::ONE(a), Slides::ONE(b)) => vec![a, b],
            (Slides::ONE(a), Slides::MANY(b)) => {
                let mut a = vec![a];
                a.append(&mut b.to_vec());
                a
            },
            (Slides::MANY(a), Slides::ONE(b)) => {
                let mut a = a.to_vec();
                a.push(b);
                a
            }
            (Slides::MANY(a), Slides::MANY(b)) => {
                let mut a = a.to_vec();
                a.append(&mut b.to_vec());
                a
            }

        }
    }
}
