fn main() {
    let num1 = 1;
    let num2 = 4;
    println!("sum of num1 and num2 is {}", sum(num1, num2));
    println!("num1 is {num1}, num2 is {num2}"); // value типам похуй, они хранятся не в боксе (куче), а напрямую в стэке

    let str1 = String::from("Hello");
    let str2 = String::from("world");

    print_strings(&str1, &str2);

    // print_strings(str1.clone(), str2.clone());
    // print_strings(str1, str2); <- переменные "исчезают", функция берет владение и после выполнения удаляет все ссылки

    println!("{}, {}!", str1, str2);
}

fn sum(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

fn print_strings(str1: &String, str2: &String) {
    println!("{} {}!", str1, str2);
}

// fn print_strings(str1: String, str2: String) {
//     println!("{} {}!", str1, str2);
// }