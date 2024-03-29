use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Visitor {
        Visitor{
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }

    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn main() {
    let visitor_list = [
        Visitor::new("bert", "Hello Bert, enjoy your treehouse."),
        Visitor::new("steve", "Hi Steve. Your milk is in the fridge."),
        Visitor::new("fred", "Wow, who invited Fred?"),
    ];
    println!("Hello, what's your name?");
    let name = what_is_your_name();

    println!("Hello, {}", name);
    let res = visitor_list.iter().find(|visitor| visitor.name == &name);

    match res {
        None => {
            println!("You are not on the list, {}", name);
        }
        Some(v) => {
            visitor.unwrap().greet_visitor();
        }
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}
