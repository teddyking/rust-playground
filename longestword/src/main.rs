extern crate longestword;

use std::env;

fn main() {
    let sentence = env::args().nth(1).expect("must provide sentence");
    println!("{}", longestword::longest_word(&sentence));
}
