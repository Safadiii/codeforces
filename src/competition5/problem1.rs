use core::num;
use std::io::{self, Read};

pub fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let number: i64 = iter.next().unwrap().parse().unwrap();

        if number % 2 == 1 {
            println!("Yes");
        } else if number % 4 == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}