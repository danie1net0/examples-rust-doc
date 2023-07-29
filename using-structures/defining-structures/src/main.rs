struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(String, i32, i32, i32);

fn main() {
    let mut user_a = User {
        active: true,
        username: String::from("user_a"),
        email: String::from("user_a@example.com"),
        sign_in_count: 1,
    };
    print_user(&user_a);

    user_a.email = String::from("changed_user_a@example.com");
    print_user(&user_a);

    let user_b = build_user("user_b".to_string(), "user_b@example.com".to_string());
    print_user(&user_b);

    let user_c = User {
        // username: String::from("user_c"),
        email: String::from("user_c@example.com"),
        ..user_a
    };

    print_user(&user_c);

    // don't work because user_a has been moved to user_c
    // print_user(&user_a);

    let color = Color("black".to_string(), 0, 0, 0);
    println!("{} = red: {}, green: {}, blue: {}", color.0, color.1, color.2, color.3);
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

fn print_user(user: &User) -> () {
    println!("Username: {}", user.username);
    println!("E-mail: {}", user.email);
    println!("Is active: {}", user.active);
    println!("Sign in count: {}\n", user.sign_in_count);
}
