fn main() {
    let x = five();
    let y = {
        let a = 5;
        a
    };
    println!("The value of x: {x}");
    println!("The value of y: {y}")
}

fn five() -> i32 {
    5
}
