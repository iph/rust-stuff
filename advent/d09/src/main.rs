use clap::{App, Arg};
use std::fs;

// FullCipher is all the numbers in the cipher to be "understood", but doesn't do the core
// algorithm which traverses through these numbers.
#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct FullCipher {
    numbers: Vec<i64>
}

impl FullCipher {
    fn new(content: &str) -> Result<FullCipher, String> {
        let mut numbers: Vec<i64> = Vec::new();
        for line in content.lines() {
            let num: i64 = match line.parse() {
                Ok(i) => i,
                Err(_) => return Err(format!("Could not parse {}", line))
            };

            numbers.push(num)
        }

        return Ok(FullCipher{numbers});
    }
}


struct MovingCipher {
    full_cipher: FullCipher,

    // These define the "sliding window" into the full cipher.
    start_position: usize,
    end_position: usize,

    // window_size is the amount the sliding window can be. In the program, it's referred to as
    // the "preamble", but the preamble slides along with the cipher, which makes it a window :)
    window_size: u32,
}

impl MovingCipher {
    fn new(window: u32, content: &str) -> Result<MovingCipher, String> {
        let full_cipher = FullCipher::new(content)?;

        if window > full_cipher.numbers.len() as u32 {
            return Err("Don't support windows that large".to_string());
        }

        return Ok(MovingCipher{full_cipher, window_size: window, start_position: 0, end_position: window as usize});
    }

    fn find_cracks(& mut self) -> Result<i64, String> {

        loop {
            let next_pos = self.end_position;
            let next_num: i64 = match self.full_cipher.numbers.get(next_pos) {
                Some(i) => { i.clone() }
                None => {return Err("Reached end of the line".to_string())}
            };

            let legit = self.within_pool(next_num)?;

            if legit {
                self.move_window();
            } else {
                return Ok(next_num)
            }
        }

    }

    fn within_pool(&self, num: i64) -> Result<bool, String> {
        let current_pool = match self.calculate_pool(){
            Some(p) => p,
            None => return Err("Pool doesn't exist".to_string()),
        };

        for i in current_pool.iter() {
            if num == i.clone() {
                return Ok(true);
            }
        }

        return Ok(false);

    }

    fn move_window(& mut self) {
        self.start_position += 1;
        self.end_position += 1;
    }

    fn calculate_pool(&self) -> Option<Vec<i64>> {
        let mut pool = Vec::new();

        for i in self.start_position..self.end_position {
            for j in i+1..self.end_position {
                if i == j {
                    continue;
                }

                let x = self.full_cipher.numbers.get(i)?;
                let y = self.full_cipher.numbers.get(j)?;
                pool.push((x+y) as i64);
            }
        }

        return Some(pool);
    }

    fn see_window(&self) -> Vec<i64> {
        let mut v = Vec::new();
        for i in self.start_position..self.end_position {
            let x = self.full_cipher.numbers.get(i).unwrap();
            v.push(x.clone());
        }
        return v;
    }
}

#[test]
fn test_rolling(){
    let content = "\
35
20
15
";
    let cipher = MovingCipher::new(3, content).unwrap();
    assert_eq!(vec![55, 50, 35], cipher.calculate_pool().unwrap())

}

#[test]
fn test_rolling_window(){
    let content = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";

    let mut cipher = MovingCipher::new(5, content).unwrap();
    println!("{}", cipher.find_cracks().unwrap());

}


struct GrowingCipher {
    full_cipher: FullCipher,
    size: usize,
    start: usize,
}

impl GrowingCipher {
     fn new(start: usize, full_cipher: FullCipher) -> Result<GrowingCipher, String> {
        if start >= full_cipher.numbers.len() {
            return Err("Don't support starting positions that large".to_string());
        }

        return Ok(GrowingCipher{full_cipher, size: 2, start});
    }

    fn is_contiguous(&mut self, goal: i64) -> bool {
        loop {
            let mut accumulator = 0;
            
            for i in self.start..self.start+self.size{
                accumulator = match self.full_cipher.numbers.get(i){
                    Some(x) => x + accumulator,
                    None => return false,
                }
            }

            if accumulator == goal {
                return true;
            }

            if accumulator > goal {
                return false;
            }

            self.size += 1;
        }
    }
}

#[test]
fn test_growing_cipher(){
    let content = "\
35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
";
    let full_cipher = FullCipher::new(content).unwrap();
    
    for i in 0..full_cipher.numbers.len(){
        let mut cipher = GrowingCipher::new(i, full_cipher.clone()).unwrap();
        if cipher.is_contiguous(127) {
            println!("WE GOTTA {}, {}", cipher.start, cipher.size);
            let mut smallest = full_cipher.numbers[cipher.start];
            let mut largest = full_cipher.numbers[cipher.start];


            for index in cipher.start..cipher.start+cipher.size {
                if full_cipher.numbers[index] > largest {
                    largest = full_cipher.numbers[index]
                }

                if full_cipher.numbers[index] < smallest {
                    smallest = full_cipher.numbers[index]
                }
            }

            println!("With my numbers combined: {}, {}, {}", smallest, largest, smallest+largest);
            break;
        }
    }
    

}


fn main() {
    let matches = App::new("Reader for advent")
        .version("1.0")
        .author("Sean Tyler Myers <seanmyers0608@gmail.com>")
        .about("Reads advent calendar programs")
        .arg(
            Arg::new("INPUT")
                .about("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let txt_location: &str = matches.value_of("INPUT").unwrap();
    let contents = fs::read_to_string(txt_location).unwrap();
    let mut cipher = MovingCipher::new(25, &contents).unwrap();
    let s = cipher.find_cracks().unwrap();
    println!("{}", s);

    let full_cipher = FullCipher::new(&contents).unwrap();
    
    for i in 0..full_cipher.numbers.len(){
        let mut cipher = GrowingCipher::new(i, full_cipher.clone()).unwrap();
        if cipher.is_contiguous(s) {
            println!("The strand {}, {}", cipher.start, cipher.size);
            let mut smallest = full_cipher.numbers[cipher.start];
            let mut largest = full_cipher.numbers[cipher.start];


            for index in cipher.start..cipher.start+cipher.size {
                println!("Number: {}", full_cipher.numbers[index]);
                if full_cipher.numbers[index] > largest {
                    largest = full_cipher.numbers[index]
                }

                if full_cipher.numbers[index] < smallest {
                    smallest = full_cipher.numbers[index]
                }
            }

            println!("With my numbers combined: {}, {}, {}", smallest, largest, smallest+largest);
            break;
        }
    }
    

}
