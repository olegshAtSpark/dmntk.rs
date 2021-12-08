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

//! `FEEL` days and time durations.

use self::errors::*;
use dmntk_common::DmntkError;
use regex::Regex;
use std::convert::TryFrom;

/// Regular expression pattern for parsing days and time duration.
const REGEX_DAYS_AND_TIME: &str =
  r#"^(?P<sign>-)?P((?P<days>[0-9]+)D)?(T((?P<hours>[0-9]+)H)?((?P<minutes>[0-9]+)M)?((?P<seconds>[0-9]+)(?P<fractional>\.[0-9]*)?S)?)?$"#;

lazy_static! {
  static ref RE_DAYS_AND_TIME: Regex = Regex::new(REGEX_DAYS_AND_TIME).unwrap();
}

/// Days and time duration in `FEEL`.
/// Holds the number of nanoseconds in the duration.
#[derive(Debug, Default, Clone, PartialEq, PartialOrd)]
pub struct FeelDaysAndTimeDuration(i128);

/// Number of nanoseconds in a day.
const NANOSECONDS_IN_DAY: i128 = 24 * NANOSECONDS_IN_HOUR;

/// Number of nanoseconds in an hour.
const NANOSECONDS_IN_HOUR: i128 = 60 * NANOSECONDS_IN_MINUTE;

/// Number of nanoseconds in a minute.
const NANOSECONDS_IN_MINUTE: i128 = 60 * NANOSECONDS_IN_SECOND;

/// Number of nanoseconds in a second.
const NANOSECONDS_IN_SECOND: i128 = 1_000_000_000;

impl FeelDaysAndTimeDuration {
  /// Adds nanoseconds to current duration.
  pub fn nano(&mut self, nano: i64) -> &mut Self {
    self.0 += nano as i128;
    self
  }
  /// Adds seconds to current duration.
  pub fn second(&mut self, sec: i64) -> &mut Self {
    self.0 += sec as i128 * NANOSECONDS_IN_SECOND;
    self
  }
  ///
  pub fn build(&mut self) -> Self {
    Self(self.0)
  }
  /// Returns the number of days in this duration.
  pub fn get_days(&self) -> usize {
    (self.0.abs() / NANOSECONDS_IN_DAY) as usize
  }
  /// Returns the number of hours in this duration.
  pub fn get_hours(&self) -> usize {
    ((self.0.abs() % NANOSECONDS_IN_DAY) / NANOSECONDS_IN_HOUR) as usize
  }
  /// Returns the number of minutes in this duration.
  pub fn get_minutes(&self) -> usize {
    ((self.0.abs() % NANOSECONDS_IN_DAY % NANOSECONDS_IN_HOUR) / NANOSECONDS_IN_MINUTE) as usize
  }
  /// Returns the number of seconds in this duration.
  pub fn get_seconds(&self) -> usize {
    ((self.0.abs() % NANOSECONDS_IN_DAY % NANOSECONDS_IN_HOUR % NANOSECONDS_IN_MINUTE) / NANOSECONDS_IN_SECOND) as usize
  }
  /// Returns the seconds component of this duration with sign.
  pub fn as_seconds(&self) -> isize {
    (self.0 / NANOSECONDS_IN_SECOND) as isize
  }
  /// Returns absolute value of the duration.
  pub fn abs(&self) -> Self {
    Self(self.0.abs())
  }
}

impl std::ops::Add<FeelDaysAndTimeDuration> for FeelDaysAndTimeDuration {
  type Output = Self;
  /// Returns the sum of durations.
  fn add(self, rhs: FeelDaysAndTimeDuration) -> Self {
    Self(self.0 + rhs.0)
  }
}

impl std::ops::Sub<FeelDaysAndTimeDuration> for FeelDaysAndTimeDuration {
  type Output = Self;
  /// Returns the subtraction of durations.
  fn sub(self, rhs: FeelDaysAndTimeDuration) -> Self {
    Self(self.0 - rhs.0)
  }
}

impl std::ops::Neg for FeelDaysAndTimeDuration {
  type Output = Self;
  /// Returns the arithmetic negation of this duration.
  fn neg(self) -> Self {
    Self(-self.0)
  }
}

