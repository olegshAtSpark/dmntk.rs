/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * Apache license, Version 2.0
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! `FEEL` date.

use self::errors::*;
use crate::temporal::ym_duration::FeelYearsAndMonthsDuration;
use crate::temporal::{after, after_or_equal, before, before_or_equal, between, equal, weekday, FeelDateTime, FeelTime};
use crate::FeelNumber;
use chrono::{DateTime, Datelike, FixedOffset, Local};
use dmntk_common::DmntkError;
use regex::Regex;
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};

lazy_static! {
  static ref RE_DATE: Regex = Regex::new(format!("^{}$", super::DATE_PATTERN).as_str()).unwrap();
}

/// FEEL date.
#[derive(Debug, Clone)]
pub struct FeelDate(i32, u8, u8);

impl std::fmt::Display for FeelDate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
  }
}

impl TryFrom<&str> for FeelDate {
  type Error = DmntkError;
  /// Converts string into [FeelDate].
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_DATE.captures(value) {
      if let Some(year_match) = captures.name("year") {
        if let Ok(mut year) = year_match.as_str().parse::<i32>() {
          if captures.name("sign").is_some() {
            year = -year;
          }
          if let Some(month_match) = captures.name("month") {
            if let Ok(month) = month_match.as_str().parse::<u8>() {
              if let Some(day_match) = captures.name("day") {
                if let Ok(day) = day_match.as_str().parse::<u8>() {
                  if is_valid_date(year, month, day) {
                    return Ok(FeelDate(year, month, day));
                  }
                }
              }
            }
          }
        }
      }
    }
    Err(invalid_date_literal(value.to_string()))
  }
}

impl TryFrom<(FeelNumber, FeelNumber, FeelNumber)> for FeelDate {
  type Error = DmntkError;
  /// Converts a tuple of numbers into [FeelDate].
  fn try_from(value: (FeelNumber, FeelNumber, FeelNumber)) -> Result<Self, Self::Error> {
    let year = value.0.into();
    if value.1 > FeelNumber::zero() && value.2 > FeelNumber::zero() {
      let month = value.1.into();
      let day = value.2.into();
      if is_valid_date(year, month, day) {
        return Ok(Self(year, month, day));
      }
    }
    Err(invalid_date(value.0.into(), value.1.into(), value.2.into()))
  }
}

impl PartialEq for FeelDate {
  ///
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1 && self.2 == other.2
  }
}

impl PartialOrd for FeelDate {
  ///
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self == other {
      return Some(Ordering::Equal);
    }
    if let Some(before) = self.before(other) {
      if before {
        return Some(Ordering::Less);
      }
    }
    if let Some(after) = self.after(other) {
      if after {
        return Some(Ordering::Greater);
      }
    }
    None
  }
}

impl TryFrom<FeelDate> for DateTime<FixedOffset> {
  type Error = DmntkError;
  /// Converts the `FEEL` date into chrono::DateTime<FixedOffset>.
  fn try_from(me: FeelDate) -> Result<Self, Self::Error> {
    let result: DateTime<FixedOffset> = FeelDateTime(me, FeelTime::utc(0, 0, 0, 0)).try_into()?;
    Ok(result)
  }
}

impl FeelDate {
  ///
  pub fn new(year: i32, month: u8, day: u8) -> Self {
    Self(year, month, day)
  }
  ///
  pub fn new_opt(year: i32, month: u8, day: u8) -> Option<Self> {
    if is_valid_date(year, month, day) {
      Some(Self(year, month, day))
    } else {
      None
    }
  }
  ///
  pub fn today_local() -> Self {
    let today = Local::today();
    Self(today.year(), today.month() as u8, today.day() as u8)
  }
  ///
  pub fn equal(&self, other: &Self) -> Option<bool> {
    let midnight = FeelTime::utc(0, 0, 0, 0);
    equal(&FeelDateTime(self.clone(), midnight.clone()), &FeelDateTime(other.clone(), midnight))
  }
  ///
  pub fn before(&self, other: &Self) -> Option<bool> {
    let midnight = FeelTime::utc(0, 0, 0, 0);
    before(&FeelDateTime(self.clone(), midnight.clone()), &FeelDateTime(other.clone(), midnight))
  }
  ///
  pub fn before_or_equal(&self, other: &Self) -> Option<bool> {
    let midnight = FeelTime::utc(0, 0, 0, 0);
    before_or_equal(&FeelDateTime(self.clone(), midnight.clone()), &FeelDateTime(other.clone(), midnight))
  }
  ///
  pub fn after(&self, other: &Self) -> Option<bool> {
    let midnight = FeelTime::utc(0, 0, 0, 0);
    after(&FeelDateTime(self.clone(), midnight.clone()), &FeelDateTime(other.clone(), midnight))
  }
  ///
  pub fn after_or_equal(&self, other: &Self) -> Option<bool> {
    let midnight = FeelTime::utc(0, 0, 0, 0);
    after_or_equal(&FeelDateTime(self.clone(), midnight.clone()), &FeelDateTime(other.clone(), midnight))
  }
  ///
  pub fn between(&self, left: &Self, right: &Self, left_closed: bool, right_closed: bool) -> Option<bool> {
    let midnight = FeelTime::utc(0, 0, 0, 0);
    between(
      &FeelDateTime(self.clone(), midnight.clone()),
      &FeelDateTime(left.clone(), midnight.clone()),
      &FeelDateTime(right.clone(), midnight),
      left_closed,
      right_closed,
    )
  }
  ///
  pub fn ym_duration(&self, other: &FeelDate) -> FeelYearsAndMonthsDuration {
    let mut months;
    if self.0 < other.0 {
      months = 12 * (other.0 as i64 - self.0 as i64) + (other.1 as i64 - self.1 as i64);
      if self.2 > other.2 {
        months -= 1;
      }
      months *= -1;
    } else {
      months = 12 * (self.0 as i64 - other.0 as i64) + (self.1 as i64 - other.1 as i64);
      if other.2 > self.2 {
        months -= 1;
      }
    }
    FeelYearsAndMonthsDuration::new_m(months)
  }
  ///
  pub fn year(&self) -> i32 {
    self.0
  }
  ///
  pub fn month(&self) -> u8 {
    self.1
  }
  ///
  pub fn day(&self) -> u8 {
    self.2
  }
  ///
  pub fn weekday(&self) -> Option<u32> {
    weekday(&FeelDateTime(self.clone(), FeelTime::utc(0, 0, 0, 0)))
  }
  ///
  pub fn as_tuple(&self) -> (i32, u32, u32) {
    (self.0, self.1 as u32, self.2 as u32)
  }
}

