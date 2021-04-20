use std::collections::HashMap;

fn main() {
    let mut v = vec![1,2,3,4,5];
    let m = mean(&v);
    let med = median(& mut v);
    let mo = mode(&v);
    println!("Mean {}, Median: {}, Mode {}", m, med, mo)
}

fn mean(x: &Vec<i32>) -> i32 {
    let mut b: i32 = 0;
    let mut counter: i32 = 0;
    for val in x {
        b += val;
        counter+=1;
    }

    return b/counter;
}

fn median(x: & mut Vec<i32>) -> i32 {
    x.sort();
    println!("{:?}", x);
    let val = x.len();

    if val % 2 == 0 && val > 1 {
        let a = x[x.len()/2-1];
        let b = x[(x.len()/2)];
        println!("{}, {}", a, b);
        return (a+b)/2;
    }
    let index = x.len()/2;
    return x[index];
}

fn mode(x : &Vec<i32>) -> i32 {
    let mut counter = HashMap::new();

    for i in x {
        let result = counter.entry(i).or_insert(0);
        *result += 1;
    }

    let mut max_key = x[0];
    let mut max_value = x[0];
    for (k, v) in counter {
        if v > max_value {
            max_value = v;
            max_key = *k;
        }
    }

    return max_key
}