use std::io::{self, Read};

fn main() {
    let mut input = String::from("6
1 2 3
2 2 2
3 4 5
1 1 4
8 9 10
3 5 7
");

    io::stdin::read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let npiles: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..npiles {
        // We have 3 numbers representing each pile
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();
        let c: i64 = iter.next().unwrap().parse().unwrap();

        if (a == b && b == c) {
            println!("NO");
            continue;
        } 
        
        if (abs(a - b) > 2 || abs(c -b) > 2 || abs(c - a) > 2) {
            println!("NO");
            continue;
        }
        else {
            println!("YES");
        }
    }
}