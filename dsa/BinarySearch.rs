fn main() {
  let numbers: [i32; 5] = [1, 2, 3, 4, 5];
  let target = 6;
  let result = binary_search(&numbers, target);
  println!("{result}");
}

fn binary_search(arr: &[i32], target: i32) -> i32 {
  let mut mid;
  let mut low = 0;
  let mut high = arr.len() - 1;

  while low <= high {
    mid = (low + high) / 2;
    if arr[mid] == target {
      return mid as i32;
    } else if arr[mid] < target {
      low = mid + 1;
    } else {
      high = mid - 1;
    }
  }
  -1
}
