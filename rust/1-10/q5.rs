fn reverse<T>(list: &mut [T]) {
  let mut stop = list.len()-1;
  let mut start = 0;
  while start < stop {
    list.swap(start, stop);
    start += 1;
    stop -= 1;
  }
}

fn main() {
  let mut arr1 = [1, 2];
  reverse(&mut arr1);
  for x in &arr1 {
    print!("{}, ", x);
  }

  let mut arr2 = ['a', 'b', 'c'];
  reverse(&mut arr2);
  for x in &arr2 {
    print!("{}, ", x);
  }
}