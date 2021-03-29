use std::convert::TryInto;

fn size<T>(list: &[T]) -> i32 {
  list.len().try_into().unwrap()
}

fn main() {
  println!("{}", size(&[0, 1, 2, 3, 4]));
  println!("{}", size(&['a', 'b', 'c']));
  let empty: [i32; 0] = [];
  println!("{}", size(&empty));
}