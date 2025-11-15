// dir/mod.rs is the old way to define module

pub mod vegetables;

use vegetables::Vegetable as Vegie;

pub struct Garden {
    vegetables: Vec<Vegie>
}

impl Garden {
    pub fn print_vegetables(&self) -> () {
        for vegetable in &self.vegetables {
            println!("The {} growing in garden!", vegetable.name);
        }
    } 

    pub fn add_vegetable(&mut self, vegetable: Vegie) -> () {
        self.vegetables.push(vegetable);
    }

    pub fn create_from(vegetables: Vec<Vegie>) -> Garden {
        Garden { vegetables }
    }
}