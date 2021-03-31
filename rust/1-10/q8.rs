fn dedupList<T: Clone + PartialEq>(list: &mut [T]) -> &[T] {
  if list.len() <= 1 {
    return list
  }
  let mut replace = 1;
  let mut total = 1;
  let mut curr = 1;
  while curr < list.len() {
    if list[curr] != list[curr-1] {
      list[replace] = list[curr].clone();
      total += 1;
      replace += 1;
    }
    curr += 1;
  }
  &list[0..total]
}

fn main() {
  println!("{:?}", dedupList(&mut [1, 1, 2, 3, 3, 2]));
  println!("{:?}", dedupList(&mut [1, 2, 3]));
  println!("{:?}", dedupList(&mut [1, 1, 1, 1, 1, 1, 1]));

}