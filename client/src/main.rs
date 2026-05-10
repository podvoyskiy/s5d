use std::env::args;

use s5::utils;

fn main() {
    let test = utils::collect_args(args());
    println!("{:?}", test);
}