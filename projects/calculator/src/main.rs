use std::io;


fn main() {
    println!("Calculator +-*/");

    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut operator = String::new();


    println!("Enter the first number...");
    io::stdin().read_line(&mut num1).expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Invalid number");


    println!("Enter the Operator...");
    io::stdin().read_line(&mut operator).expect("Failed to read input");
    let operator = operator.trim();


    println!("Enter the second number...");
    io::stdin().read_line(&mut num2).expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Invalid number");



    println!("You entered: {} {} {}", num1, operator, num2);
    // println!("Result: {}", result);


    match calculate(num1, num2, operator) {
        Some(result) => println!("Result: {}", result),
        None => println!("Error: Invalid operator or division by zero"),
    }

}

fn calculate(num1: f64, num2: f64, operator: &str) -> Option<f64> {
    match operator {
        "+" => Some(num1 + num2),
        "-" => Some(num1 - num2),
        "*" => Some(num1 * num2),
        "/" => {
            if num2 == 0.0 {
                None
            } else {
                Some(num1 / num2)
            }
        }
        _ => None,
    }
}