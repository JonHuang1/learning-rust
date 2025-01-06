mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
  pub mod hosting2 {}
}

use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting2;

mod customer {
  // Both of the following work
  // use crate::front_of_house::hosting;
  use super::hosting;

  pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
  }
}