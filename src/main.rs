use std::io;

fn fib(n: usize) -> usize {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        fib(n-1) + fib(n-2)
    }
}

fn main() {
    println!("enter a number: ");

    let mut num = String::new();

    io::stdin()
        .read_line(&mut num)
        .expect("failed to read the number");

    let num: usize = match num.trim().parse(){
        Ok(num) => num,
        Err(_) => {
            println!("invalid input, enter a number");
            return;
        }
    };
    
    let result = fib(num);
    println!("fib({}) = {}", num, result);
}
