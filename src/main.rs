use std::io::{stdout, Write, stdin};

fn read(input: &mut String){
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
    println!("Welcome to the calcutor!");
    println!("----------");

    let mut num1 = String::new();
    let mut num2: String = String::new();
    let mut operator = String::new();

    print!("What is ther first number? ");
    read(&mut num1);

    print!("What is ther second number? ");
    read(&mut num2);

    print!("what opeation would you like to do [_+*/]? ");
    read(&mut operator);

    // println!("{} {} {}", num1, operator, num2);

    let num1: f32 = num1.trim().parse().unwrap();
    let num2: f32 = num2.trim().parse().unwrap();
    let operator: char = operator.trim().chars().next().unwrap();

    let operators = String::from("+-/*");

    if !operators.contains(operator){
        println!("Oops! invalid operator");
    }

    let result = match operator {
        '+' => num1 + num2,
        '-' => num1 - num1,
        '*' => num1 * num2,
        '/' => num1 / num2,
        _ => panic!("Oh no, the opeator is not supported")
    };

    println!("the result of {} {} {} is {}", num1, operator, num2, result)

}

