fn last<T>(list: &[T]) -> Result<&T, i32> {
  if list.len() == 0 {
    Err(0)
  }
  else {
    Ok(list.last().unwrap())
  }
}

fn main() {
  match last(&['a', 'b', 'c']) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
  match last(&[1, 2, 3]) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
  let empty: [i32; 0] = [];
  match last(&empty) {
    Ok(l) => println!("{}", l),
    Err(e) => println!("{}", e),
  };
}