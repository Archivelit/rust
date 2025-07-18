#![allow(unused)]

struct Color(i32, i32, i32); // tuple struct

struct AlwaysEqual; // unit-like struct

struct Point { x: i32, y: i32 }

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("John"),
        email: String::from("email@example.com"),
        sign_in_count: 10,
    };

    let user2 = build_user(String::from("Bob"), String::from("secondemail@example.com"));

    let user3 = User{
        email: String::from("thirdemail@example.com"),
        ..user2 // same values as user2, but different email
    };
    
    println!("{}, {1}", user1.username, user3.username);

    let black = Color(0, 0, 0);
    println!("{}", black.0);

    let mut p = Point{ x: 0, y: 0 };
    
    let x = &mut p.x;

    *x += 1;
    
    println!("{}, {}", p.x, p.y);

    let subject = AlwaysEqual;
}

fn build_user(username: String, email: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}