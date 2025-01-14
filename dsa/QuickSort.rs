fn main() {
  let mut numbers = [5, 4, 3, 2, 1];
  quick_sort(&mut numbers);
  println!("{:?}", numbers);
}

fn quick_sort(arr: &mut [i32]) {
  let len = arr.len();
  if len <= 1 {
      return;
  }
  
  let pi = partition(arr);
  quick_sort(&mut arr[0..pi]);
  quick_sort(&mut arr[pi+1..]);
}

fn partition(arr: &mut [i32]) -> usize {
  let len = arr.len();
  let pivot = arr[len - 1];
  let mut i = 0;
  
  for j in 0..len - 1 {
    if pivot >= arr[j] {
        arr.swap(i, j);
        i += 1;
    }
  }
  arr.swap(i, len - 1);
  i
}