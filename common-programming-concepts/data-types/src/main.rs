use std::{array, mem};

fn main() {
    println!("------------- SCALAR TYPES -------------");

    let integer: u8 = 42;
    let floating_point: f64 = 3.14;
    let character: char = '😻';
    let boolean: bool = true;
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tuple;
    let array: [i32; 3] = [1, 2, 3];

    println!("Integer type (u8): {integer} - {} byte(s)", mem::size_of_val(&integer));
    println!("Floating point type (f64): {floating_point} - {} byte(s)", mem::size_of_val(&floating_point));
    println!("Character type: {character} - {} byte(s)", mem::size_of_val(&character));
    println!("Boolean type: {boolean} - {} byte(s)", mem::size_of_val(&boolean));
    println!("Tuple type: ({x}, {y}, {}) - {} byte(s)", tuple.2, mem::size_of_val(&tuple));
    println!("Array type: [{}, {}, {}] - {} byte(s)", array[0], array[1], array[2], mem::size_of_val(&array));

    // Invalid element access: this result in a runtime error
    // let element = array[3];

    println!("\n---------- NUMERIC OPERATIONS ----------");
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3;
    let remainder = 43 % 5;

    println!("Adition: 5 + 10 = {sum}");
    println!("Subtraction: 95.5 - 4.3 = {difference}");
    println!("Multiplication: 4 * 30 = {product}");
    println!("Division (quotient): 56.7 / 32.2 = {quotient}");
    println!("Division (truncated): -5 / 3 = {truncated}");
    println!("Remainder 43 % 5 = {remainder}");
}