impl std::fmt::Display for FeelDaysAndTimeDuration {
  ///
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let sign = if self.0 < 0 { "-" } else { "" };
    let mut nanoseconds = self.0.abs();
    let day = nanoseconds / NANOSECONDS_IN_DAY;
    nanoseconds -= day * NANOSECONDS_IN_DAY;
    let hour = nanoseconds / NANOSECONDS_IN_HOUR;
    nanoseconds -= hour * NANOSECONDS_IN_HOUR;
    let minute = nanoseconds / NANOSECONDS_IN_MINUTE;
    nanoseconds -= minute * NANOSECONDS_IN_MINUTE;
    let seconds = nanoseconds / NANOSECONDS_IN_SECOND;
    nanoseconds -= seconds * NANOSECONDS_IN_SECOND;
    let nanoseconds_str = super::nanoseconds_to_string(nanoseconds as u64);
    match (day > 0, hour > 0, minute > 0, seconds > 0, nanoseconds > 0) {
      (false, false, false, false, false) => write!(f, "PT0S"),
      (false, false, false, true, false) => write!(f, "{}PT{}S", sign, seconds),
      (false, false, true, false, false) => write!(f, "{}PT{}M", sign, minute),
      (false, false, true, true, false) => write!(f, "{}PT{}M{}S", sign, minute, seconds),
      (false, true, false, false, false) => write!(f, "{}PT{}H", sign, hour),
      (false, true, false, true, false) => write!(f, "{}PT{}H{}S", sign, hour, seconds),
      (false, true, true, false, false) => write!(f, "{}PT{}H{}M", sign, hour, minute),
      (false, true, true, true, false) => write!(f, "{}PT{}H{}M{}S", sign, hour, minute, seconds),
      (true, false, false, false, false) => write!(f, "{}P{}D", sign, day),
      (true, false, false, true, false) => write!(f, "{}P{}DT{}S", sign, day, seconds),
      (true, false, true, false, false) => write!(f, "{}P{}DT{}M", sign, day, minute),
      (true, false, true, true, false) => write!(f, "{}P{}DT{}M{}S", sign, day, minute, seconds),
      (true, true, false, false, false) => write!(f, "{}P{}DT{}H", sign, day, hour),
      (true, true, false, true, false) => write!(f, "{}P{}DT{}H{}S", sign, day, hour, seconds),
      (true, true, true, false, false) => write!(f, "{}P{}DT{}H{}M", sign, day, hour, minute),
      (true, true, true, true, false) => write!(f, "{}P{}DT{}H{}M{}S", sign, day, hour, minute, seconds),
      (false, false, false, false, true) => write!(f, "{}PT0.{}S", sign, nanoseconds_str),
      (false, false, false, true, true) => write!(f, "{}PT{}.{}S", sign, seconds, nanoseconds_str),
      (false, false, true, false, true) => write!(f, "{}PT{}M0.{}S", sign, minute, nanoseconds_str),
      (false, false, true, true, true) => write!(f, "{}PT{}M{}.{}S", sign, minute, seconds, nanoseconds_str),
      (false, true, false, false, true) => write!(f, "{}PT{}H0.{}S", sign, hour, nanoseconds_str),
      (false, true, false, true, true) => write!(f, "{}PT{}H{}.{}S", sign, hour, seconds, nanoseconds_str),
      (false, true, true, false, true) => write!(f, "{}PT{}H{}M0.{}S", sign, hour, minute, nanoseconds_str),
      (false, true, true, true, true) => write!(f, "{}PT{}H{}M{}.{}S", sign, hour, minute, seconds, nanoseconds_str),
      (true, false, false, false, true) => write!(f, "{}P{}DT0.{}S", sign, day, nanoseconds_str),
      (true, false, false, true, true) => write!(f, "{}P{}DT{}.{}S", sign, day, seconds, nanoseconds_str),
      (true, false, true, false, true) => write!(f, "{}P{}DT{}M0.{}S", sign, day, minute, nanoseconds_str),
      (true, false, true, true, true) => write!(f, "{}P{}DT{}M{}.{}S", sign, day, minute, seconds, nanoseconds_str),
      (true, true, false, false, true) => write!(f, "{}P{}DT{}H0.{}S", sign, day, hour, nanoseconds_str),
      (true, true, false, true, true) => write!(f, "{}P{}DT{}H{}.{}S", sign, day, hour, seconds, nanoseconds_str),
      (true, true, true, false, true) => write!(f, "{}P{}DT{}H{}M0.{}S", sign, day, hour, minute, nanoseconds_str),
      (true, true, true, true, true) => write!(f, "{}P{}DT{}H{}M{}.{}S", sign, day, hour, minute, seconds, nanoseconds_str),
    }
  }
}

