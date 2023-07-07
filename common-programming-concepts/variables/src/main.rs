const GLOBAL_CONST: u32 = 2 * 2;

fn main() {
    println!("------------- VARIABLES -------------");
    let mut x = 5;

    println!("The value of x is: {x}");

    x = 6;

    println!("The value of x is: {x}");

    println!("------------- CONSTANTS -------------");
    const LOCAL_CONST: u32 = 3 * 3;

    println!("GLOBAL_CONST={GLOBAL_CONST}");
    println!("LOCAL_CONST={LOCAL_CONST}");

    println!("------------- SHADOWING -------------");
    let y = 5;

    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {y}");
    }

    println!("The value of y is: {y}");
}
