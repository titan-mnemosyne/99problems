mod q9;

fn encode<T: Copy + PartialEq>(list: &Vec<T>) -> Vec<(i32, T)> {
  let packed_list = q9::pack(list);
  packed_list.iter().map(|x| (x.len() as i32, x[0])).collect()
}

fn main() {
  println!("{:?}", encode(&vec![1, 1, 1, 2, 3, 3]));
  println!("{:?}", encode(&vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e']));
}