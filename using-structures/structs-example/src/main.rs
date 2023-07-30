#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", area(&rectangle));
    println!("rectangle is {:?}", rectangle);
    println!("rectangle is {:#?}", rectangle);

    let scale: u32 = 2;
    let rectangle_debug = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rectangle_debug);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}