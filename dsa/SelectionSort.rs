fn main() {
  let mut nums: [i32; 5] = [5, 4, 3, 2, 1];
  selection_sort(&mut nums);
  println!("{:?}", nums);
}

fn selection_sort(arr: &mut [i32]) {
  let mut minidx;
  for i in 0..arr.len() - 1 {
    minidx = i;
    for j in i+1..arr.len() {
      if arr[minidx] > arr[j] {
          minidx = j;
      }
      arr.swap(minidx, i);
    }
  }
}