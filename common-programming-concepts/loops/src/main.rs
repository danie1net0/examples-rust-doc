use std::{thread, time};

fn main() {
    println!("---------- RETURNING VALUES ---------");

    let mut counter: u8 = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("the result is {result}");

    println!("\n------------ LOOP LABELS ------------");

    let mut count: u8 = 0;

    'counting_up: loop {
        println!("count = {count}");

        let mut remaining: u8 = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("end count = {count}");

    println!("\n--------- CONDITIONAL LOOPS ---------");

    let mut number: u8 = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");

    println!("\n-------- LOOPING A COLLECTION -------");

    let array: [u8; 5] = [10, 20, 30, 40, 50];

    for element in array {
        println!("the value is: {element}");
    }

    println!("\n--------------- RANGE ---------------");

    for number in (1..4).rev() {
        println!("{number}!");
    }

    println!("LIFTOFF!!!");

    println!("\n----------- INFINITE LOOP -----------");

    loop {
        println!("again!");
        thread::sleep(time::Duration::from_millis(1000))
    }
}
