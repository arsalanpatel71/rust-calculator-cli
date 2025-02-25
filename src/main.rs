use std::io::{self};

#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Multiply,
    Divide,
}

fn main() {
    println!("enter the operation (add,sub,multiply,divide)");
    let operation = get_operation();
    println!("Enter the first number");
    let input1:i64=get_input();
    println!("Enter the second number");

  let input2:i64=get_input();
    let result = calculate(input1, input2, &operation);
    println!("the {:?} result of {} and {} is {}", operation, input1, input2, result)
}

fn calculate(number1: i64, number2: i64, operation: &Operation) -> i64 {
    match operation {
        Operation::Add => number1 + number2,
        Operation::Sub => number1 - number2,
        Operation::Multiply => number1 * number2,
        Operation::Divide => if number2 != 0 {
            number1 / number2
        } else {
            panic!("cannot divide by zero")
        }
    }
}

fn get_operation()->Operation {
    let mut user_action=String::new();
    io::stdin().read_line(&mut user_action).unwrap();
  
    match user_action.trim() {
        "add"=>Operation::Add,
        "sub"=> Operation::Sub,
        "multiply"=>Operation::Multiply,
        "divide"=>Operation::Divide,
        _=>panic!("invalid operation")
    }
}

fn get_input()->i64{
    let mut input=String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().parse::<i64>().unwrap()
}
