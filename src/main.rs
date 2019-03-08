// #![allow(dead_code)]
#![allow(unused_variables)]
mod types;
mod io;
use io::read;


fn main() {
    let pics = read("data/a_example.txt");
    for pic in &pics {
        println!("{:?}", pic);
    }

    println!("{:?}", pics[1].combine_with(&pics[2]));
    println!("{:?}", pics[1].combine_with(&pics[2]).score_with(&pics[3]));
    println!("{:?}", pics[3].score_with(&pics[0]));
}
