// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

#[derive(Debug, PartialEq)]
pub struct Seats {
  num: i32 // private
}

#[derive(Debug, PartialEq)]
pub enum Error {
  BadCount(i32)
}

impl Seats {
  pub fn make(num: i32) -> Result<Seats, Error> {
    if num < 0 {
      Err(Error::BadCount(num))
    } else {
      Ok(Seats { num: num })
    }
  }
}
