fn isPalindrome<T: PartialEq>(list: &[T]) -> bool {
  let mut i = 0;
  let mut j = list.len() - 1;
  while i < j {
    if list[i] != list[j] {
      return false
    }
    i += 1;
    j -= 1;
  }
  true
}

fn main() {
  println!("{}", isPalindrome(&['a', 'b', 'c']));
  println!("{}", isPalindrome(&['m', 'a', 'd', 'a', 'm', 'i', 'm', 'a', 'd', 'a', 'm']));
  println!("{}", isPalindrome(&['a']));
  println!("{}", isPalindrome(&[2, 5, 5, 2]));
  let empty: [i32; 0] = [];
  println!("{}", isPalindrome(&empty));
}