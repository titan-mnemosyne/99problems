fn nextToLast<T>(list: &[T]) -> Result<&T, i32> {
  if list.len() <= 1 {
    Err(0)
  }
  else {
    Ok(&list[list.len() - 2])
  }
}

fn main() {
  match nextToLast(&['a', 'b', 'c']) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
  match nextToLast(&[1, 2, 3]) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
  let empty: [i32; 0] = [];
  match nextToLast(&empty) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
}