// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

/// For demo purposes, a person just has a name.
#[derive(Debug, PartialEq)]
pub struct Person {
  name: String
}

pub fn person_a() -> Person {
 Person { name: "A".to_owned() }
}

pub fn person_b() -> Person {
 Person { name: "B".to_owned() }
}

pub fn person_c() -> Person {
 Person { name: "C".to_owned() }
}

pub fn person_y() -> Person {
 Person { name: "Y".to_owned() }
}

pub fn person_z() -> Person {
 Person { name: "Z".to_owned() }
}

impl Person {
  /// Dummy implementation only.
  fn best_friend(&self) -> Option<Person> {
    if self.name == "A" {
      Some(person_b())
    } else {
      None
    }
  }

  /// Dummy implementation only.
  fn oldest_sister(&self) -> Option<Person> {
    if self.name == "B" {
      Some(person_c())
    } else {
      None
    }
  }

  /// Dummy implementation only.
  fn youngest_child(&self) -> Option<Person> {
    if self.name == "Y" {
      Some(person_z())
    } else {
      None
    }
  }
}

/// Assume: best_friend(), oldest_sister(), youngest_child()
/// each returns Option<Person>
pub fn winner(person: Person) -> Option<Person> {
  person.best_friend()      .and_then( |friend|
     friend.oldest_sister() .and_then( |sister|
     sister.youngest_child()
  ))
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn a_has_no_winner() {
    assert_eq!(winner(person_a()), None);
  }
}
