fn main() {


    let mut input = String::from("5
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

    // io::stdin().read_to_string().unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n = iter.next().unwrap().parse().unwrap(); //number of books
        let mut k = 0;

        if n <= 1 {
            k = n;
        }

        let books = iter.split_whitespace().collect();

        println!("{:?}", books);


    }

}   