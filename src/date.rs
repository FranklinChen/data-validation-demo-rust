// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

#[derive(Debug, PartialEq)]
pub struct Date {
  seconds: i32, // private
}

#[derive(Debug, PartialEq)]
pub enum Error {
  BadParse(String)
}

impl Date {
  pub fn new(seconds: i32) -> Date {
    Date {
      seconds: seconds
    }
  }

  pub fn parse(s: &str) -> Result<Date, Error> {
    match s.parse::<i32>() {
      Ok(seconds) => Ok(Date { seconds: seconds }),
      Err(_) => Err(Error::BadParse(s.to_string()))
        // Just for demo, ignore the error.
    }
  }

  pub fn is_before(&self, other: &Date) -> bool {
    self.seconds < other.seconds
  }
}
