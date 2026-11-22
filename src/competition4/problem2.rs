use std::io::{self, Read};

fn main() {
    let mut input = String::from("
    4
1 2 5
-+-+-
1 1 5
-+-+-
0 1 5
-----
0 50 10
++++++++++
    ");

    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..n {
        let mut lvl1 = iter.next().unwrap().parse().unwrap();
        let mut lvl2 = iter.next().unwrap().parse().unwrap();
        let length = iter.next().unwrap().parse().unwrap();
        
        for spell_action in iter.next().unwrap() {
            println!("{}", spell_action);
        }
        

    }
}