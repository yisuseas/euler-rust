//! You are given the following information,
//! but you may prefer to do some research for yourself.
//!  - 1 Jan 1900 was a Monday.
//!  - Thirty days has September,
//!    April, June and November.
//!    All the rest have thirty-one,
//!    Saving February alone,
//!    Which has twenty-eight, rain or shine.
//!    And on leap years, twenty-nine.
//!  - A leap year occurs on any year evenly divisible by 4,
//!    but not on a century unless it is divisible by 400.
//! How many Sundays fell on the first of the month during the
//! twentieth century (1 Jan 1901 to 31 Dec 2000)?

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WeekDay {
  Mon,
  Tue,
  Wed,
  Thu,
  Fri,
  Sat,
  Sun,
}

#[derive(Debug, Clone, Copy)]
struct Day {
  of_week: WeekDay,
  of_month: u8,
}

impl Day {
  fn is_first_sunday(&self) -> bool {
    self.of_month == 1 && self.of_week == WeekDay::Sun
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Month {
  Jan,
  Feb,
  Mar,
  Apr,
  May,
  Jun,
  Jul,
  Aug,
  Sep,
  Oct,
  Nov,
  Dec,
}

#[derive(Debug, Clone, Copy)]
struct Date {
  day: Day,
  month: Month,
  year: u16,
}

impl std::fmt::Display for Date {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{:?}, {} {:?}, {}",
      self.day.of_week, self.day.of_month, self.month, self.year
    )
  }
}

#[derive(Debug)]
struct Calendar {
  date: Date,
  is_leap_year: bool,
  days_in_month: u8,
}

impl Calendar {
  fn new() -> Calendar {
    Calendar {
      date: Date {
        day: Day {
          of_week: WeekDay::Sun,
          of_month: 31,
        },
        month: Month::Dec,
        year: 1899,
      },
      is_leap_year: false,
      days_in_month: 31,
    }
  }

  fn next_year(&mut self) {
    let new_year = self.date.year + 1;
    self.is_leap_year = new_year % 4 == 0 && !(new_year % 100 == 0);
    self.date.year = new_year;
  }

  fn next_month(&mut self) {
    self.date.month = match self.date.month {
      Month::Jan => Month::Feb,
      Month::Feb => Month::Mar,
      Month::Mar => Month::Apr,
      Month::Apr => Month::May,
      Month::May => Month::Jun,
      Month::Jun => Month::Jul,
      Month::Jul => Month::Aug,
      Month::Aug => Month::Sep,
      Month::Sep => Month::Oct,
      Month::Oct => Month::Nov,
      Month::Nov => Month::Dec,
      Month::Dec => {
        self.next_year();
        Month::Jan
      }
    };
    let months_with_30 = [Month::Apr, Month::Jun, Month::Sep, Month::Nov];
    self.days_in_month = if months_with_30.contains(&self.date.month) {
      30
    } else {
      match self.date.month {
        Month::Feb => {
          if self.is_leap_year {
            29
          } else {
            28
          }
        }
        _ => 31,
      }
    };
  }

  fn next_day(&mut self) {
    self.date.day = Day {
      of_week: match self.date.day.of_week {
        WeekDay::Mon => WeekDay::Tue,
        WeekDay::Tue => WeekDay::Wed,
        WeekDay::Wed => WeekDay::Thu,
        WeekDay::Thu => WeekDay::Fri,
        WeekDay::Fri => WeekDay::Sat,
        WeekDay::Sat => WeekDay::Sun,
        WeekDay::Sun => WeekDay::Mon,
      },
      of_month: if self.date.day.of_month < self.days_in_month {
        self.date.day.of_month + 1
      } else {
        self.next_month();
        1
      },
    }
  }
}

// Will iterate for ever
impl Iterator for Calendar {
  type Item = Date;

  fn next(&mut self) -> Option<Self::Item> {
    self.next_day();
    Some(self.date)
  }
}

fn answer() -> u64 {
  let calendar = Calendar::new();
  let mut first_sundays = 0;

  for date in calendar {
    if date.year > 2000 {
      break;
    }
    if date.day.is_first_sunday() && date.year > 1900 {
      // println!("{}", &date);
      first_sundays += 1;
    }
  }

  println!("How many Sundays fell on the first of the month during the");
  println!("twentieth century (1 Jan 1901 to 31 Dec 2000)?");

  first_sundays
}

fn main() {
  let a = answer();
  println!("\nAnswer: {}\n", &a);
}

////////////////////////////////////////////////////////////

#[cfg(test)]
mod e019_tests {
  use super::*;

  #[test]
  fn check_answer() {
    let expected = 171;
    assert_eq!(expected, answer());
  }
}
