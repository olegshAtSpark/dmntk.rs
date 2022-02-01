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

//! Implementation of FEEL time.

use super::date::FeelDate;
use super::nanos_to_string;
use super::zone::FeelZone;
use crate::temporal::errors::err_invalid_time_literal;
use crate::temporal::{feel_time_offset, feel_time_zone, is_valid_time, RE_TIME};
use crate::FeelDateTime;
use chrono::{DateTime, FixedOffset};
use dmntk_common::{DmntkError, Result};
use std::cmp::Ordering;
use std::str::FromStr;

/// FEEL time.
#[derive(Debug, Clone)]
pub struct FeelTime(pub u8, pub u8, pub u8, pub u64, pub FeelZone); //TODO make these fields private

impl std::fmt::Display for FeelTime {
  /// Converts [FeelTime] into [String].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.3 > 0 {
      write!(f, "{:02}:{:02}:{:02}.{}{}", self.0, self.1, self.2, nanos_to_string(self.3), self.4)
    } else {
      write!(f, "{:02}:{:02}:{:02}{}", self.0, self.1, self.2, self.4)
    }
  }
}

impl FromStr for FeelTime {
  type Err = DmntkError;
  /// Converts [String] into [FeelTime].
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if let Some(captures) = RE_TIME.captures(s) {
      if let Some(hour_match) = captures.name("hours") {
        if let Ok(hour) = hour_match.as_str().parse::<u8>() {
          if let Some(min_match) = captures.name("minutes") {
            if let Ok(min) = min_match.as_str().parse::<u8>() {
              if let Some(sec_match) = captures.name("seconds") {
                if let Ok(sec) = sec_match.as_str().parse::<u8>() {
                  let mut fractional = 0.0;
                  if let Some(frac_match) = captures.name("fractional") {
                    if let Ok(frac) = frac_match.as_str().parse::<f64>() {
                      fractional = frac;
                    }
                  }
                  let nanos = (fractional * 1e9).trunc() as u64;
                  if let Some(zone) = FeelZone::from_captures(&captures) {
                    let time = FeelTime(hour, min, sec, nanos, zone);
                    // even if parsing from string was successful, the time may still be invalid, so another check
                    let _: DateTime<FixedOffset> = time.clone().try_into().map_err(|_| err_invalid_time_literal(s))?;
                    return Ok(time);
                  }
                }
              }
            }
          }
        }
      }
    }
    Err(err_invalid_time_literal(s))
  }
}

impl PartialEq for FeelTime {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0 && self.1 == other.1 && self.2 == other.2 && self.3 == other.3 && self.4 == other.4
  }
}

impl PartialOrd for FeelTime {
  /// Returns the ordering of two times.
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    if self.4 == other.4 {
      let h = self.0.cmp(&other.0);
      let m = self.1.cmp(&other.1);
      let s = self.2.cmp(&other.2);
      let n = self.3.cmp(&other.3);
      match (h, m, s, n) {
        (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Equal) => Some(Ordering::Equal),
        (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Less) => Some(Ordering::Less),
        (Ordering::Equal, Ordering::Equal, Ordering::Equal, Ordering::Greater) => Some(Ordering::Greater),
        (Ordering::Equal, Ordering::Equal, Ordering::Less, _) => Some(Ordering::Less),
        (Ordering::Equal, Ordering::Equal, Ordering::Greater, _) => Some(Ordering::Greater),
        (Ordering::Equal, Ordering::Less, _, _) => Some(Ordering::Less),
        (Ordering::Equal, Ordering::Greater, _, _) => Some(Ordering::Greater),
        (Ordering::Less, _, _, _) => Some(Ordering::Less),
        (Ordering::Greater, _, _, _) => Some(Ordering::Greater),
      }
    } else {
      None
    }
  }
}

impl TryFrom<FeelTime> for DateTime<FixedOffset> {
  type Error = DmntkError;
  /// Converts [FeelTime] into [DateTime] with [FixedOffset].
  fn try_from(me: FeelTime) -> Result<Self, Self::Error> {
    let result: DateTime<FixedOffset> = FeelDateTime(FeelDate::today_local(), me).try_into()?;
    Ok(result)
  }
}

