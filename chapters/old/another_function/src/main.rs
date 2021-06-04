fn main() {
    another_function(5, 10);
    not_main()
}

fn not_main(){
    let my_string = String::from("hello world");

    let word = first_word(&my_string[..]);
    println!("The first word is {}", word);
    let my_string_literal = "hello world";
    let word = first_word(my_string_literal);
    println!("The first word is {} of {}", word, my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn another_function(x:i32, y:i32){
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
}
