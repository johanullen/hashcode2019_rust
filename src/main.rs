#![allow(dead_code)]
#![allow(unused_variables)]
mod io;
mod types;
use io::read;
use types::Score;

fn main() {
    let pics = read("data/a_example.txt");
    // let pics = read("data/b_lovely_landscapes.txt");
    // let pics = read("data/c_memorable_moments.txt");
    for pic in &pics {
        println!("{:?}", pic);
    }

    println!("combine 1&2 {:?}", pics[1].combine_with(&pics[2]));
    println!(
        "score 1&2 with 3: {:?}",
        pics[1].combine_with(&pics[2]).score_with(&pics[3])
    );
    println!("score 3 with 0{:?}", pics[3].score_with(&pics[0]));
    println!("score all {:?}", pics.score());
    println!("score for 0 {:?}", pics[0].all_scores(&pics));
    let scores_matrix = pics.scores_matrix();
    println!("{:?}", scores_matrix);
    println!("{:?}", scores_matrix[(0, 0)]);
}
