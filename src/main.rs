use regex::Regex;
use std::io;
use std::io::{Error, ErrorKind};

mod executor;
use executor::constants::*;
use executor::*;

fn interpret(line: String) -> Result<i32, Error> {
    let pattern = Regex::new(r"(?P<lh>\d+)\s*(?P<op>[+*-/^])\s*(?P<rh>\d+)").unwrap();

    if !pattern.is_match(&line.trim()) {
        return Err(Error::new(
            ErrorKind::InvalidData,
            "Did not understand {line}",
        ));
    }
    let mut res: i32 = 0;
    for cap in pattern.captures_iter(&line.trim()) {
        let left = cap
            .name("lh")
            .expect("Unable to parse left hand character")
            .as_str()
            .parse::<i32>()
            .expect("Unable to parse left hand side as a number");

        let operator_character = cap
            .name("op")
            .expect("Unable to parse operator character")
            .as_str();

        let right = cap
            .name("rh")
            .expect("Unable to parse right hand character")
            .as_str()
            .parse::<i32>()
            .expect("Unable to parse right hand side as a number");

        let operator = match operator_character {
            "+" => Operator::Addition,
            "-" => Operator::Subtraction,
            "*" => Operator::Multiplication,
            "/" => Operator::Division,
            "^" => Operator::Exponentiation,
            _other => {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Unknown operator character",
                ))
            }
        };

        let result = execute(left, operator, right);

        println!("{result}");

        res = result;
    }
    return Ok(res);
}

fn main() {
    println!(">>");

    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.as_str().trim() {
            "exit" => break,
            "x" => break,
            "quit" => break,
            _other => {}
        };

        let res = interpret(line);

        if res.is_err() {
            println!("{:#?}", res);
        }
    }
}
