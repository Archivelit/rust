fn main() {
    let s = String::from("Hello world!");
    println!("{}", second_word(&s));
}

fn second_word(s: &str) -> &str {
    let str: &str = cut_first_word(s);
    if str.len() == s.len() {
        return &s[..];
    }

    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}

fn cut_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[i+1..];
        }
    }

    &s[..]
}