use std::io;

fn main() {
    let mut celsius = String::new();

    println!("Input temperature in celsius");

    io::stdin()
        .read_line(&mut celsius)
        .expect("Failed to read line");

    let celsius: i32 = celsius.trim().parse().expect("Please type a number!");

    let fahrenheit: i32 = (celsius * 9 / 5) + 32;

    println!("Temperature in celsius is {celsius}");
    println!("Temperature in fahrenheit is {fahrenheit}");
}
