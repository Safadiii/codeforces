use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();


    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let l: i64 = iter.next().unwrap().parse().unwrap();
        let r: i64 = iter.next().unwrap().parse().unwrap();

        let mut ans: i64 = 0;

        if l > 0 || r < 0 {
            if l == 0 || r == 0 {
                ans = r - l - 1;
            } else {
                ans = r - l;
            }
        } else {
            let sum: i64 = (l..=r).map(|x| x).sum();
            if sum == 0 || sum == r || sum == l {
                ans = 0;
            } else {
                if l.abs() > r.abs() {
                    ans = l.abs() - r.abs() -1;
                } else if r.abs() > l.abs() {
                    ans = r.abs() - l.abs() -1;
                }
            }
        }

        println!("{}", ans);
    }
}