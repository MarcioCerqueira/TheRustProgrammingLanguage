use std::io;

fn fibonacci(index: u32) -> u32 {
    if index <= 1 {
        index
    } else {
        fibonacci(index - 1) + fibonacci(index - 2)
    }
}

fn main() {
    println!("Enter a value 'n' to discover the nth Fibonacci number!");

    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Please type a positive number!");

    let result = fibonacci(n - 1);
    println!("The Fibonacci number is: {result}");
}
