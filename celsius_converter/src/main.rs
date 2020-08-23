use std::io;

fn main() {
    println!("Please choose what you want to convert: ");
    println!("1. Celsius to Fahrenheit");
    println!("2. Fahrenheit to Celsius");
    let mut selection = String::new();
    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    if selection.trim() == "1" {
        println!("Please enter the temperature in Celsius: ");
        let mut cel = String::new();
        io::stdin()
            .read_line(&mut cel)
            .expect("Failed to read line");
        let cel: f32 = cel.trim().parse().expect("Please type a number!");
        println!("It's {} Fahrenheit", cel_to_fahr(cel));
    } else if selection.trim() == "2" {
        println!("Please enter the temperature in Fahrenheit: ");
        let mut fahr = String::new();
        io::stdin()
            .read_line(&mut fahr)
            .expect("Failed to read line");
        let fahr: f32 = fahr.trim().parse().expect("Please type a number");
        println!("It's {} Celsius", fahr_to_cel(fahr));
    } else {
        println!("Wrong entry! Should be 1 or 2");
    }
}

fn cel_to_fahr(cel: f32) -> f32{
    cel*9.0/5.0 + 32.0
}

fn fahr_to_cel(fahr: f32) -> f32{
    (fahr - 32.0) * 5.0/9.0
}
