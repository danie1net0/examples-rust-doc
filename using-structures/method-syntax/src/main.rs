#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other_rectangle: &Rectangle) -> bool {
        self.width > other_rectangle.width && self.height > other_rectangle.height
    }
}

fn main() {
    let rectangle_a = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle_b = Rectangle {
        width: 10,
        height: 40,
    };
    let rectangle_c = Rectangle {
        width: 60,
        height: 45,
    };

    if rectangle_a.width() {
        println!("The rectangle has a nonzero width; it is {}\n", rectangle_a.width);
    }

    println!("The area of the rectangle is {} square pixels.\n", rectangle_a.area());

    println!("Can rectangle_a hold rectangle_b? {}", rectangle_a.can_hold(&rectangle_b));
    println!("Can rectangle_a hold rectangle_c? {}\n", rectangle_a.can_hold(&rectangle_c));

    let square = Rectangle::square(10);

    println!("Square: {:#?}", square);
}
