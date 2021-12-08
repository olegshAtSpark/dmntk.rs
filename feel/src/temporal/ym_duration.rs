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

//! `FEEL` years and months durations.

use crate::temporal::errors::*;
use dmntk_common::DmntkError;
use regex::Regex;
use std::convert::TryFrom;

/// Regular expression pattern for parsing years and months duration.
const REGEX_YEARS_AND_MONTHS: &str = r#"^(?P<sign>-)?P((?P<years>[0-9]+)Y)?((?P<months>[0-9]+)M)?$"#;

lazy_static! {
  static ref RE_YEARS_AND_MONTHS: Regex = Regex::new(REGEX_YEARS_AND_MONTHS).unwrap();
}

/// Years and months duration in `FEEL`.
/// Holds the number of months in the duration.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct FeelYearsAndMonthsDuration(i64);

/// Number of months in a year.
const MONTHS_IN_YEAR: i64 = 12;

impl FeelYearsAndMonthsDuration {
  /// Created a new years and months duration from given number of `years` and `months`.
  pub fn new_ym(years: i64, months: i64) -> Self {
    Self(years * MONTHS_IN_YEAR + months)
  }
  /// Created a new years and months duration from given number of `months`.
  pub fn new_m(months: i64) -> Self {
    Self(months)
  }
  /// Returns the number of years in this duration.
  pub fn years(&self) -> i64 {
    self.0 / MONTHS_IN_YEAR
  }
  /// Returns the number of months in this duration.
  pub fn months(&self) -> i64 {
    self.0 % MONTHS_IN_YEAR
  }
  /// Returns the total number of months of this duration.
  pub fn as_months(&self) -> i64 {
    self.0
  }
  /// Returns absolute value of the duration.
  pub fn abs(&self) -> Self {
    FeelYearsAndMonthsDuration(self.0.abs())
  }
}

impl std::fmt::Display for FeelYearsAndMonthsDuration {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let sign = if self.0 < 0 { "-" } else { "" };
    let mut month = self.0.abs();
    let year = month / MONTHS_IN_YEAR;
    month -= year * MONTHS_IN_YEAR;
    match (year > 0, month > 0) {
      (false, false) => write!(f, "P0M"),
      (false, true) => write!(f, "{}P{}M", sign, month),
      (true, false) => write!(f, "{}P{}Y", sign, year),
      (true, true) => write!(f, "{}P{}Y{}M", sign, year, month),
    }
  }
}

impl TryFrom<&str> for FeelYearsAndMonthsDuration {
  type Error = DmntkError;
  /// Converts a text into [FeelYearsAndMonthsDuration].
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_YEARS_AND_MONTHS.captures(value) {
      let mut is_valid = false;
      let mut total_months = 0_i64;
      if let Some(years_match) = captures.name("years") {
        if let Ok(years) = years_match.as_str().parse::<u64>() {
          total_months += (years as i64) * MONTHS_IN_YEAR;
          is_valid = true;
        }
      }
      if let Some(months_match) = captures.name("months") {
        if let Ok(months) = months_match.as_str().parse::<u64>() {
          total_months += months as i64;
          is_valid = true;
        }
      }
      if captures.name("sign").is_some() {
        total_months = -total_months;
      }
      if is_valid {
        return Ok(FeelYearsAndMonthsDuration(total_months));
      }
    }
    Err(err_invalid_years_and_months_duration_literal(value))
  }
}

#[cfg(test)]
mod tests {
  use super::{FeelYearsAndMonthsDuration, MONTHS_IN_YEAR};
  use std::convert::TryFrom;

  /// Utility function for testing years and months durations equality.
  fn equals(months: i64, text: &str) {
    let expected = FeelYearsAndMonthsDuration(months);
    let actual = FeelYearsAndMonthsDuration::try_from(text).unwrap();
    assert_eq!(expected, actual);
  }

  /// Utility function for testing invalid years and months durations.
  fn invalid(text: &str) {
    let actual = FeelYearsAndMonthsDuration::try_from(text);
    assert!(actual.is_err());
  }

