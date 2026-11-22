use core::num;
use std::{collections::HashMap, hash::Hash, io::{self, Read}, num::IntErrorKind::PosOverflow, sync::Arc, thread::available_parallelism};

pub fn main() {
    let mut input = String::new();

    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();

        let a: Vec<i32> = iter.by_ref().take(n).map(|x| x.parse().unwrap()).collect();

        let mut ans = 0;
        let mut len: usize = 1;

        for i in 1..n {
            if a[i] == a[i-1] {
                len += 1;
            }
            else {
                ans += len.saturating_sub(n - k);
                len = 1;
            }
        }

        ans += len.saturating_sub(n - k);

        println!("{}", ans);


    }
}