impl FeelTime {
  pub fn new_hmso_opt(hour: u8, minute: u8, second: u8, nano: u64, offset: i32) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      Some(Self(hour, minute, second, nano, FeelZone::from_offset(offset)))
    } else {
      None
    }
  }

  pub fn new_hms_opt(hour: u8, minute: u8, second: u8, nano: u64) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      Some(Self(hour, minute, second, nano, FeelZone::Local))
    } else {
      None
    }
  }

  /// Creates UTC time from specified time values.
  pub fn utc(hour: u8, minute: u8, second: u8, nanos: u64) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Utc)
  }

  /// Creates local time from specified time values.
  pub fn local(hour: u8, minute: u8, second: u8, nanos: u64) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Local)
  }

  /// Creates a time from specified time and offset values.
  pub fn offset(hour: u8, minute: u8, second: u8, nanos: u64, offset: i32) -> Self {
    Self(hour, minute, second, nanos, FeelZone::Offset(offset))
  }

  pub fn hour(&self) -> u8 {
    self.0
  }

  pub fn minute(&self) -> u8 {
    self.1
  }

  pub fn second(&self) -> u8 {
    self.2
  }

  pub fn feel_time_offset(&self) -> Option<i32> {
    feel_time_offset(&FeelDateTime(FeelDate::today_local(), self.clone()))
  }

  pub fn feel_time_zone(&self) -> Option<String> {
    feel_time_zone(&FeelDateTime(FeelDate::today_local(), self.clone()))
  }
}

#[cfg(test)]
mod tests {
  use super::super::zone::FeelZone;
  use super::*;

  fn eq_time(h: u8, m: u8, s: u8, n: u64, z: FeelZone, time: &str) {
    let expected = FeelTime(h, m, s, n, z);
    let actual = FeelTime::from_str(time).expect("should not fail");
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_time_from_str() {
    eq_time(18, 37, 9, 0, FeelZone::Local, "18:37:09");
    eq_time(16, 37, 9, 0, FeelZone::Utc, "16:37:09z");
    eq_time(16, 37, 9, 0, FeelZone::Utc, "16:37:09Z");
    eq_time(16, 37, 9, 0, FeelZone::Zone("Etc/UTC".to_string()), "16:37:09@Etc/UTC");
    eq_time(17, 37, 9, 0, FeelZone::Zone("Europe/London".to_string()), "17:37:09@Europe/London");
    eq_time(18, 37, 9, 0, FeelZone::Zone("Europe/Warsaw".to_string()), "18:37:09@Europe/Warsaw");
    eq_time(18, 37, 9, 0, FeelZone::Zone("Africa/Johannesburg".to_string()), "18:37:09@Africa/Johannesburg");
    eq_time(10, 37, 9, 0, FeelZone::Zone("America/Vancouver".to_string()), "10:37:09@America/Vancouver");
    eq_time(13, 37, 9, 0, FeelZone::Zone("America/New_York".to_string()), "13:37:09@America/New_York");
  }

  #[test]
  fn test_time_from_str_errors() {
    assert_eq!(Err(err_invalid_time_literal("24:37:09")), FeelTime::from_str("24:37:09"));
    assert_eq!(Err(err_invalid_time_literal("18:60:09")), FeelTime::from_str("18:60:09"));
    assert_eq!(Err(err_invalid_time_literal("05:12:60")), FeelTime::from_str("05:12:60"));
  }

  #[test]
  fn test_eq() {
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) == FeelTime(0, 0, 0, 0, FeelZone::Utc)));
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) != FeelTime(0, 0, 0, 0, FeelZone::Local)));
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) != FeelTime(0, 0, 0, 0, FeelZone::Offset(18_000))));
    assert!((FeelTime(0, 0, 0, 0, FeelZone::Utc) != FeelTime(0, 0, 0, 0, FeelZone::Zone("Europe/Warsaw".to_string()))));
  }
}
