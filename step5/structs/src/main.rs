struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user.email = {}", user.email);

    let mut mutable_user = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    mutable_user.email = String::from("hello@world.com");
    println!("user.email = {}", mutable_user.email);

    fn build_user(email: String, username: String) -> User {
        User {
            email, // if variable name is same with field name, you don't have to specify the name.
            username,
            active: true,
            sign_in_count: 1,
        }
    }

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
        // above is same with -> active: user1.active, sign_in_count: user1.sign_in_count,
    };
}

fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
