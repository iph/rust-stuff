use clap::{App, Arg};
use std::fs;

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
struct JoltAdapter {
    rating: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Eq, Ord)]
struct JoltLeap {
    amount: i32
}

impl JoltAdapter {
    fn new(rating: i32) -> JoltAdapter{
        return JoltAdapter{rating};
    }

    // capacitance is the amount the incoming jolts are allowed to jump to.
    fn handle(&self, incoming: i32, capacitance: i32) -> Option<JoltLeap> {
        if incoming >= self.rating - capacitance {
            return Option::Some(JoltLeap{amount: self.rating - incoming})
        }

        return Option::None;
    }
}

#[test]
fn test_jolt_capacitance(){
    let adapter = JoltAdapter::new(4);
    assert_eq!(Option::Some(JoltLeap{amount: 3}), adapter.handle(1, 3));
    assert_eq!(Option::None, adapter.handle(0, 3));
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct JoltSolver {
    adapters: Vec<JoltAdapter>,
    start: i32
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct JoltSolution1 {
    path: Vec<JoltLeap>,
    jolt_3: i32,
    jolt_1: i32
}

impl JoltSolution1 {
    fn new() -> JoltSolution1 {
        JoltSolution1{
            path: Vec::new(),
            jolt_1: 0,
            jolt_3: 0
        }
    }

    fn add(&mut self, leap: JoltLeap) {
        if leap.amount == 3 {
            self.jolt_3 += 1;
        }

        if leap.amount == 1 {
            self.jolt_1 += 1;
        }

        self.path.push(leap);
    }
}


#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct JoltBag {
    path: Vec<JoltAdapter>,
    current: i32
}

impl JoltBag {
    fn new(current: i32) -> JoltBag {
        JoltBag{
            path: Vec::new(),
            current
        }
    }
    fn add(&self, adapter: JoltAdapter) -> Option<JoltBag> {
        return match adapter.handle(self.current, 3) {
            Some(x) => {
                let mut cloned = self.clone();
                cloned.current = adapter.rating;
                cloned.path.push(adapter);
                Option::Some(cloned)
            },
            None => {
                Option::None
            }
        }
    }
}


impl JoltSolver {
    fn parse(contents: String, current: i32) -> JoltSolver{
        let mut adapters: Vec<JoltAdapter> = Vec::new();

        for x in contents.lines() {
            let rating = x.parse::<i32>().unwrap();
            let adapter = JoltAdapter::new(rating);
            adapters.push(adapter);
        }

        adapters.sort();
        let highest_device = adapters.get(adapters.len()-1).unwrap();
        adapters.push(JoltAdapter{rating: highest_device.rating+3});

        return JoltSolver{
            adapters,
            start: current
        }
    }

    fn solve(&self) -> Result<JoltSolution1, &str> {
        let mut current = self.start;
        let mut solution = JoltSolution1::new();
        for x in self.adapters.iter() {
            let leap = x.handle(current, 3);

            let real_leap = match leap {
                Some(x) => x,
                None => return Result::Err("No value provided for this instance")
            };

            current = x.rating;
            solution.add(real_leap)

        }
        return Result::Ok(solution);
    }

    fn solve2(&self) -> Result<Vec<JoltBag>, &str> {
        let mut solutions: Vec<JoltBag> = Vec::new();

        let mut jb = JoltBag::new(self.start);
        solutions.push(jb);
        for x in self.adapters.iter(){
            let mut next_iter: Vec<JoltBag> = Vec::new();
            for solution in solutions.clone() {
                let possible_path = solution.clone();
                match solution.add(x.clone()) {
                    Some(x) => {
                        next_iter.push(possible_path);
                        next_iter.push(x);
                    },
                    None => {}
                };
            }
            solutions = next_iter;
        }

        return Result::Ok(solutions)
    }
}

#[test]
fn test_jolt_solver(){
    let content = "\
16
10
15
5
1
11
7
19
6
12
4
";
    let mut solver = JoltSolver::parse(content.to_owned(), 0);
    assert_eq!(vec![
        JoltAdapter::new(1),
        JoltAdapter::new(4),
        JoltAdapter::new(5),
        JoltAdapter::new(6),
        JoltAdapter::new(7),
        JoltAdapter::new(10),
        JoltAdapter::new(11),
        JoltAdapter::new(12),
        JoltAdapter::new(15),
        JoltAdapter::new(16),
        JoltAdapter::new(19),
        // Device adapter itself, it's never in the actual list.
        JoltAdapter::new(22),
    ], solver.adapters);

    let result = solver.solve().unwrap();

    assert_eq!(7, result.jolt_1);
    assert_eq!(5, result.jolt_3);
}

#[test]
fn test_jolt_solver_part2(){
    let content = "\
16
10
15
5
1
11
7
19
6
12
4
";
    let mut solver = JoltSolver::parse(content.to_owned(), 0);
    let result = solver.solve2().unwrap();
    let len = result.len();
    println!("{:?}", result);
    for solution in result.iter() {
        for adapter in solution.path.iter() {
            print!("{} ", adapter.rating)
        }
        println!()
    }
    assert_eq!(8, len/2);
}


#[test]
fn test_jolt_solver_longer(){
    let content = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
    let mut solver = JoltSolver::parse(content.to_owned(), 0);
    let result = solver.solve().unwrap();

    assert_eq!(22, result.jolt_1);
    assert_eq!(10, result.jolt_3);
}

#[test]
fn test_jolt_solver2_longer(){
    let content = "\
28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3
";
    let mut solver = JoltSolver::parse(content.to_owned(), 0);
    let result = solver.solve2().unwrap();

    assert_eq!(19208, result.len()/2);
}


fn main() {
    let matches = App::new("Reader for advent")
        .version("1.0")
        .author("Sean Tyler Myers <seanmyers0608@gmail.com")
        .about("Reads advent calendar programs")
        .arg(
            Arg::new("Input")
                .about("Sets the input file to use")
                .required(true)
                .index(1)
        )
        .get_matches();

    let txt_location = matches.value_of("INPUT").unwrap();
    let contents = fs::read_to_string(txt_location).unwrap();
    let solver = JoltSolver::parse(contents.to_owned(), 0);
    let solution1 = solver.solve().unwrap();

    println!("{} * {} = {}", solution1.jolt_1, solution1.jolt_3, solution1.jolt_1 * solution1.jolt_3);

    let solution2 = solver.solve2().unwrap();
    println!("{}", solution2.len()/2)
}
