pub fn pack<T: Copy + PartialEq>(list: &Vec<T>) -> Vec<Vec<T>> {
  let mut pack: Vec<T> = vec![list[0]];
  let mut out: Vec<Vec<T>> = Vec::new();
  let mut curr = 1;
  while curr < list.len() {
    if list[curr] != list[curr-1] {
      out.push(pack);
      pack = Vec::new();
    }
    pack.push(list[curr]);
    curr += 1;
  }
  if !pack.is_empty() {
    out.push(pack);
  }
  out
}

fn main() {
  println!("{:?}", pack(&vec![1, 1, 1, 2, 3, 3]));
  println!("{:?}", pack(&vec!['a', 'a', 'a', 'b', 'c', 'c', 'c', 'b', 'b']));
}