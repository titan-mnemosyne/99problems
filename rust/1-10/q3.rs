fn k<T>(list: &[T], index: usize) -> Result<&T, i32> {
  if index >= list.len() {
    Err(0)
  }
  else {
    Ok(&list[index])
  }
}

fn main() {
  match k(&['a', 'b', 'c'], 1) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
  match k(&[1, 2, 3], 0) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
  let empty: [i32; 0] = [];
  match k(&empty, 1) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
}