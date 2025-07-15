const DAYS: [&str; 12] = [
    "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth",
    "eleventh", "twelfth",
];

const GIFTS: [&str; 11] = [
    "Two turtle doves and",
    "Three french hens",
    "Four calling birds",
    "Five golden rings",
    "Six geese a-laying",
    "Seven swans a-swimming",
    "Eight maids a-milking",
    "Nine ladies dancing",
    "Ten lords a-leaping",
    "Eleven pipers piping",
    "Twelve drummers drumming",
];

fn main() {
    for i in 0..12 {
        print_day(i);
        println!();
    }
}

fn print_day(day: usize) {
    println!(
        "On the {} day of Christmas, my true love sent to me",
        DAYS[day]
    );
    for gift in (0..day).rev() {
        println!("{}", GIFTS[gift as usize]);
    }
    println!(
        "{}",
        if day == 0 {
            "A partridge in a pear tree"
        } else {
            "And a partridge in a pear tree"
        }
    );
}
