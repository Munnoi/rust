use std::io::{self, Write};
fn main() {
  let mut num1 = String::new();
  let mut num2 = String::new();
  let mut operator = String::new();
  
  print!("Enter the first number: ");
  io::stdout().flush().unwrap(); 
  io::stdin().read_line(&mut num1).expect("Failed to read input!");

  print!("Enter the operator: ");
  io::stdout().flush().unwrap(); 
  io::stdin().read_line(&mut operator).expect("Failed to read input!");
  
  print!("Enter the second number: ");
  io::stdout().flush().unwrap(); 
  io::stdin().read_line(&mut num2).expect("Failed to read input!");

  let num1: f64 = match num1.trim().parse() {
    Ok(n) => n,
    Err(_) => {
      println!("Invalid number!");
      return;
    }
  };

  let num2: f64 = match num2.trim().parse() {
    Ok(n) => n,
    Err(_) => {
      println!("Invalid number!");
      return;
    }
  };

  let operator = operator.trim();

  let result = match operator {
    "+" => addition(num1, num2),
    "-" => subtraction(num1, num2),
    "*" => multiplication(num1, num2),
    "/" => division(num1, num2),
    _ => {
      println!("Invalid operator!");
      return;
    }
  };

  println!("Result: {result}");
}

fn addition(num1: f64, num2: f64) -> f64 {
  num1 + num2
}

fn subtraction(num1: f64, num2: f64) -> f64 {
  num1 - num2
}

fn multiplication(num1: f64, num2: f64) -> f64 {
  num1 * num2
}

fn division(num1: f64, num2: f64) -> f64 {
  num1 / num2
}
