fn main() {
  let num = 4;
  println!("{}", factorial(num));  
  countdown(5);
}

fn factorial(n: u32) -> u32 {
  if n == 0 {
    1
  } else {
    n * factorial(n - 1)
  }
}

fn countdown(seconds: i32) {
  if seconds == 0 {
    println!("Blastoff!");
  } else {
    println!("{seconds} seconds to blastoff...");
    countdown(seconds - 1);
  }
}
