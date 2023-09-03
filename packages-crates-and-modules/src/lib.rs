mod front_of_house;

mod back_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist(); // Absolute path

    // crate::front_of_house::hosting::add_to_waitlist(); // Absolute path

    // front_of_house::hosting::add_to_waitlist(); // Relative path

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

use std::fmt::{self, Result};
use std::io::{self, Result as IoResult};

fn function_a_with_same_type() -> fmt::Result {
    fmt::Result::Ok(())
}

fn function_b_with_same_type() -> io::Result<()> {
    io::Result::Ok(())
}

fn function_a_with_alias() -> Result {
    Result::Ok(())
}

fn function_b_with_alias() -> IoResult<()> {
    IoResult::Ok(())
}