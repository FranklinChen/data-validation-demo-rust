// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

/// For demo purposes, a person just has a name.
#[derive(Debug, PartialEq)]
pub struct Person {
  name: String
}

#[derive(Debug, PartialEq)]
pub struct MyError {
  message: String
}

pub fn person_a() -> Person {
 Person { name: "A".to_string() }
}

pub fn person_b() -> Person {
 Person { name: "B".to_string() }
}

pub fn person_c() -> Person {
 Person { name: "C".to_string() }
}

pub fn person_y() -> Person {
 Person { name: "Y".to_string() }
}

pub fn person_z() -> Person {
 Person { name: "Z".to_string() }
}

impl Person {
  /// Dummy implementation only.
  fn best_friend(&self) -> Result<Person, MyError> {
    if self.name == "A" {
      Ok(person_b())
    } else {
      Err(MyError {
        message: format!("{} has no best friend", self.name)
      })
    }
  }

  /// Dummy implementation only.
  fn oldest_sister(&self) -> Result<Person, MyError> {
    if self.name == "B" {
      Ok(person_c())
    } else {
      Err(MyError {
        message: format!("{} has no oldest sister", self.name)
      })
    }
  }

  /// Dummy implementation only.
  fn youngest_child(&self) -> Result<Person, MyError> {
    if self.name == "Y" {
      Ok(person_z())
    } else {
      Err(MyError {
        message: format!("{} has no youngest child", self.name)
      })
    }
  }
}

/// Assume: best_friend(), oldest_sister(), youngest_child()
/// each returns Result<Person, MyError>
pub fn winner(person: Person) -> Result<Person, MyError> {
  let friend = try!(person.best_friend());
  let sister = try!(friend.oldest_sister());
  sister.youngest_child()
}


#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn a_has_no_winner_because_c_no_youngest_child() {
    assert_eq!(winner(person_a()),
               Err(MyError {
                 message: "C has no youngest child".to_string()
               }));
  }
}
