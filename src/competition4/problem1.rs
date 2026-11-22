use std::io::{self, Read};

fn main() {
    let mut input = String::from("6
1
2
3
4
10
10000
    ");

    // io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    
    for _ in 0..n {
        let opp_health: usize = iter.next().unwrap().parse().unwrap();
        let charges = 0;
        let number_of_moves = 0;
        let attack_power = 1;

        while opp_health > 0 {
            if opp_health == 1 {
                number_of_moves = 1;
                break;
            }


            if attack_power >= opp_health {
                break;
            }   

            charges += 1;
            attack_power += charges + 1;
            number_of_moves += 1;

        }
        
        

        println!("{}", number_of_moves);

    }

}