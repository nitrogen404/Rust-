use std::io;

fn main() {
    println!("Enter two numbers x and y");
    let mut x = String::new();
    let mut y = String::new();
    io::stdin().read_line(&mut x).expect("invalid input");
    io::stdin().read_line(&mut y).expect("invalid input");
    let x:u32 = x.trim().parse().expect("expected an integer");
    let y:u32 = y.trim().parse().expect("expected an integer");
    let result = add_twonumbers(x, y);
    println!("result is {}", &result);
}

fn add_twonumbers(x:u32, y:u32) -> u32 {
    x + y
}

