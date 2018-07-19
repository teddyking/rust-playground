extern crate firstfactorial;

use std::env;

fn main() {
    let num = env::args()
        .nth(1)
        .expect("must provide num")
        .parse()
        .expect("error parsing input number");

    println!("{}", firstfactorial::factorial(num));
}
