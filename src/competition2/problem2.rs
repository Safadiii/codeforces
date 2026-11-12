use std::io::{self, Read};
 
fn main() {
    let mut input = String::new();
 
    io::stdin().read_to_string(&mut input).unwrap();
 
    let mut iter = input.split_whitespace();
 
    let t: usize = iter.next().unwrap().parse().unwrap();
 
    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap(); //length of string
        //let r = true, b = false
        let mut overrides: i32 = 0;
        let mut red: bool = true;
        let s = iter.next().unwrap();
 
        for c in s.chars() {
            if c == 'B' && red {
                overrides += 1;
                continue;
            } else if c == 'R' && !red {
                overrides += 1;
                continue;
            }
            red = !red;
        }
        println!("{}", overrides);
    }
}