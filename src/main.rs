use num_bigint::{BigInt, ToBigInt};
use std::io;
fn main() {
    println!("enter a number between 0 and 185");
    let mut num = String::new();

    io::stdin().read_line(&mut num).expect("failed to read");

    let num: BigInt = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("enter a number");
            return;
        }
    };
    let mut previous_number: BigInt = 1.to_bigint().unwrap();
    let mut current_number: BigInt = 0.to_bigint().unwrap();
    
    let mut  i = 0.to_bigint().unwrap();
    while i <= num {
        println!(" fib({i}) = {current_number}");
        let previous_previous_number = previous_number;
        previous_number = current_number + &previous_previous_number;
        current_number = previous_previous_number;
        i += 1;
    }
}
