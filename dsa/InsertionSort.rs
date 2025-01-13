fn main() {
  let mut numbers: [i32; 5] = [5, 4, 3, 2, 1];
  insertion_sort(&mut numbers);
  println!("{:?}", numbers);
}

fn insertion_sort(arr: &mut [i32]) {
  let mut key;
  let mut j;

  for i in 1..arr.len() {
    key = arr[i];
    j = i;

    while j > 0 && arr[j - 1] > key {
      arr[j] = arr[j - 1];
      j -= 1;
    }
    arr[j] = key;
  } 
}
