fn main() {
  let mut numbers: [i32; 5] = [5, 4, 3, 2, 1];
  bubble_sort(&mut numbers);
  println!("{:?}", numbers);
}

fn bubble_sort(arr: &mut [i32]) {
  for i in 0..arr.len() - 1 {
    for j in 0..arr.len() - i - 1 {
      if arr[j] > arr[j + 1] {
        arr.swap(j, j + 1);
      }
    }
  }
}
