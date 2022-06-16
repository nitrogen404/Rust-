fn main() {
    let result = weird_loop();
    // println!("{}", result);
    classic_whileloop();
    for_loop();
}

fn weird_loop() {
    let mut counter = 0;
    loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
}

fn classic_whileloop() {
    let mut number = 3;
    while number != 0 {
        number -= 1;
    }
    println!("Rotate")
}

fn for_loop() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("value {}", element);
    }

    for i in 1..a.len() {
        println!("Value {}", a[i]);
    }
}


