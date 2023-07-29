fn main() {
    println!("\n================== References and Borrowing ==================");
    {
        let string: String = String::from("hello");

        let length: usize = calculate_length(&string);

        println!("The length of '{}' is {}.", string, length);
    }

    println!("\n====================== Mutable References ====================");
    {
        let mut string: String = String::from("hello");

        change(&mut string);

        println!("Changed value: '{string}'")
    }
}

// string is a reference to a String
fn calculate_length(string: &String) -> usize {
    // string goes out of scope
    // because it does not have ownership of what it refers to, it is not dropped
    string.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}