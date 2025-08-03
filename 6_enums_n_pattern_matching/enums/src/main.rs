#[derive(Debug)]

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
// "Alternative" to Message enum:

/*
struct QuitMessage;
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String);
struct ChangeColorMessage(i32, i32, i32);
*/

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn route(ip: IpAddr) {
    println!("Ip address is {ip:?}")
}

fn main() {
    let four = IpAddr::V4(127, 0, 0, 1);
    let six = IpAddr::V6(String::from("::1"));

    route(four);
    route(six);

    let msg = Message::Quit;
    let msgWrite = Message::Write(String::from("Hello"));

    // both are valid
    msg.call();
    msgWrite.call();


}
