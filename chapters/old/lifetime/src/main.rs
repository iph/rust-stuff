fn main() {
    let a = String::from("Hello");
    let b = String::from("World");

    println!("{}", longest(&a, &b));
}

fn longest<'a> (x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}