impl TryFrom<&str> for FeelDaysAndTimeDuration {
  type Error = DmntkError;
  /// Converts a text form of the days and time duration into [FeelDaysAndTimeDuration] struct.
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_DAYS_AND_TIME.captures(value) {
      let mut is_valid = false;
      let mut nanoseconds = 0_i128;
      if let Some(days_match) = captures.name("days") {
        if let Ok(days) = days_match.as_str().parse::<u64>() {
          nanoseconds += (days as i128) * NANOSECONDS_IN_DAY;
          is_valid = true;
        }
      }
      if let Some(hours_match) = captures.name("hours") {
        if let Ok(hours) = hours_match.as_str().parse::<u64>() {
          nanoseconds += (hours as i128) * NANOSECONDS_IN_HOUR;
          is_valid = true;
        }
      }
      if let Some(minutes_match) = captures.name("minutes") {
        if let Ok(minutes) = minutes_match.as_str().parse::<u64>() {
          nanoseconds += (minutes as i128) * NANOSECONDS_IN_MINUTE;
          is_valid = true;
        }
      }
      if let Some(seconds_match) = captures.name("seconds") {
        if let Ok(seconds) = seconds_match.as_str().parse::<u64>() {
          nanoseconds += (seconds as i128) * NANOSECONDS_IN_SECOND;
          is_valid = true;
        }
      }
      if let Some(fractional_match) = captures.name("fractional") {
        if let Ok(fractional) = fractional_match.as_str().parse::<f64>() {
          nanoseconds += (fractional * NANOSECONDS_IN_SECOND as f64).trunc() as i128;
          is_valid = true;
        }
      }
      if captures.name("sign").is_some() {
        nanoseconds = -nanoseconds;
      }
      if is_valid {
        return Ok(FeelDaysAndTimeDuration(nanoseconds));
      }
    }
    Err(invalid_date_and_time_duration_literal(value.to_string()))
  }
}

/// Definitions of date and time duration errors.
pub mod errors {
  use dmntk_common::DmntkError;

  /// Date and time duration errors.
  #[derive(Debug, PartialEq)]
  enum DtDurationError {
    InvalidDateAndTimeDurationLiteral(String),
  }

  impl From<DtDurationError> for DmntkError {
    fn from(e: DtDurationError) -> Self {
      DmntkError::new("DtDurationError", &format!("{}", e))
    }
  }

  impl std::fmt::Display for DtDurationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        DtDurationError::InvalidDateAndTimeDurationLiteral(literal) => {
          write!(f, "invalid date and time duration literal: {}", literal)
        }
      }
    }
  }

  pub fn invalid_date_and_time_duration_literal(literal: String) -> DmntkError {
    DtDurationError::InvalidDateAndTimeDurationLiteral(literal).into()
  }
}

#[cfg(test)]
mod tests {
  use super::super::nanoseconds_to_string;
  use super::FeelDaysAndTimeDuration;
  use std::cmp::Ordering;
  use std::convert::TryFrom;

  /// Utility function for testing days and time durations equality.
  fn equals(neg: bool, sec: i64, nano: i64, text: &str) {
    let expected = if neg {
      -FeelDaysAndTimeDuration::default().second(sec).nano(nano).build()
    } else {
      FeelDaysAndTimeDuration::default().second(sec).nano(nano).build()
    };
    let actual = FeelDaysAndTimeDuration::try_from(text).unwrap();
    assert_eq!(expected, actual);
  }

  /// Utility function for testing invalid days and time durations.
  fn invalid(text: &str) {
    let actual = FeelDaysAndTimeDuration::try_from(text);
    assert!(actual.is_err());
  }

