mod basket;

use basket::Basket;

fn main() {
  let b1 = Basket::new(String::from("testsetset"));
  let b2 = Basket::new(1);
  let b3 = Basket::new(true);

}
