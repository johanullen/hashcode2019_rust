use crate::types::{Pic, PicType, Pics, Tags};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

pub fn read(filename: &str) -> Pics {
    let file = File::open(filename).expect(&format!("file {:?} not found", filename));
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    let len = reader.read_line(&mut first_line);
    let no_lines = first_line
        .trim()
        .parse::<usize>()
        .expect(&format!("{:?}", first_line));
    let mut pics = Pics::with_capacity(no_lines);
    for (ix, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let idx = (ix,);
        let mut part_iter = line.split(" ");
        let pic_type = part_iter.next().unwrap();
        let no_tags = part_iter.next().unwrap();
        let no_tags = no_tags.parse::<usize>().expect(&format!("{:?}", no_tags));
        let mut tags = Tags::with_capacity(no_tags);
        for tag in part_iter {
            tags.insert(tag.to_string());
        }
        let tags = Rc::new(tags);
        let id = match pic_type {
            "H" => PicType::H { idx },
            "V" => PicType::V { idx },
            x => panic!("{:?}", x),
        };
        let pic = Pic { tags, id };
        pics.push(pic);
    }
    pics
}
