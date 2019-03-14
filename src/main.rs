#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(optin_builtin_traits)]
#![feature(thread_spawn_unchecked)]
mod io;
mod types;
mod analyse;
use std::time::Instant;
use io::read;
use types::Score;
use analyse::iterative_greedy;

fn main() {
    let start = Instant::now();
    // let filename = "data/a_example.txt";
    // let filename = "data/b_lovely_landscapes.txt";
    // let filename = "data/c_memorable_moments.txt";
    let filenames = vec![
        "data/a_example.txt",
        // "data/b_lovely_landscapes.txt",
        "data/c_memorable_moments.txt",
    ];
    println!("*******************************************");
    for filename in filenames {
        let ds_timer = Instant::now();
        let interm = Instant::now();
        let pics = read(filename);
        let end = interm.elapsed();
        println!("read {:?}", filename);
        // println!("{:?}", pics);
        println!("\tscore all {:?}", pics.score());
        println!("\ttime: {:.4}s", end.as_secs() as f64 + end.subsec_nanos() as f64 *1e-9);
        println!("\t-----------------------------------");

        let interm = Instant::now();
        let pics = iterative_greedy(&pics);
        let end = interm.elapsed();
        println!("\titerative_greedy");
        // println!("{:?}", pics);
        println!("\tscore all {:?}", pics.score());
        println!("\ttime: {:.4}s", end.as_secs() as f64 + end.subsec_nanos() as f64 *1e-9);
        println!("\t-----------------------------------");

        let interm = Instant::now();
        let pics = iterative_greedy(&pics);
        let end = interm.elapsed();
        println!("\titerative_greedy #2");
        // println!("{:?}", pics);
        println!("\tscore all {:?}", pics.score());
        println!("\ttime: {:.4}s", end.as_secs() as f64 + end.subsec_nanos() as f64 *1e-9);
        println!("\t-----------------------------------");

        let end = ds_timer.elapsed();
        println!("time: {:.4}s", end.as_secs() as f64 + end.subsec_nanos() as f64 *1e-9);
        println!("*******************************************");
    }

    let end = start.elapsed();
    println!("time: {:.4}s", end.as_secs() as f64 + end.subsec_nanos() as f64 *1e-9);
}
