// Newer way to define module. The path to this module is garden::vegetables 

pub struct Vegetable {
    pub name: String
}

impl Vegetable {
    pub fn print_name(&self) -> () {
        println!("{}", self.name);
    }
}
