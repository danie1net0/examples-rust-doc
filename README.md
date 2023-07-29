# 1. Getting started

## 1.1. Installation

### Installing on Linux or MacOS
```
curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
```

### Updating and uninstalling

To update:
```
rustup update
```

To uninstall:
```
rustup self uninstall
```

### Local documentation
```
rustup doc
```

## 1.2. Running and compiling

### Running a Rust program

To compile the program:
```
rustc main.rs
```

To run the binary:
```
./main
```

## 1.3. Cargo

### Creating a project:
```
cargo new project_name
```

### Building and running:
To build the project:
```
cargo build
```

To run the binary:
```
./target/debug/project_name
```

To run the project without compile and execute in different steps:
```
cargo run
```

To quickly checks the code to make sure it compiles but doesn’t produce an executable:
```
cargo check
```

### Building for release:
To compile the project with optimizations: 
```
cargo build --release
```

# 2. Common programming concepts

## 2.1. Variables and mutability

### Variables
Are immutables by default
  * Are preceded by the `let` keyword: `let x: u8 = 4;`
  * Must be in snake case pattern: `let some_var: u8 = 5;`
  * The keyword `mut` should be used to define a variable as mutable

### Constants
Are always immutables, it is never possible to change their value

  * Can be used in global scope
  * May be set only to a constant expression, not the result of a value that could only be computed at runtime
  * Are preceded by the `const` keyword: `const X: u8 = 4;`
  * Must be in all uppercase with underscores between words: `const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;`

### Shadowing
Are declared as a new variable with the same name as a previous variable:
```
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
```

* Result in a compile-time error if we accidentally try to reassign to this variable without using the `let` keyword
* Effectively creates a new variable
* The type can't be changed, unlike the `let` keyword

## 2.2. Compound types

Compound types can group multiple values into one type. Rust has two primitive compound types: tuples and arrays.

### Tuple type

A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.

```
fn main() {
    let tuple: (i32, f64, u8) = (500, 6.4, 1);
}
```

### Array type

Another way to have a collection of multiple values is with an array. Unlike a tuple, every element of an array must have the same type. Unlike arrays in some other languages, arrays in Rust have a fixed length.

```
fn main() {
    let array = [1, 2, 3, 4, 5];
}
```

## 2.3. Statements and Expressions

**Statements** are instructions that perform some action and do not return a value:
```
fn main() {
    let y = 6;
}
```

**Expressions** evaluate to a resultant value:
```
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}
```

The expression:
```
{
    let x = 3;
    x + 1
}
```

# 3. References and Borrowing

## 3.1. Mutable References

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other
references to that value.

This code that attempts to create two mutable references to `string` will fail because we cannot borrow `string` as 
mutable more than once at a time:
```
let mut string = String::from("hello");

let reference_a = &mut string;
let reference_b = &mut string;

println!("{}, {}", reference_a, reference_b);
```

We can use curly brackets to create a new scope, allowing for multiple mutable references, just not simultaneous ones:
```
let mut string = String::from("hello");

{
    // reference_a goes out of scope here, so we can make a new reference with no problems
    let reference_a = &mut string;
} 

let reference_b = &mut string;
```

Mutable and immutable references cannot be combined in sequence, this code will fail:
```
let mut string = String::from("hello");

let reference_a = &string; // no problem
let reference_b = &string; // no problem
let reference_c = &mut string; // BIG PROBLEM

println!("{}, {}, and {}", reference_a, reference_b, reference_c);
```

This code already works because `reference_a` and `reference_b` go out of scope before the declaration of `reference_c`:
```
let mut string = String::from("hello");

let reference_a = &string; // no problem
let reference_b = &string; // no problem

// variables reference_a and reference_b will not be used after this point
println!("{} and {}", reference_a, reference_b);

let reference_c = &mut string; // no problem
println!("{}", reference_c);
```

## 3.2. Dangling References

The Rust compiler guarantees that references will never be dangling references: if you have a reference to some data, it
will ensure that the data will not go out of scope before the reference to the data does.

This code will not work because when the `dangle` function is finished it will try to return a reference for `string` which will 
be deallocated at the end of the function:
```
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let string = String::from("hello");

    &string
}
```
the soluction for that is to return a `String` directly:
``` 
fn no_dangle() -> String {
    let string = String::from("hello");

    string
}
```