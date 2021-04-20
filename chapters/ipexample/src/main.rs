enum IpAddrKind {
    V4{x:i32},
    V6,
}

fn print_ip(x: IpAddrKind) {
    let b = match x {
        IpAddrKind::V4{x} => {
            let s1 = String::from("X is ");
            let s2 = x.to_string();
            s1 + &s2
        },
        IpAddrKind::V6 => String::from("bai"),
    };

    println!("{}", b)
}

fn main() {
    let four = IpAddrKind::V4{x: 10};
    let six = IpAddrKind::V6;

    print_ip(four);
    print_ip(six);
    println!("Hello, world!");
}
