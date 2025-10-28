mod basket;
mod container;
mod stack;

use basket::Basket;
use stack::Stack;
use container::Container;

fn add_string<T: Container<String>>(cont: &mut T, str: String) {
  cont.put(str);
}

fn main() {
  let mut b1 = Basket::new(String::from("testsetset"));
  let b2 = Basket::new(1);
  let b3 = Basket::new(true);

  let mut s1 = Stack::new(vec![String::from("blah")]);
  let s2 = Stack::new(vec![1, 2, 3]);

  add_string(&mut b1, String::from("hi"));
  add_string(&mut s1, String::from("hi"));
}
