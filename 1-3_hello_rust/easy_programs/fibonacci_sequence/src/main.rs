use std::io;

fn main() {
    println!("Input count of cycles to got number from fibonacci sequence");

    let mut cycles_count = String::new();

    io::stdin()
        .read_line(&mut cycles_count)
        .expect("Failed to read line");

    let cycles_count: u32 = cycles_count
        .trim()
        .parse()
        .expect("Count of cycles cannot be negative");

    println!(
        "The {cycles_count}th number in fibonacci sequence is {}",
        fibonacci_number(cycles_count)
    );
}

fn fibonacci_number(cycles: u32) -> u32 {
    let mut current = 1;
    let mut previous = 0;
    for _ in 1..cycles {
        let temp = current;
        current += previous;
        previous = temp;
    }

    return current;
}
