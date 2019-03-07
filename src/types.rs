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
