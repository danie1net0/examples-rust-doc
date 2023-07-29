fn main() {
    println!("\n================== String Slices ==================");
    {
        let string = String::from("hello world");

        let slice = &string[0..2];
        println!("string[0..2]: {slice}");

        let slice = &string[..2];
        println!("string[..2]: {slice}");

        let len = string.len();
        let slice = &string[3..len];
        println!("string[3..len]: {slice}");

        let slice = &string[3..];
        println!("string[3..]: {slice}");

        let slice = &string[0..len];
        println!("string[0..len]: {slice}");

        let slice = &string[..];
        println!("string[..]: {slice}");
    }

    println!("\n============ String Literals as Slices ============");
    {
        let string = "Hello, world!";
        let slice = &string[..5];
        println!("string[..5]: {slice}");
    }

    println!("\n=========== String Slices as Parameters ===========");
    {
        let string = String::from("hello world");
        println!("string: {string}");

        let word = first_word(&string[0..6]);
        println!("first_word(&string[0..6]): {word}");

        let word = first_word(&string[..]);
        println!("first_word(&string[..]): {word}");

        let word = first_word(&string);
        println!("first_word(&string): {word}");

        let my_string_literal = "hello world";
        println!("\nmy_string_literal: {my_string_literal}");

        let word = first_word(&my_string_literal[0..6]);
        println!("first_word(&my_string_literal[0..6]): {word}");

        let word = first_word(&my_string_literal[..]);
        println!("first_word(&my_string_literal[..]): {word}");

        let word = first_word(my_string_literal);
        println!("first_word(my_string_literal): {word}");
    }
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}