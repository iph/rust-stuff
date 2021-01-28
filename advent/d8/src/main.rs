use clap::{App, Arg};
use nom::character::complete::{digit1, space1};
use nom::error::ErrorKind;
use nom::lib::std::collections::HashMap;
use nom::{sequence::tuple, IResult};
use std::fs;
use nom::sequence::terminated;
use nom::multi::many1;
use crate::Instruction::{Nop, Jmp, Acc};
use nom::bytes::complete::{is_a, take, tag};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
enum Instruction{
    Nop(i32),
    Jmp(i32),
    Acc(i32),
}

impl Instruction {
}


#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Program {
    instructions: Vec<Instruction>,
}
#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct RanInstruction {
    instruction: Instruction,
    pointer: i32,
}

#[derive(Debug, PartialEq, PartialOrd, Clone)]
struct Computer {
    program: Program,
    accumulator: i32,
    instruction_pointer: i32,
}

impl Computer { 

    fn next(& mut self) -> Result<RanInstruction, String>{
        let current_pointer = self.instruction_pointer;
        let ins = match self.program.instructions.get(current_pointer as usize) {
            Some(i) => i,
            None => return Err(format!("Incorrect instruction {}", current_pointer)),
        };
        let future_point = match ins {
            Nop(_) => current_pointer+1,
            Jmp(i) => current_pointer+i,
            Acc(i) => {
                self.accumulator += i;
                current_pointer+1
            },
        };

        self.instruction_pointer = future_point;
        return Ok(RanInstruction {instruction: ins.clone(), pointer: current_pointer});
    }

    fn new(content: &str) -> Computer {
        let program = parse_program(content).unwrap();
        return Computer{program, accumulator: 0, instruction_pointer: 0};
    }

    fn run(& mut self) -> Option<i32>{
        let mut m = HashMap::new();
        loop {
            if m.contains_key(&self.instruction_pointer) {
                return Some(self.accumulator);
            }

            let ran_instruction = match self.next() {
                Ok(ins) => ins,
                Err(_) => return None,
            };
            m.insert(ran_instruction.pointer, true);
        }
    }
}

fn parse_program(contents: &str) -> Result<Program, nom::Err<nom::error::Error<&str>>>{
    let mut full_program = many1(terminated(parse_line, tag("\n")));

    if let Err(h) = full_program(contents) {
        return Err(h);
    }

    let (_, prog) = full_program(contents)?;
    return Ok(Program{instructions: prog});
}

fn parse_line(i: &str) -> IResult<&str, Instruction> {
    let mut line = tuple((
        take(3usize),
        space1,
        is_a("+-"),
        digit1
    ));

    let (s, (operation, _, operand, amount)) = line(i)?;

    let operation_amount = match amount.parse() {
        Ok(q) => q,
        Err(_) => return Err(nom::Err::Failure(nom::error::make_error(s, ErrorKind::Digit)))
    };

    let operation_amount = match operand {
        "+" => operation_amount,
        "-" => operation_amount * -1,
        _ => { panic!("Impossible operand")}
    };

    let op = match operation {
        "nop" => Nop(operation_amount),
        "jmp" => Jmp(operation_amount),
        "acc" => Acc(operation_amount),
        _ => return Err(nom::Err::Failure(nom::error::make_error(s, ErrorKind::TakeWhile1)))
    };

    Ok((s, op))
}

#[test]
fn test_program(){
    let contents = "\
nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
";

    let mut computer = Computer::new(contents);
    let accum = computer.run();
    assert_eq!(5, accum)

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
    let mut computer = Computer::new(&contents);
    let accum = computer.run();
    println!("{}", accum.unwrap());

    let computer = Computer::new(&contents);

    for (loc, ins) in computer.program.instructions.iter().enumerate() {
        let swapped_ins = match ins {
            Nop(i) => Some(Instruction::Jmp(i.clone())),
            Jmp(i) => Some(Instruction::Nop(i.clone())),
            Acc(_) => None,
        };

        if let Some(x) = swapped_ins {
            let mut cloned = computer.clone();
            cloned.program.instructions[loc] = x;
            match cloned.run() {
                Some(_) => continue,
                None => {
                    println!("The correct mutation has been found, accumulator at {}", cloned.accumulator)
                }
            }
        } else {
            continue;
        }
    }
}
