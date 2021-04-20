#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

impl User {
    fn email(&self) -> &str {
        return &self.email[..]
    }

    fn new_user(username: String, email: String) -> User{
        return User{
            username,
            email,
            sign_in_count: 1,
            active: true,
        }
    }
}

fn main() {
    let user1 = User{
        username: String::from("someusername123"),
        email: String::from("seanmyers0608@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("Username: {}", user1.username);

    let mut _user1 = User{
        username: String::from("someusername123"),
        email: String::from("seanmyers0608@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    let user = User::new_user(String::from("hi"), String::from("bye"));

    println!("Username: {:#?}", user.email());
}
