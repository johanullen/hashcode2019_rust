mod types;
mod io;
use io::read;

fn main() {
    let pics = read("data/a_example.txt");
    for pic in pics {
        println!("{:?}", pic);
    }
}
