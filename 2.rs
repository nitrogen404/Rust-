// TODO Read two numbers from Command Line Arguments and print the sum of the numbers

use std::io;

fn main() {
    let mut a = String::new();
    println!("enter first number");
    io::Stdin().read_line(&mut a).expect("invalid input");
    let a: i32 = a.trim().parse().expect("invalid input");

    let mut b = String::new();
    println!("Enter second number");
    io::Stdin().read_line(&mut b).expect("invalid input");
    let b: i32 = b.trim().parse().expect("invalid input");

    println!("Sum of two numbers is {} ", a + b);
}