  /// Utility function for testing equality of textual forms of days and time durations.
  fn equals_str(expected: &str, neg: bool, sec: i64, nano: i64) {
    let actual: String = if neg {
      (-FeelDaysAndTimeDuration::default().second(sec).nano(nano).build()).to_string()
    } else {
      FeelDaysAndTimeDuration::default().second(sec).nano(nano).build().to_string()
    };
    assert_eq!(expected, actual);
  }

  #[test]
  fn parsing_from_string_should_pass() {
    equals(false, 86_400, 0, "P1D");
    equals(true, 86_400, 0, "-P1D");
    equals(false, 97200, 0, "P1DT3H");
    equals(true, 97200, 0, "-P1DT3H");
    equals(false, 97980, 0, "P1DT3H13M");
    equals(true, 97980, 0, "-P1DT3H13M");
    equals(false, 98023, 0, "P1DT3H13M43S");
    equals(true, 98023, 0, "-P1DT3H13M43S");
    equals(false, 10800, 0, "PT3H");
    equals(true, 10800, 0, "-PT3H");
    equals(false, 12960, 0, "PT3H36M");
    equals(true, 12960, 0, "-PT3H36M");
    equals(false, 12982, 0, "PT3H36M22S");
    equals(true, 12982, 0, "-PT3H36M22S");
    equals(false, 3540, 0, "PT59M");
    equals(true, 3540, 0, "-PT59M");
    equals(false, 3558, 0, "PT59M18S");
    equals(true, 3558, 0, "-PT59M18S");
    equals(false, 31, 0, "PT31S");
    equals(true, 31, 0, "-PT31S");
    equals(false, 87180, 0, "P1DT13M");
    equals(true, 87180, 0, "-P1DT13M");
    equals(false, 97243, 0, "P1DT3H43S");
    equals(true, 97243, 0, "-P1DT3H43S");
    equals(false, 0, 999_000_000, "PT0.999S");
    equals(false, 0, 0, "PT0.S");
    equals(false, 58, 123_123_123, "PT58.123123123S");
    equals(false, 999, 999_999_999, "PT999.999999999999S");
  }

  #[test]
  fn parsing_from_string_should_fail() {
    invalid("P");
    invalid("-P");
    invalid("PT");
    invalid("-PT");
    invalid("T");
    invalid("-T");
    invalid("P11");
    invalid("-P11");
    invalid("PT1S1M");
    invalid("-PT1S1M");
    invalid("PT2M3H12S");
    invalid("-PT2M3H12S");
  }

