use std::io::{self, Write};

#[allow(unused_variables)]
fn main() {
  let mut input: String = String::new();

  print!("Enter a number: ");
  io::stdout().flush().expect("Failed to flush output");
  io::stdin().read_line(&mut input).expect("Failed to read line.");
  let number: i32 = match input.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Invalid input.");
        return;
      } 
  };
  println!("You entered: {}", number);
}
