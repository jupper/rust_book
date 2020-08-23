use std::io;

fn main() {
    println!("Please enter the position you want to calculate!");
    let mut position = String::new();
    io::stdin()
        .read_line(&mut position)
        .expect("Failed to read line");

    let position: u32 = position.trim().parse().expect("Please type a number!");

    let mut counter = 3;
    let mut value: u128 = 0;
    let mut value1: u128 = 1;
    let mut value2: u128 = 1;

    while counter <= position {
        value = value1 + value2;
        value1 = value2;
        value2 = value;
        counter = counter + 1;
    }

    println!("The fibonacci value is: {}", value);
}
