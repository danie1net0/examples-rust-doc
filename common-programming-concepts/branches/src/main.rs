fn main() {
    println!("----------- IF EXPRESSIONS -----------");

    let number: u8 = 6;

    if number % 4 == 0 {
        println!("Number is divisible by 4");
    } else if number % 3 == 0 {
        println!("Number is divisible by 3");
    } else if number % 2 == 0 {
        println!("Number is divisible by 2");
    } else {
        println!("Number is not divisible by 4, 3, or 2");
    }

    println!("\n------- IF IN A LET STATEMENT --------");
    let condition: bool = true;
    let number: u8 = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}
