fn main() {
    println!("\n================== The String Type =================");
    {
        let mut s: String = String::from("hello");

        s.push_str(", world!");

        println!("s = {s}");
    }

    println!("\n===== Variables and Data Interacting with Move =====");
    {
        let s1: String = String::from("hello");

        // s1 was moved to s2 and s1 goes out of scope
        let s2: String = s1;

        // doesn't work because s1 goes out of scope
        // println!("{}, world!", s1);

        println!("s2 = {s2}, world!");
    }

    println!("\n==== Variables and Data Interacting with Clone =====");
    {
        let s1: String = String::from("hello");
        let mut s2: String = s1.clone();

        s2.push_str(", world!");

        println!("s1 = {s1}, s2 = {s2}");
    }

    println!("\n=============== Stack-Only Data: Copy ==============");
    {
        let mut x: u8 = 5;

        // y takes a copy of x and it works because integers implements the Copy trait
        let y: u8 = x;

        x = 7;

        println!("x = {x}\ny = {y}");
    }

    println!("\n============== Ownership and Functions =============");
    {
        let s: String = String::from("hello");

        // s's value moves into the function and so is no longer valid here
        takes_ownership(s);

        // doesn't work because the s's ownership was taked by the takes_ownership function
        // println!("s = {s}");

        let x: u8 = 5;

        // x would move into the function, but i32 is Copy, so it's okay to still use x afterward
        makes_copy(x);

        println!("x = {x}");

        // x goes out of scope, then s
        // because s's value was moved, nothing special happens
    }

    println!("\n============== Return Values and Scope =============");
    {
        // gives_ownership moves its return value into s1
        let s1: String = gives_ownership();

        // s2 comes into scope
        let s2: String = String::from("hello");

        // s2 is moved into  takes_and_gives_back, which also moves its return value into s3
        let s3: String = takes_and_gives_back(s2);

        println!("s1 = {s1}");
        // doesn't work because s2 alrealdy goes out of scope
        // println!("s2 = {s2}");
        println!("s3 = {s3}");

        // s3 goes out of scope and is dropped.
        // s2 was moved, so nothing happens.
        // s1 goes out of scope and is dropped.
    }

    println!();
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);

    // some_string goes out of scope and `drop` is called
    // the backing memory is freed.
}

fn makes_copy(some_integer: u8) { // some_integer comes into scope
    println!("{}", some_integer);

    // some_integer goes out of scope
    // nothing special happens
}

// gives_ownership will move its return value into the function that calls it
fn gives_ownership() -> String {
    // some_string comes into scope
    let some_string = String::from("yours");

    // some_string is returned and  moves out to the calling function
    some_string
}

// takes_and_gives_back takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into scope

    // a_string is returned and moves out to the calling function
    a_string
}