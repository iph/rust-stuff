use clap::{App, Arg};
use std::fs;

#[derive(Debug, PartialEq, Clone)]
enum Space {
    Clear,
    Tree,
}

#[derive(Debug, PartialEq, Clone)]
struct Direction {
    right: i32,
    down: i32,
}

impl Space {
    // parses a space in the board game, which looks like..
    // ..##....... <<< where . is a clearing and # is a tree.
    // we don't allow boards that have any other symbol.
    fn parse(x: char) -> Result<Space, &'static str> {
        match x {
            '#' => return Ok(Space::Tree),
            '.' => return Ok(Space::Clear),
            _ => return Err("Not a parsable character"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
struct InfiniteRow {
    columns: Vec<Space>,
}

impl InfiniteRow {
    fn new(spaces: Vec<Space>) -> InfiniteRow {
        return InfiniteRow { columns: spaces };
    }

    fn get(&self, i: i32) -> Space {
        let board_space: usize = i as usize % self.columns.len();
        return self.columns[board_space].clone();
    }
}

#[test]
fn check_infinite_row_continues_forever() {
    let contents = vec![Space::Tree, Space::Clear, Space::Clear];
    let row = InfiniteRow::new(contents);

    assert_eq!(row.get(0), Space::Tree);
    assert_eq!(row.get(3), Space::Tree);
    assert_eq!(row.get(4), Space::Clear);
}

#[derive(Debug, PartialEq, Clone)]
struct InfiniteBoard {
    rows: Vec<InfiniteRow>,
}

#[derive(Debug, PartialEq, Clone)]
struct ObservedPath {
    observed: Vec<Space>,
}

impl InfiniteBoard {
    fn new(contents: String) -> Result<InfiniteBoard, &'static str> {
        let mut all_rows = Vec::new();
        for line in contents.lines() {
            let mut row = Vec::new();
            for c in line.trim().chars() {
                let space = match Space::parse(c) {
                    Ok(s) => s,
                    Err(e) => return Err(e),
                };

                row.push(space);
            }

            all_rows.push(InfiniteRow::new(row));
        }

        return Ok(InfiniteBoard { rows: all_rows });
    }

    fn traverse(&self, direction: Direction) -> ObservedPath {
        let mut current_row: i32 = 0;
        let mut current_column: i32 = 0;
        let mut observed: Vec<Space> = Vec::new();

        loop {
            current_row += direction.down;
            current_column += direction.right;

            if current_row as usize >= self.rows.len() {
                break;
            }

            let row: &InfiniteRow = self.rows.get(current_row as usize).unwrap();

            let space = row.get(current_column);
            observed.push(space);
        }

        let op = ObservedPath { observed: observed };
        return op;
    }
}

#[test]
fn test_traverse() {
    let contents = "\
.#
..
##";
    let board = InfiniteBoard::new(contents.to_owned()).unwrap();
    let path = board.traverse(Direction { right: 1, down: 1 });
    assert_eq!(vec!(Space::Clear, Space::Tree), path.observed)
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
    let board = InfiniteBoard::new(contents).unwrap();

    // part 1
    let path = board.traverse(Direction { right: 3, down: 1 });

    let mut tree_accumulator = 0;
    for space in path.observed {
        match space {
            Space::Tree => tree_accumulator += 1,
            _ => {}
        };
    }

    println!("Trees seen {}", tree_accumulator);

    // part 2: Using a bunch of random directions, gather all the tree "summations" and multiply them together
    let all_puzzles = vec![
        Direction { right: 1, down: 1 },
        Direction { right: 3, down: 1 },
        Direction { right: 5, down: 1 },
        Direction { right: 7, down: 1 },
        Direction { right: 1, down: 2 },
    ];

    let mut mult_accumulator = 1;
    for puzzle in all_puzzles {
        let path = board.traverse(puzzle);

        let mut tree_accumulator = 0;
        for space in path.observed {
            match space {
                Space::Tree => tree_accumulator += 1,
                _ => {}
            };
        }
        mult_accumulator *= tree_accumulator;
    }

    println!("All trees seen in all paths: {}", mult_accumulator)
}