  #[test]
  fn converting_to_string_should_pass() {
    equals_str("PT0.999S", false, 0, 999_000_000);
    equals_str("PT0S", false, 0, 0);
    equals_str("PT1S", false, 1, 0);
    equals_str("-PT1S", true, 1, 0);
    equals_str("PT1.123S", false, 1, 123_000_000);
    equals_str("-PT1.123S", true, 1, 123_000_000);
    equals_str("PT59S", false, 59, 0);
    equals_str("-PT59S", true, 59, 0);
    equals_str("PT1M", false, 60, 0);
    equals_str("-PT1M", true, 60, 0);
    equals_str("PT1M0.987987987S", false, 60, 987987987);
    equals_str("-PT1M0.987987987S", true, 60, 987987987);
    equals_str("PT1M1S", false, 61, 0);
    equals_str("-PT1M1S", true, 61, 0);
    equals_str("PT1M1.584S", false, 61, 584_000_000);
    equals_str("-PT1M1.584S", true, 61, 584_000_000);
    equals_str("PT59M59S", false, 3_599, 0);
    equals_str("-PT59M59S", true, 3_599, 0);
    equals_str("PT59M59.999999999S", false, 3_599, 999_999_999);
    equals_str("-PT59M59.999999999S", true, 3_599, 999_999_999);
    equals_str("PT1H", false, 3_600, 0);
    equals_str("-PT1H", true, 3_600, 0);
    equals_str("PT1H10S", false, 3_610, 0);
    equals_str("-PT1H10S", true, 3_610, 0);
    equals_str("PT1H0.3459S", false, 3_600, 345_900_000);
    equals_str("-PT1H0.3459S", true, 3_600, 345_900_000);
    equals_str("PT1H59.11S", false, 3_659, 110_000_000);
    equals_str("-PT1H59.11S", true, 3_659, 110_000_000);
    equals_str("PT1H1M", false, 3_660, 0);
    equals_str("-PT1H1M", true, 3_660, 0);
    equals_str("PT1H1M0.123S", false, 3_660, 123_000_000);
    equals_str("-PT1H1M0.123S", true, 3_660, 123_000_000);
    equals_str("PT1H1M1S", false, 3_661, 0);
    equals_str("-PT1H1M1S", true, 3_661, 0);
    equals_str("PT1H1M1.123S", false, 3_661, 123_000_000);
    equals_str("-PT1H1M1.123S", true, 3_661, 123_000_000);
    equals_str("PT23H59M59S", false, 86_399, 0);
    equals_str("-PT23H59M59S", true, 86_399, 0);
    equals_str("PT23H59M59.123S", false, 86_399, 123_000_000);
    equals_str("-PT23H59M59.123S", true, 86_399, 123_000_000);
    equals_str("P1D", false, 86_400, 0);
    equals_str("-P1D", true, 86_400, 0);
    equals_str("P1DT0.123S", false, 86_400, 123_000_000);
    equals_str("-P1DT0.123S", true, 86_400, 123_000_000);
    equals_str("P1DT1S", false, 86_401, 0);
    equals_str("-P1DT1S", true, 86_401, 0);
    equals_str("P1DT1.123S", false, 86_401, 123_000_000);
    equals_str("-P1DT1.123S", true, 86_401, 123_000_000);
    equals_str("P1DT59S", false, 86_459, 0);
    equals_str("-P1DT59S", true, 86_459, 0);
    equals_str("P1DT59.123S", false, 86_459, 123_000_000);
    equals_str("-P1DT59.123S", true, 86_459, 123_000_000);
    equals_str("P1DT1M", false, 86_460, 0);
    equals_str("-P1DT1M", true, 86_460, 0);
    equals_str("P1DT1M0.123S", false, 86_460, 123_000_000);
    equals_str("-P1DT1M0.123S", true, 86_460, 123_000_000);
    equals_str("P1DT1M1S", false, 86_461, 0);
    equals_str("-P1DT1M1S", true, 86_461, 0);
    equals_str("P1DT1M1.123S", false, 86_461, 123_000_000);
    equals_str("-P1DT1M1.123S", true, 86_461, 123_000_000);
    equals_str("P1DT59M59S", false, 89_999, 0);
    equals_str("-P1DT59M59S", true, 89_999, 0);
    equals_str("P1DT59M59.123S", false, 89_999, 123_000_000);
    equals_str("-P1DT59M59.123S", true, 89_999, 123_000_000);
    equals_str("P1DT1H", false, 90_000, 0);
    equals_str("-P1DT1H", true, 90_000, 0);
    equals_str("P1DT1H0.123S", false, 90_000, 123_000_000);
    equals_str("-P1DT1H0.123S", true, 90_000, 123_000_000);
    equals_str("P1DT1H59S", false, 90_059, 0);
    equals_str("-P1DT1H59S", true, 90_059, 0);
    equals_str("P1DT1H59.123S", false, 90_059, 123_000_000);
    equals_str("-P1DT1H59.123S", true, 90_059, 123_000_000);
    equals_str("P1DT1H1M", false, 90_060, 0);
    equals_str("-P1DT1H1M", true, 90_060, 0);
    equals_str("P1DT1H1M0.123S", false, 90_060, 123_000_000);
    equals_str("-P1DT1H1M0.123S", true, 90_060, 123_000_000);
    equals_str("P1DT1H1M1S", false, 90_061, 0);
    equals_str("-P1DT1H1M1S", true, 90_061, 0);
    equals_str("P1DT1H1M1.123S", false, 90_061, 123_000_000);
    equals_str("-P1DT1H1M1.123S", true, 90_061, 123_000_000);
    equals_str("P1DT23H59M59S", false, 172_799, 0);
    equals_str("-P1DT23H59M59S", true, 172_799, 0);
    equals_str("P1DT23H59M59.123S", false, 172_799, 123_000_000);
    equals_str("-P1DT23H59M59.123S", true, 172_799, 123_000_000);
  }

