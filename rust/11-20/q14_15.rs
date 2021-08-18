fn duplicate<T: Copy>(list: &mut Vec<T>) -> &mut Vec<T> {
  let len = list.len();
  let mut curr = 0;
  while curr < len {
    list.insert(2*curr, list[2*curr]);
    curr += 1;
  }
  return list; 
}

fn nplicate<T: Copy>(list: &mut Vec<T>, n: usize) -> &mut Vec<T> {
  let len = list.len();
  let mut pos = 0;
  while pos < len {
    let mut copies = 1;
    while copies < n {
      list.insert(n*pos, list[n*pos]);
      copies += 1;
    }
    pos += 1;
  }
  return list;
}

fn main() {
  println!("{:?}", duplicate(&mut vec!['a', 'b', 'c', 'd']));
  println!("{:?}", duplicate(&mut vec![1, 2, 3, 4]));
  println!("{:?}", nplicate(&mut vec!['a', 'b', 'c', 'd'], 3));
  println!("{:?}", nplicate(&mut vec![1, 2, 3, 4], 4));
}