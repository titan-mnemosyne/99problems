enum NestedList<T> {
  Elem(T),
  List(Vec<NestedList<T>>),
}

fn flatten<T>(list: &NestedList<T>) -> Vec<&T> {
  let mut out = Vec::new();
  match list {
    NestedList::Elem(x) => out.insert(out.len(), x),
    NestedList::List(l) => 
      for i in l {
        out.append(&mut flatten(i))
      },
  }
  return out
}

fn main() {
  let single = NestedList::Elem(5);
  let nested = NestedList::List(vec![NestedList::Elem(1), 
                                     NestedList::List(vec![NestedList::Elem(2),
                                                           NestedList::List(vec![NestedList::Elem(3), NestedList::Elem(4)])])]);
  println!("{:?}", flatten(&single));
  println!("{:?}", flatten(&nested));
}