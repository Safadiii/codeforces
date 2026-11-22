use core::num;
use std::{collections::HashMap, hash::Hash, io::{self, Read}, num::IntErrorKind::PosOverflow, sync::Arc, thread::available_parallelism};

pub fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let len: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();

        let array: Vec<i32> = iter.by_ref().take(len).map(|x| x.parse().unwrap()).collect();

        let mut available_indexes: HashMap<i32, Vec<usize>> = HashMap::new();
        let mut iterations = 0;

        for window in array.windows(k) {
            if iterations == 0 {
                //seed the available indexes first run
                for (pos, number) in window.iter().enumerate() {
                    available_indexes.entry(*number).or_default().push(pos);
                }
            } else {
                //check every available indexes against the current window
                available_indexes.retain(|&key, positions| {
                    positions.retain(|&pos| window[pos] == key);
                    !positions.is_empty()  
                });
            }
            iterations += 1;
        }

        let num: usize = available_indexes.values().into_iter().map(|arr| arr.len()).sum();

        println!("{}", num);
    }
}