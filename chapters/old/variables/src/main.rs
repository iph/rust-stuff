fn main() {
    let a = [1,2,3,4,5];
    let mut b = [1,2,3,4,5];

    b[0] = 2;

    let mut x = 5;
    println!("The value of x is {}", x);

    x = 6;
    println!("The value of b is {}", b[0]);
}
