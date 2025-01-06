mod front_of_house;

pub use crate::front_of_house::{hosting, serving};

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::seat_at_table();
}

pub fn eating() {
  serving::take_order();
  serving::serve_order();
  serving::take_payment();
}