  #[test]
  fn converting_nanos_to_string_should_pass() {
    assert_eq!("", nanoseconds_to_string(0));
    assert_eq!("000000001", nanoseconds_to_string(1));
    assert_eq!("00000001", nanoseconds_to_string(10));
    assert_eq!("0000001", nanoseconds_to_string(100));
    assert_eq!("000001", nanoseconds_to_string(1_000));
    assert_eq!("00001", nanoseconds_to_string(10_000));
    assert_eq!("0001", nanoseconds_to_string(100_000));
    assert_eq!("001", nanoseconds_to_string(1_000_000));
    assert_eq!("01", nanoseconds_to_string(10_000_000));
    assert_eq!("1", nanoseconds_to_string(100_000_000));
    assert_eq!("", nanoseconds_to_string(1_000_000_000));
  }

  #[test]
  fn eq_should_pass() {
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::default()
        .second(0)
        .nano(0)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(0).nano(0))
    );
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::default()
        .second(0)
        .nano(10)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(0).nano(10))
    );
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::default()
        .second(0)
        .nano(999_999_999)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(0).nano(999_999_999))
    );
    assert_eq!(
      Some(Ordering::Equal),
      FeelDaysAndTimeDuration::default()
        .second(86_400)
        .nano(999_999_999)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(86_400).nano(999_999_999))
    );
    assert_eq!(
      FeelDaysAndTimeDuration::default().nano(0).build(),
      FeelDaysAndTimeDuration::default().nano(0).build()
    );
    assert_eq!(
      FeelDaysAndTimeDuration::default().nano(0).build(),
      FeelDaysAndTimeDuration::default().nano(-0).build()
    );
    assert_eq!(
      FeelDaysAndTimeDuration::default().second(0).nano(10),
      FeelDaysAndTimeDuration::default().second(0).nano(10)
    );
    assert_eq!(
      FeelDaysAndTimeDuration::default().second(0).nano(999_999_999),
      FeelDaysAndTimeDuration::default().second(0).nano(999_999_999)
    );
    assert_eq!(
      FeelDaysAndTimeDuration::default().second(86_400).nano(999_999_999),
      FeelDaysAndTimeDuration::default().second(86_400).nano(999_999_999)
    );
  }

  #[test]
  fn lt_should_pass() {
    assert_eq!(
      Some(Ordering::Less),
      FeelDaysAndTimeDuration::default()
        .second(10)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(11))
    );
    assert_eq!(
      Some(Ordering::Less),
      FeelDaysAndTimeDuration::default()
        .second(10)
        .nano(1)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(10).nano(2))
    );
    assert!(FeelDaysAndTimeDuration::default().second(10) < FeelDaysAndTimeDuration::default().second(11));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(1) < FeelDaysAndTimeDuration::default().second(10).nano(2));
    assert!(FeelDaysAndTimeDuration::default().second(11) >= FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(2) >= FeelDaysAndTimeDuration::default().second(10).nano(1));
  }

  #[test]
  fn le_should_pass() {
    assert!(FeelDaysAndTimeDuration::default().second(10) <= FeelDaysAndTimeDuration::default().second(11));
    assert!(FeelDaysAndTimeDuration::default().second(10) <= FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10) <= FeelDaysAndTimeDuration::default().second(10).nano(1));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(1) <= FeelDaysAndTimeDuration::default().second(10).nano(1));
    assert!(FeelDaysAndTimeDuration::default().second(11) > FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(2) > FeelDaysAndTimeDuration::default().second(10).nano(1));
  }

  #[test]
  fn gt_should_pass() {
    assert_eq!(
      Some(Ordering::Greater),
      FeelDaysAndTimeDuration::default()
        .second(11)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(10))
    );
    assert_eq!(
      Some(Ordering::Greater),
      FeelDaysAndTimeDuration::default()
        .second(10)
        .nano(1)
        .partial_cmp(&FeelDaysAndTimeDuration::default().second(10))
    );
    assert!(FeelDaysAndTimeDuration::default().second(11) > FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(1) > FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10) <= FeelDaysAndTimeDuration::default().second(11));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(1) <= FeelDaysAndTimeDuration::default().second(10).nano(2));
  }

  #[test]
  fn ge_should_pass() {
    assert!(FeelDaysAndTimeDuration::default().second(11) >= FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10) >= FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(1) >= FeelDaysAndTimeDuration::default().second(10));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(1) >= FeelDaysAndTimeDuration::default().second(10).nano(1));
    assert!(FeelDaysAndTimeDuration::default().second(10) < FeelDaysAndTimeDuration::default().second(11));
    assert!(FeelDaysAndTimeDuration::default().second(10).nano(1) < FeelDaysAndTimeDuration::default().second(10).nano(2));
  }

  #[test]
  fn add_should_pass() {
    let a = FeelDaysAndTimeDuration::default().second(11).build();
    let b = FeelDaysAndTimeDuration::default().second(83).build();
    let c = FeelDaysAndTimeDuration::default().second(94).build();
    assert_eq!(c, a + b);
    let a = FeelDaysAndTimeDuration::default().second(11).nano(2_837).build();
    let b = FeelDaysAndTimeDuration::default().second(83).nano(23).build();
    let c = FeelDaysAndTimeDuration::default().second(94).nano(2_860).build();
    assert_eq!(c, a + b);
    let a = FeelDaysAndTimeDuration::default().second(1).nano(999_999_999).build();
    let b = FeelDaysAndTimeDuration::default().second(1).nano(2).build();
    let c = FeelDaysAndTimeDuration::default().second(3).nano(1).build();
    assert_eq!(c, a + b);
  }

  #[test]
  fn sub_should_pass() {
    let a = FeelDaysAndTimeDuration::default().second(12).build();
    let b = FeelDaysAndTimeDuration::default().second(2).build();
    let c = FeelDaysAndTimeDuration::default().second(10).build();
    assert_eq!(c, a - b);
    let a = FeelDaysAndTimeDuration::default().second(99).nano(999_999_999).build();
    let b = FeelDaysAndTimeDuration::default().second(77).nano(888_888_888).build();
    let c = FeelDaysAndTimeDuration::default().second(22).nano(111_111_111).build();
    assert_eq!(c, a - b);
    let a = FeelDaysAndTimeDuration::default().second(1).nano(1).build();
    let b = FeelDaysAndTimeDuration::default().nano(2).build();
    let c = FeelDaysAndTimeDuration::default().nano(999_999_999).build();
    assert_eq!(c, a - b);
  }

  #[test]
  fn duration_as_seconds() {
    let duration = FeelDaysAndTimeDuration::try_from("PT12S").unwrap();
    assert_eq!(12, duration.as_seconds());
    let duration = FeelDaysAndTimeDuration::try_from("-PT12S").unwrap();
    assert_eq!(-12, duration.as_seconds());
    let duration = FeelDaysAndTimeDuration::try_from("PT2H").unwrap();
    assert_eq!(7_200, duration.as_seconds());
    let duration = FeelDaysAndTimeDuration::try_from("-PT2H").unwrap();
    assert_eq!(-7_200, duration.as_seconds());
  }

  #[test]
  fn duration_properties() {
    let duration = FeelDaysAndTimeDuration::try_from("P3DT5H18M36S").unwrap();
    assert_eq!(3, duration.get_days());
    assert_eq!(5, duration.get_hours());
    assert_eq!(18, duration.get_minutes());
    assert_eq!(36, duration.get_seconds());
    assert_eq!(278_316, duration.as_seconds());
    let duration = FeelDaysAndTimeDuration::try_from("-P3DT5H18M36S").unwrap();
    assert_eq!(3, duration.get_days());
    assert_eq!(5, duration.get_hours());
    assert_eq!(18, duration.get_minutes());
    assert_eq!(36, duration.get_seconds());
    assert_eq!(-278_316, duration.as_seconds());
  }

  #[test]
  fn test_abs_should_pass() {
    let duration = FeelDaysAndTimeDuration::try_from("PT12S").unwrap();
    assert_eq!("PT12S", duration.abs().to_string());
    let duration = FeelDaysAndTimeDuration::try_from("-PT12S").unwrap();
    assert_eq!("PT12S", duration.abs().to_string());
    let duration = FeelDaysAndTimeDuration::try_from("P3DT5H18M36S").unwrap();
    assert_eq!("P3DT5H18M36S", duration.abs().to_string());
    let duration = FeelDaysAndTimeDuration::try_from("-P3DT5H18M36S").unwrap();
    assert_eq!("P3DT5H18M36S", duration.abs().to_string());
  }
}
