use std::io::{self, Read};
fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();


    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();

        let nums: Vec<i64> = (0..n).map(|_| iter.next().unwrap().parse().unwrap()).collect();

        let mut divisor = 1;

        let mut max_count = 0;


        for &d in nums.iter() {
            let count = nums.iter().filter(|&&x| x % d == 0).count();

            if count > max_count {
                max_count = count;
                divisor = d;
            }
        }

        let removed = nums.len() - nums.iter()
                                    .filter(|&&x| x % divisor == 0)
                                    .count();

        

        println!("{:?}", removed);
    }
}