///
pub fn is_valid_date(year: i32, month: u8, day: u8) -> bool {
  if DateTime::try_from(FeelDate(year, month, day)).is_ok() {
    return true;
  }
  if year >= -999_999_999 && year <= 999_999_999 {
    if let Some(last_day_of_month) = last_day_of_month(year, month) {
      return day <= last_day_of_month;
    }
  }
  false
}

///
pub fn is_leap_year(year: i32) -> bool {
  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

///
pub fn last_day_of_month(year: i32, month: u8) -> Option<u8> {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
    4 | 6 | 9 | 11 => Some(30),
    2 => Some(if is_leap_year(year) { 29 } else { 28 }),
    _ => None,
  }
}

/// Definitions of date errors.
pub mod errors {
  use dmntk_common::DmntkError;

  /// Date errors.
  #[derive(Debug, PartialEq)]
  enum FeelDateError {
    InvalidDateLiteral(String),
    InvalidDate(i32, u8, u8),
  }

  impl From<FeelDateError> for DmntkError {
    fn from(e: FeelDateError) -> Self {
      DmntkError::new("FeelDateError", &format!("{}", e))
    }
  }

  impl std::fmt::Display for FeelDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        FeelDateError::InvalidDateLiteral(literal) => {
          write!(f, "invalid date literal: {}", literal)
        }
        FeelDateError::InvalidDate(y, m, d) => {
          write!(f, "invalid date: {}-{}-{}", y, m, d)
        }
      }
    }
  }

  pub fn invalid_date_literal(literal: String) -> DmntkError {
    FeelDateError::InvalidDateLiteral(literal).into()
  }

  pub fn invalid_date(y: i32, m: u8, d: u8) -> DmntkError {
    FeelDateError::InvalidDate(y, m, d).into()
  }
}

#[cfg(test)]
mod tests {
  use super::{is_leap_year, is_valid_date, last_day_of_month};

  #[test]
  fn test_is_valid_date() {
    assert!(is_valid_date(999_999_999, 12, 13));
    assert!(!is_valid_date(1_000_000_000, 1, 1));
    assert!(is_valid_date(-999_999_999, 1, 1));
    assert!(!is_valid_date(-1_000_000_000, 12, 31));
    assert!(!is_valid_date(2021, 2, 29));
  }

  #[test]
  fn test_is_leap_year() {
    assert!(!is_leap_year(2500));
    assert!(is_leap_year(2400));
    assert!(!is_leap_year(2300));
    assert!(!is_leap_year(2200));
    assert!(!is_leap_year(2100));
    assert!(is_leap_year(2000));
    assert!(!is_leap_year(1900));
    assert!(!is_leap_year(1800));
  }

  #[test]
  fn test_last_day_of_month() {
    assert_eq!(31, last_day_of_month(2021, 1).unwrap());
    assert_eq!(28, last_day_of_month(2021, 2).unwrap());
    assert_eq!(31, last_day_of_month(2021, 3).unwrap());
    assert_eq!(30, last_day_of_month(2021, 4).unwrap());
    assert_eq!(31, last_day_of_month(2021, 5).unwrap());
    assert_eq!(30, last_day_of_month(2021, 6).unwrap());
    assert_eq!(31, last_day_of_month(2021, 7).unwrap());
    assert_eq!(31, last_day_of_month(2021, 8).unwrap());
    assert_eq!(30, last_day_of_month(2021, 9).unwrap());
    assert_eq!(31, last_day_of_month(2021, 10).unwrap());
    assert_eq!(30, last_day_of_month(2021, 11).unwrap());
    assert_eq!(31, last_day_of_month(2021, 12).unwrap());
    assert_eq!(None, last_day_of_month(2021, 13));
    assert_eq!(None, last_day_of_month(2021, 0));
  }

  #[test]
  fn test_last_day_of_month_leap_year() {
    assert_eq!(31, last_day_of_month(2020, 1).unwrap());
    assert_eq!(29, last_day_of_month(2020, 2).unwrap());
    assert_eq!(31, last_day_of_month(2020, 3).unwrap());
    assert_eq!(30, last_day_of_month(2020, 4).unwrap());
    assert_eq!(31, last_day_of_month(2020, 5).unwrap());
    assert_eq!(30, last_day_of_month(2020, 6).unwrap());
    assert_eq!(31, last_day_of_month(2020, 7).unwrap());
    assert_eq!(31, last_day_of_month(2020, 8).unwrap());
    assert_eq!(30, last_day_of_month(2020, 9).unwrap());
    assert_eq!(31, last_day_of_month(2020, 10).unwrap());
    assert_eq!(30, last_day_of_month(2020, 11).unwrap());
    assert_eq!(31, last_day_of_month(2020, 12).unwrap());
    assert_eq!(None, last_day_of_month(2020, 13));
    assert_eq!(None, last_day_of_month(2020, 0));
  }
}
