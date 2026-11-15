use std::io::{self, Read};

fn main() {
    let mut input = String::new();



    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..n {
        // We need to construct permutation of n numbers such that each is a composite
        // Already composite numbers should be 
        let x: usize = iter.next().unwrap().parse().unwrap();

        if x <= 3 {
            println!("{}", -1);
            continue;
        }
        let mut final_array: Vec<usize> = Vec::with_capacity(x);

        final_array.push(4); //first number not prime is 4 so place it here
        final_array.push(2);
        final_array.push(3);
        final_array.push(1);

        for num in 5..x+1 {
            final_array.push(num);
        }

        let output = final_array.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" ");

        println!("{}", output);

    }

}