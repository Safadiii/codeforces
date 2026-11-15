use std::{backtrace::BacktraceStatus::Disabled, io::{self, Read}};

fn main() {
    let mut input = String::from("
4
1 1
0
4 4
1001
5 1
11111
7 2
1101001
"
    );


    // io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n_tests: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..n_tests {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let k: usize = iter.next().unwrap().parse().unwrap();

        let initial_string: String = iter.next().unwrap().parse().unwrap();

        let mut diversity = 0;

        if n == 1 {
            if k >= 1 { 
                println!("{}", 2);
            } else {
                println!("{}", 1);
            }
        }

        for (i, char) in initial_string.chars().enumerate() {
            //find diversity for bit 1
            if char == '1' {
                for (next_ch_idx, next_ch) in initial_string.chars().skip(i+1).enumerate() {
                    if next_ch == '1' {
                        let distance = next_ch_idx - i;
                        if distance <= k {
                            let permutations: u64 = (1..=(distance as u64)).product();
                            diversity += permutations;
                        } 
                    }
                }
            }

            if char == '0' {
                for (next_ch_idx, next_ch) in initial_string.chars().skip(i+1).enumerate() {
                    if next_ch == '0' {
                        let distance = next_ch_idx - i;
                        if distance <= k {
                            let permutations: u64 = (1..=(distance as u64)).product();
                            diversity += permutations;
                        }
                    }
                }
            }   
        }

        println!("{}", diversity+1);
    }

}