struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User{
        username: String::from("someusername123"),
        email: String::from("seanmyers0608@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username);

    let mut user1 = User{
        username: String::from("someusername123"),
        email: String::from("seanmyers0608@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    user1.username = String::from("hi");

    println!("Username: {}", user1.username);
}
