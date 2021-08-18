fn drop_n<T>(list: &mut Vec<T>, n: i32) -> &mut Vec<T> {
  let mut m = n;
  let mut curr: usize = 0;
  while curr < list.len() {
    m -= 1;
    if m == 0 {
      list.remove(curr);
      m = n-1;
    }
    curr += 1;
  }
  return list;
}

fn main() {
  println!("{:?}", drop_n(&mut vec!['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'k'], 3));
}