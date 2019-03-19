use crate::pics_type::{PicSet, Pics, PicsFn};
use carte::pic_tyep::{Pic, PicType};

pub fn iter_greedy(pics: &Pics) -> Pics {
    let mut pic = pics[0].clone();
    let mut verts = PicSet::from_iter(pics.filter(vec!["V"]));
    let mut pics_set = PicSet::from_iter(pics.clone());
    let mut new_pics = Pics::with_capacity(pics.len());

    while !pics_set.is_empty() {
        new_pics.push(pic.clone());
        pics_set.remove(&pic);
        if pics.is_empty() {
            break;
        }
        pic = match pic.source {
            PicType::V(_) => {
                verts.remove(&pic);
                if verts.is_empty() {
                    new_pics.pop();
                }
                let other = pic.min_intersection(&pics_set);
                verts.remove(&other);
                pics_set.remove(&other);
                new_pics.pop();
                Pic {
                    id: 0,
                    tags: pic.union_with(&other),
                    source: PicType::VV(pic.id, other.id),
                }
            }
            PicType::H(_) => pic.best_match(&pics_set),
            PicType::VV(_, _) => pic.best_match(&pics_set),
        }
    }
    new_pics
}
