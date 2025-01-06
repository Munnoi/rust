fn main() {
  let mut numbers: [i32; 6] = [4, 8, 15, 16, 17, 18];
  let strings = ["Hello", "Hola", "Konnichiwa", "Halo"];

  numbers[1] = 5;

  for i in numbers {
    println!("{}", i);
  }

  for word in strings {
    print!("{} ", word);
  }
  print!("\n");
}
