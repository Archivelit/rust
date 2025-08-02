#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 { // &self is short for self: &Self
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

impl Rectangle { // We can have more impl blocks
    fn square(size: u32) -> Self { // Associated function
        Self { // In this case Self is allias for Rectangle
            width: size,
            height: size,
        }
    }
}

fn main() {

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 20,
        height: 10,
    };

    let rect3 = Rectangle {
        width: 40,
        height: 70,
    };

    println!("rect1 is {rect1:?}");

    println!(
        "The area of the rectangle 1 is {} square pixels",
        rect1.area()
    );

    if rect1.width() {
        println!("The rectangle 1 has a nonzero width; it is {}", rect1.width);
    }

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect2 hold rect3? {}", Rectangle::can_hold(&rect2, &rect3)); // Alternative of calling methods

    let square = Rectangle::square(20); // Associated function call

    println!("The square height and width is {}, {}", square.height, square.width)
}
