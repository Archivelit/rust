// Executable crate

mod garden;

use garden::vegetables::Vegetable;

fn main() { // Should have main function
    let vegie: Vegetable = Vegetable {
        name: String::from("Tomato")
    };
    
    vegie.print_name();
}
