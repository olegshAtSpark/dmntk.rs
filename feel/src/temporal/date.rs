/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2022 Dariusz Depta Engos Software
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

//! Implementation of FEEL date.

use super::errors::{err_invalid_date, err_invalid_date_literal};
use super::ym_duration::FeelYearsAndMonthsDuration;
use super::{Day, FeelDateTime, FeelTime, Month, Year, RE_DATE};
use crate::temporal::{DayOfWeek, MonthOfYear};
use crate::{DayOfYear, FeelNumber, WeekOfYear};
use chrono::{DateTime, Datelike, FixedOffset, Local, NaiveDate, Weekday};
use dmntk_common::DmntkError;
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};
use std::str::FromStr;

/// FEEL date.
#[derive(Debug, Clone)]
pub struct FeelDate(Year, Month, Day);

impl std::fmt::Display for FeelDate {
  /// Converts [FeelDate] into [String].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:04}-{:02}-{:02}", self.0, self.1, self.2)
  }
}

impl FromStr for FeelDate {
  type Err = DmntkError;
  /// Converts [String] into [FeelDate].
  fn from_str(date: &str) -> Result<Self, Self::Err> {
    if let Some(captures) = RE_DATE.captures(date) {
      if let Some(year_match) = captures.name("year") {
        if let Ok(mut year) = year_match.as_str().parse::<Year>() {
          if captures.name("sign").is_some() {
            year = -year;
          }
          if let Some(month_match) = captures.name("month") {
            if let Ok(month) = month_match.as_str().parse::<Month>() {
              if let Some(day_match) = captures.name("day") {
                if let Ok(day) = day_match.as_str().parse::<Day>() {
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
    Err(err_invalid_date_literal(date))
  }
}

impl TryFrom<(FeelNumber, FeelNumber, FeelNumber)> for FeelDate {
  type Error = DmntkError;
  /// Converts a tuple of numbers into [FeelDate].
  fn try_from(value: (FeelNumber, FeelNumber, FeelNumber)) -> Result<Self, Self::Error> {
    let year = value.0.into();
    if value.1 > FeelNumber::zero() && value.2 > FeelNumber::zero() {
      let month: u32 = value.1.into();
      let day = value.2.into();
      if is_valid_date(year, month, day) {
        return Ok(Self(year, month, day));
      }
    }
    Err(err_invalid_date(value.0.into(), value.1.into(), value.2.into()))
  }
}

impl PartialEq for FeelDate {
  /// Returns `true` when two dated are equal.
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1 && self.2 == other.2
  }
}

impl PartialOrd for FeelDate {
  /// Returns the ordering of two dates.
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    let y = self.0.cmp(&other.0);
    let m = self.1.cmp(&other.1);
    let d = self.2.cmp(&other.2);
    match (y, m, d) {
      (Ordering::Equal, Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
      (Ordering::Equal, Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
      (Ordering::Equal, Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
      (Ordering::Equal, Ordering::Less, _) => Some(Ordering::Less),
      (Ordering::Equal, Ordering::Greater, _) => Some(Ordering::Greater),
      (Ordering::Less, _, _) => Some(Ordering::Less),
      (Ordering::Greater, _, _) => Some(Ordering::Greater),
    }
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
  pub fn new(year: Year, month: Month, day: Day) -> Self {
    Self(year, month, day)
  }
  ///
  pub fn new_opt(year: Year, month: Month, day: Day) -> Option<Self> {
    if is_valid_date(year, month, day) {
      Some(Self(year, month, day))
    } else {
      None
    }
  }
  ///
  pub fn today_local() -> Self {
    let today = Local::today();
    Self(today.year() as Year, today.month() as Month, today.day() as Day)
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
  pub fn year(&self) -> Year {
    self.0
  }
  ///
  pub fn month(&self) -> Month {
    self.1
  }
  ///
  pub fn day(&self) -> Day {
    self.2
  }
  ///
  pub fn day_of_week(&self) -> Option<DayOfWeek> {
    NaiveDate::from_ymd_opt(self.0, self.1, self.2).map(|naive_date| match naive_date.weekday() {
      Weekday::Mon => ("Monday".to_string(), 1_u8),
      Weekday::Tue => ("Tuesday".to_string(), 2_u8),
      Weekday::Wed => ("Wednesday".to_string(), 3_u8),
      Weekday::Thu => ("Thursday".to_string(), 4_u8),
      Weekday::Fri => ("Friday".to_string(), 5_u8),
      Weekday::Sat => ("Saturday".to_string(), 6_u8),
      Weekday::Sun => ("Sunday".to_string(), 7_u8),
    })
  }
  ///
  pub fn day_of_year(&self) -> Option<DayOfYear> {
    NaiveDate::from_ymd_opt(self.0, self.1, self.2).map(|naive_date| naive_date.ordinal() as u16)
  }
  ///
  pub fn week_of_year(&self) -> Option<WeekOfYear> {
    NaiveDate::from_ymd_opt(self.0, self.1, self.2).map(|naive_date| naive_date.iso_week().week() as u8)
  }
  ///
  pub fn month_of_year(&self) -> Option<MonthOfYear> {
    if let Some(naive_date) = NaiveDate::from_ymd_opt(self.0, self.1, self.2) {
      match naive_date.month() {
        1 => Some(("January".to_string(), 1_u8)),
        2 => Some(("February".to_string(), 2_u8)),
        3 => Some(("March".to_string(), 3_u8)),
        4 => Some(("April".to_string(), 4_u8)),
        5 => Some(("May".to_string(), 5_u8)),
        6 => Some(("June".to_string(), 6_u8)),
        7 => Some(("July".to_string(), 7_u8)),
        8 => Some(("August".to_string(), 8_u8)),
        9 => Some(("September".to_string(), 9_u8)),
        10 => Some(("October".to_string(), 10_u8)),
        11 => Some(("November".to_string(), 11_u8)),
        12 => Some(("December".to_string(), 12_u8)),
        _ => None,
      }
    } else {
      None
    }
  }
  ///
  pub fn as_tuple(&self) -> (Year, Month, Day) {
    (self.0, self.1, self.2)
  }
}

///
pub fn is_valid_date(year: Year, month: Month, day: Day) -> bool {
  if year >= -999_999_999 && year <= 999_999_999 {
    if let Some(last_day_of_month) = last_day_of_month(year, month) {
      return day <= last_day_of_month;
    }
  }
  false
}

///
pub fn is_leap_year(year: Year) -> bool {
  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
}

///
pub fn last_day_of_month(year: Year, month: Month) -> Option<Day> {
  match month {
    1 | 3 | 5 | 7 | 8 | 10 | 12 => Some(31),
    4 | 6 | 9 | 11 => Some(30),
    2 => Some(if is_leap_year(year) { 29 } else { 28 }),
    _ => None,
  }
}

#[cfg(test)]
mod tests {
  use super::*;

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

  #[test]
  fn test_eq() {
    assert!((FeelDate(2021, 2, 1) == FeelDate(2021, 2, 1)));
    assert!((FeelDate(2021, 2, 1) != FeelDate(2021, 2, 2)));
    assert!((FeelDate(2021, 2, 1) != FeelDate(2021, 3, 1)));
    assert!((FeelDate(2021, 2, 1) != FeelDate(2022, 2, 1)));
    assert!((FeelDate(999_999_999, 12, 31) != FeelDate(-999_999_999, 1, 1)));
  }

  #[test]
  fn test_compare() {
    assert!((FeelDate(2021, 2, 1) == FeelDate(2021, 2, 1)));
    assert!((FeelDate(2021, 2, 1) < FeelDate(2021, 2, 2)));
    assert!((FeelDate(2021, 2, 1) < FeelDate(2021, 3, 1)));
    assert!((FeelDate(2021, 2, 5) < FeelDate(2022, 2, 5)));
    assert!((FeelDate(2021, 2, 2) > FeelDate(2021, 2, 1)));
    assert!((FeelDate(2021, 3, 1) > FeelDate(2021, 2, 1)));
    assert!((FeelDate(2022, 2, 1) > FeelDate(2021, 2, 1)));
    assert!((FeelDate(2021, 2, 1) >= FeelDate(2021, 2, 1)));
    assert!((FeelDate(2021, 2, 2) >= FeelDate(2021, 2, 1)));
    assert!((FeelDate(2021, 2, 1) <= FeelDate(2021, 2, 1)));
    assert!((FeelDate(2021, 2, 1) <= FeelDate(2021, 2, 2)));
  }
}
