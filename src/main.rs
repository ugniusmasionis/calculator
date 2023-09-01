<<<<<<< HEAD
use std::io;
use regex::Regex;
use core::panic;
=======
use core::panic;
use regex::Regex;
use std::io;
>>>>>>> 0859bb9 (Initial commit)

fn main() {
    println!("Welcome to the calculator!");
    loop {
        println!("Enter an expression to calculate (e.g. 2 + 2):");
        let mut input = String::new();
<<<<<<< HEAD
        io::stdin().read_line(&mut input).expect("Failed to read input");
=======
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
>>>>>>> 0859bb9 (Initial commit)
        let input = input.trim();

        if input == "exit" {
            println!("Goodbye!");
            break;
        }
        let result = calculate(input);
        println!("Result: {}", result);
    }
}
fn calculate(input: &str) -> f64 {
    let re = Regex::new(r"([\d.]+)\s*([+-/*])\s*([\d.]+)").unwrap();
    let captures = re.captures(input).unwrap();
    let num1: f64 = captures[1].parse().unwrap();
    let num2: f64 = captures[3].parse().unwrap();
    let op = &captures[2];
    match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => panic!("Invalid operator"),
    }
<<<<<<< HEAD
}
=======
}
>>>>>>> 0859bb9 (Initial commit)