  /// Utility function for testing equality of textual forms of years and months durations.
  fn equals_str(expected: &str, years: i64, months: i64) {
    let actual: String = FeelYearsAndMonthsDuration(years * MONTHS_IN_YEAR + months).to_string();
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_parsing_from_string_should_pass() {
    equals(1, "P1M");
    equals(-1, "-P1M");
    equals(12, "P1Y");
    equals(-12, "-P1Y");
    equals(15, "P1Y3M");
    equals(-15, "-P1Y3M");
    equals(999999999 * super::MONTHS_IN_YEAR, "P999999999Y");
    equals(-999999999 * super::MONTHS_IN_YEAR, "-P999999999Y");
  }

  #[test]
  fn test_parsing_from_string_should_fail() {
    invalid("P");
    invalid("-P");
    invalid("K1Y1M");
    invalid("-K1Y1M");
    invalid("P1M1Y");
    invalid("-P1M1Y");
    invalid("P10");
    invalid("-P10");
    invalid("PY");
    invalid("-PY");
    invalid("PM");
    invalid("-PM");
    invalid("P1Y3MT1D");
    invalid("-P1Y3MT1D");
  }

  #[test]
  fn test_converting_to_string_should_pass() {
    equals_str("P0M", 0, 0);
    equals_str("P1M", 0, 1);
    equals_str("-P1M", 0, -1);
    equals_str("P1Y", 1, 0);
    equals_str("-P1Y", -1, 0);
    equals_str("P2Y3M", 2, 3);
    equals_str("-P2Y3M", -2, -3);
  }

  #[test]
  fn test_eq_should_pass() {
    let a = FeelYearsAndMonthsDuration::new_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 0);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::new_ym(2, 3);
    let b = FeelYearsAndMonthsDuration::new_ym(2, 3);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::new_ym(-3, 4);
    let b = FeelYearsAndMonthsDuration::new_ym(-3, 4);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::new_m(0);
    let b = FeelYearsAndMonthsDuration::new_m(0);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::new_m(-0);
    let b = FeelYearsAndMonthsDuration::new_m(0);
    assert!((a == b));
    let a = FeelYearsAndMonthsDuration::new_m(-15);
    let b = FeelYearsAndMonthsDuration::new_m(-15);
    assert!((a == b));
  }

  #[test]
  fn test_ne_should_pass() {
    let a = FeelYearsAndMonthsDuration::new_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 1);
    assert!((a != b));
    let a = FeelYearsAndMonthsDuration::new_m(1);
    let b = FeelYearsAndMonthsDuration::new_m(0);
    assert!((a != b));
  }

  #[test]
  fn test_lt_should_pass() {
    let a = FeelYearsAndMonthsDuration::new_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 1);
    assert!(a < b);
    let a = FeelYearsAndMonthsDuration::new_m(4);
    let b = FeelYearsAndMonthsDuration::new_m(5);
    assert!(a < b);
  }

  #[test]
  fn test_le_should_pass() {
    let a = FeelYearsAndMonthsDuration::new_ym(0, 0);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 1);
    assert!(a <= b);
    let a = FeelYearsAndMonthsDuration::new_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 1);
    assert!(a <= b);
    let a = FeelYearsAndMonthsDuration::new_m(12);
    let b = FeelYearsAndMonthsDuration::new_m(18);
    assert!(a <= b);
    let a = FeelYearsAndMonthsDuration::new_m(16);
    let b = FeelYearsAndMonthsDuration::new_m(16);
    assert!(a <= b);
  }

  #[test]
  fn test_gt_should_pass() {
    let a = FeelYearsAndMonthsDuration::new_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 0);
    assert!(a > b);
    let a = FeelYearsAndMonthsDuration::new_m(5);
    let b = FeelYearsAndMonthsDuration::new_m(4);
    assert!(a > b);
  }

  #[test]
  fn test_ge_should_pass() {
    let a = FeelYearsAndMonthsDuration::new_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 0);
    assert!(a >= b);
    let a = FeelYearsAndMonthsDuration::new_ym(0, 1);
    let b = FeelYearsAndMonthsDuration::new_ym(0, 1);
    assert!(a >= b);
    let a = FeelYearsAndMonthsDuration::new_m(18);
    let b = FeelYearsAndMonthsDuration::new_m(12);
    assert!(a >= b);
    let a = FeelYearsAndMonthsDuration::new_m(16);
    let b = FeelYearsAndMonthsDuration::new_m(16);
    assert!(a >= b);
  }

  #[test]
  fn test_abs_should_pass() {
    let duration = FeelYearsAndMonthsDuration::try_from("P2Y3M").unwrap();
    assert_eq!("P2Y3M", duration.abs().to_string());
    let duration = FeelYearsAndMonthsDuration::try_from("-P2Y3M").unwrap();
    assert_eq!("P2Y3M", duration.abs().to_string());
  }

  #[test]
  fn test_properties() {
    let duration = FeelYearsAndMonthsDuration::try_from("P2Y3M").unwrap();
    assert_eq!(2, duration.years());
    assert_eq!(3, duration.months());
    assert_eq!(27, duration.as_months());
  }
}
