#![allow(dead_code)]
#![allow(unused_variables)]
#![feature(optin_builtin_traits)]
#![feature(thread_spawn_unchecked)]
mod greedy2;
mod io;
mod types;
use crate::types::Pics;
use greedy2::iterative_greedy;
use io::read;
use std::time::Instant;
use types::Score;

fn run_alg(alg: &Fn(&Pics) -> Pics, pics: &Pics, name: &str, print_pics: bool) -> Pics {
    let interm = Instant::now();
    let pics = alg(pics);
    let end = interm.elapsed();
    println!("\t{:?}", name);
    if print_pics {
        println!("{:?}", pics);
    }
    println!("\tscore all {:?}", pics.score());
    println!(
        "\ttime: {:.4}s",
        end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9
    );
    println!("\t-----------------------------------");
    pics
}

fn run_read(filename: &str, print_pics: bool) -> Pics {
    let start = Instant::now();
    let pics = read(filename);
    let end = start.elapsed();
    println!("read {:?}", filename);
    if print_pics {
        println!("{:?}", pics);
    }
    println!("\tscore all {:?}", pics.score());
    println!(
        "\ttime: {:.4}s",
        end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9
    );
    println!("\t-----------------------------------");
    pics
}

fn main() {
    let print_pics = false;
    let start = Instant::now();
    // let filename = "data/a_example.txt";
    // let filename = "data/b_lovely_landscapes.txt";
    // let filename = "data/c_memorable_moments.txt";
    let filenames = vec![
        "data/a_example.txt",
        // "data/b_lovely_landscapes.txt",
        // "data/c_memorable_moments.txt",
    ];
    println!("*******************************************");
    for filename in filenames {
        let interm = Instant::now();

        let pics = run_read(filename, print_pics);
        let pics = run_alg(&iterative_greedy, &pics, "iterative_greedy", print_pics);
        let pics = run_alg(&iterative_greedy, &pics, "iterative_greedy 2", print_pics);

        let end = interm.elapsed();
        println!(
            "time: {:.4}s",
            end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9
        );
        println!("*******************************************");
    }

    let end = start.elapsed();
    println!(
        "time: {:.4}s",
        end.as_secs() as f64 + end.subsec_nanos() as f64 * 1e-9
    );
}
