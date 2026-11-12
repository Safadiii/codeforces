fn main() {
    let mut input = String::from("5
5
BRRBR
1
R
4
RBRB
3
BBB
6
RRBBRB
    ");

    io::stdin().read_to_string(&mut input).unwrap();

    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n = iter.next().unwrap().parse().unwrap(); //length of string
        //let r = 0, b = 1

        let overrides = 0;

        let s = iter.next().unwrap();

        for c in s.chars() {
            println!("{}", c);
        }
    }
}