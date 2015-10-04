// -*- rust-indent-offset: 2 -*-
// More compact, just for slides.

use vec_validation;
use vec_validation::VecValidation;
use date;
use date::Date;
use seats;
use seats::Seats;

#[derive(Debug, PartialEq)]
pub struct BookingRequest {
  date: Date,
  seats: seats::Seats
}

#[derive(Debug, PartialEq)]
pub enum Error {
  DateError(date::Error),
  SeatsError(seats::Error),
  DateBefore(Date, Date),
  Missing(String)
}

impl BookingRequest {
  pub fn make(now: Date,
              opt_date_string: Option<String>,
              opt_seats: Option<i32>) ->
    VecValidation<BookingRequest, Error>
  {
    let date_result = opt_date_string
      .ok_or(Error::Missing("date".to_owned()))
      .and_then(|date_string|
                Date::parse(&date_string).map_err(Error::DateError)
                )
      .and_then(|date|
                timely_booking_date(date, now)
                );

    let seats_result = opt_seats
      .ok_or(Error::Missing("seats".to_owned()))
      .and_then(|num|
                Seats::make(num).map_err(Error::SeatsError)
                );

    vec_validation::combine2(vec_validation::single(date_result),
                             vec_validation::single(seats_result),
                             |date, seats|
                             BookingRequest {
                               date: date,
                               seats: seats
                             })
  }
}

fn timely_booking_date(date: Date, now: Date) -> Result<Date, Error> {
  if !date.is_before(&now) {
    Ok(date)
  } else {
    Err(Error::DateBefore(date, now))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use date::Date;
  use seats;

  #[test]
  fn date_is_early_and_seats_bad() {
    let now = Date::new(2);
    assert_eq!(BookingRequest::make(now,
                                    Some("1".to_owned()),
                                    Some(-3)),
               Err(vec![Error::DateBefore(Date::new(1),
                                          Date::new(2)),
                        Error::SeatsError(seats::Error::BadCount(-3))]));
  }

  fn all_good() {
    let now = Date::new(2);
    assert!(BookingRequest::make(now,
                                 Some("3".to_owned()),
                                 Some(5))
            .is_ok());
  }

}
