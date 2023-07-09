fn main() {
    println!("Hello, world!");

    another_function();

    another_function_with_parameter(5);

    println!("Returned value from function: {}", function_with_return());
}

fn another_function() {
    println!("Another function.");
}

fn another_function_with_parameter(x: i32) -> () {
    println!("The value of x is: {x}");
}

fn function_with_return() -> u8 {
    // let x: u8 = 6;
    //
    // x

    6 // is equivalent to the above code
}
