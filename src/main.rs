use std::io::{self};

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Multiply,
    Divide,
}

fn main() {
    loop {
        println!("Enter the operation (add, sub, multiply, divide) or 'exit' to quit:");
        let operation = match get_operation() {
            Some(op) => op,
            None => break,
        };

        println!("Enter the first number:");
        let input1: i64 = get_input();
        println!("Enter the second number:");
        let input2: i64 = get_input();
        
        let result = calculate(input1, input2, &operation);
        println!("The {:?} result of {} and {} is {}", operation, input1, input2, result);
    }
    println!("Calculator exited.");
}

fn calculate(number1: i64, number2: i64, operation: &Operation) -> i64 {
    match operation {
        Operation::Add => number1 + number2,
        Operation::Sub => number1 - number2,
        Operation::Multiply => number1 * number2,
        Operation::Divide => {
            if number2 != 0 {
                number1 / number2
            } else {
                panic!("Cannot divide by zero")
            }
        }
    }
}

fn get_operation() -> Option<Operation> {
    let mut user_action = String::new();
    io::stdin().read_line(&mut user_action).unwrap();
    let trimmed = user_action.trim();
    
    match trimmed {
        "add" => Some(Operation::Add),
        "sub" => Some(Operation::Sub),
        "multiply" => Some(Operation::Multiply),
        "divide" => Some(Operation::Divide),
        "exit" => None,
        _ => {
            println!("Invalid operation, please try again.");
            None
        }
    }
}

fn get_input() -> i64 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i64>().expect("Invalid number, please enter a valid integer")
}