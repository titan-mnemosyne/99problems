#[derive(Debug)]
enum Code<T> {
  Single(T),
  Pair(i32, T)
}

fn pack<T: Copy + PartialEq>(list: &Vec<T>) -> Vec<Vec<T>> {
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

fn encode_modified<T: Copy + PartialEq>(list: &Vec<T>) -> Vec<Code<T>> {
  let packed_list = pack(list);
  packed_list.iter().map(|x| {
    if x.len() > 1 {
      Code::Pair(x.len() as i32, x[0])
    }
    else {
      Code::Single(x[0])
    }
  }).collect()
}

fn decode_modified<T: Copy + PartialEq>(list: &Vec<Code<T>>) -> Vec<T> {
  let mut out = Vec::new();
  for elt in list {
    let mut curr = 0;
    match elt {
      Code::Single(x) => out.push(*x),
      Code::Pair(n, x) =>
        while curr < *n {
          out.push(*x);
          curr +=1;
        },
    }
  }
  return out;
}

fn main() {
  println!("Q11:");
  let encoded_list_1 = encode_modified(&vec![1, 1, 1, 2, 3, 3]);
  println!("{:?}", encoded_list_1);
  let encoded_list_2 = encode_modified(&vec!['a', 'a', 'a', 'a', 'b', 'c', 'c', 'a', 'a', 'd', 'e', 'e', 'e', 'e']);
  println!("{:?}", encoded_list_2);

  println!("Q12:");
  println!("{:?}", decode_modified(&encoded_list_1));
  println!("{:?}", decode_modified(&encoded_list_2));
}