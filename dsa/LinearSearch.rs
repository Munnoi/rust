fn main() {
  let numbers: [i32; 4] = [1, 2, 3, 4];
  let target = 5;
  let result = linear_search(&numbers, target);
  println!("{result}");
}

fn linear_search(arr: &[i32], target: i32) -> i32 {
  for i in 0..arr.len() {
    if arr[i] == target {
      return i as i32;
    }
  }
  -1
}
