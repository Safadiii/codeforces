use std::io::{self, Read};

fn main() {


    let mut input = String::from("
    5
6
1 2 2 3 3 3
1
5
3
7 7 7
4
4 1 3 2
11
3 1 4 1 5 9 2 6 5 3 5
");

    // io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n = iter.next().unwrap().parse().unwrap(); //number of books

        let mut books: Vec<i32> = Vec::with_capacity(n);

        for _ in 0..n {
            let book: i32 = iter.next().unwrap().parse().unwrap();
            books.push(book);
        }

        books.sort();

        let mid = n / 2;

        let mut right_side: Vec<i32> = Vec::new();
        let mut left_side: Vec<i32> = Vec::new();


        let mut peak_array: Vec<i32> = Vec::new();


        peak_array.extend(&books[..mid]);
        peak_array.extend(books[mid..].iter().rev());

        for book in &peak_array {
            print!("{} ", book);
        }
        println!();
    }

}   