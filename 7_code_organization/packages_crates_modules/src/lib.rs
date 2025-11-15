// Lib crate
// Only one per project

mod garden;

use garden::Garden;
use garden::vegetables::Vegetable;

// use garden::{Garden, vegetables::Vegetable}; alternative way to import modules. Same as above

pub fn create_garden() -> Garden {
    let mut vegetables: Vec<Vegetable> = Vec::new();
    
    vegetables.push(Vegetable{
        name: String::from("Tomato")
    });
    vegetables.push(Vegetable{
        name: String::from("Asparagus")
    });

    create_garden_from(vegetables)
}

pub fn create_garden_from(vegetables: Vec<Vegetable>) -> Garden {
    garden::Garden::create_from(vegetables)
}