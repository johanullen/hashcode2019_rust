use std::fs::File;
use std::io::{BufReader, BufRead};
use crate::types::{Pic, Tags, Pics};

pub fn read(filename: &str) -> Pics{
    let file = File::open(filename).expect(&format!("file {:?} not found", filename));
    let mut reader = BufReader::new(file);
    let mut first_line = String::new();
    reader.read_line(&mut first_line);
    let no_lines = first_line.trim().parse::<usize>().expect(&format!("{:?}", first_line));
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
        let pic = match pic_type {
            "H" => Pic::H{idx, tags},
            "V" => Pic::V{idx, tags},
            x => panic!("{:?}", x),
        };
        pics.push(pic);
    }
    pics
}