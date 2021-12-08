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

//! Date and time utilities.

use crate::temporal::date::{is_valid_date, FeelDate};
use crate::temporal::errors::*;
use crate::temporal::ym_duration::FeelYearsAndMonthsDuration;
use crate::temporal::zone::FeelZone;
use chrono::{DateTime, Datelike, FixedOffset, Local, LocalResult, NaiveDate, NaiveDateTime, NaiveTime, TimeZone, Utc};
use dmntk_common::{DmntkError, Result};
use regex::Regex;
use std::cmp::Ordering;
use std::convert::{TryFrom, TryInto};
use std::ops::Sub;

pub mod date;
pub mod date_time;
pub mod dt_duration;
mod errors;
pub mod time;
pub mod ym_duration;
pub mod zone;

/// Regular expression pattern for parsing dates.
const DATE_PATTERN: &str = r#"(?P<sign>-)?(?P<year>[1-9][0-9]{3,8})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})"#;

/// Regular expression pattern for parsing time.
const TIME_PATTERN: &str = r#"(?P<hours>[0-9]{2}):(?P<minutes>[0-9]{2}):(?P<seconds>[0-9]{2})(?P<fractional>\.[0-9]+)?"#;

/// Regular expression patterns for parsing time zones.
const ZULU_PATTERN: &str = r#"(?P<zulu>[zZ])"#;
const ZONE_PATTERN: &str = r#"@(?P<zone>[a-zA-Z_/]+)"#;
const OFFSET_PATTERN: &str = r#"(?P<offSign>[+-])(?P<offHours>[0-9]{2}):(?P<offMinutes>[0-9]{2})(:(?P<offSeconds>[0-9]{2}))?"#;

/// Number of nanoseconds in a second.
const NANOS_IN_SECOND: u64 = 1_000_000_000;

lazy_static! {
  static ref TIME_ZONE_PATTERN: String = format!("{}|{}|{}", ZULU_PATTERN, ZONE_PATTERN, OFFSET_PATTERN);
  static ref RE_DATE: Regex = Regex::new(format!("^{}$", DATE_PATTERN).as_str()).unwrap();
  static ref RE_TIME: Regex = Regex::new(format!("^{}({})?$", TIME_PATTERN, TIME_ZONE_PATTERN.as_str()).as_str()).unwrap();
  static ref RE_DATE_AND_TIME: Regex = Regex::new(format!("^{}T{}({})?$", DATE_PATTERN, TIME_PATTERN, TIME_ZONE_PATTERN.as_str()).as_str()).unwrap();
}

/// FEEL time.
/// Stored as hour, minute, second, nanosecond and zone.
#[derive(Debug, Clone)]
pub struct FeelTime(u8, u8, u8, u64, FeelZone);

impl std::fmt::Display for FeelTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    if self.3 > 0 {
      write!(f, "{:02}:{:02}:{:02}.{}{}", self.0, self.1, self.2, nanoseconds_to_string(self.3), self.4)
    } else {
      write!(f, "{:02}:{:02}:{:02}{}", self.0, self.1, self.2, self.4)
    }
  }
}

impl std::str::FromStr for FeelTime {
  type Err = DmntkError;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    // parse the time from the provided string
    let time = parse_time_literal(s)?;
    // even if parsing from string was successful, the time may still be invalid,
    // so check the time validity by converting to chrono::DateTime<FixedOffset>
    let _: DateTime<FixedOffset> = time.clone().try_into()?;
    // time is valid
    Ok(time)
  }
}

impl std::cmp::PartialEq for FeelTime {
  fn eq(&self, other: &Self) -> bool {
    if let Some(true) = self.equal(other) {
      return true;
    }
    false
  }
}

impl TryFrom<FeelTime> for DateTime<FixedOffset> {
  type Error = DmntkError;
  /// Tries to convert the `FEEL` time into chrono::DateTime<FixedOffset>.
  /// If the conversion fails, the `FEEL` time is invalid.
  fn try_from(me: FeelTime) -> Result<Self, Self::Error> {
    let result: DateTime<FixedOffset> = FeelDateTime(FeelDate::today_local(), me).try_into()?;
    Ok(result)
  }
}

impl FeelTime {
  pub fn new_hmso_opt(hour: u8, minute: u8, second: u8, nano: u64, offset: i32) -> Option<Self> {
    if is_valid_time(hour, minute, second) {
      Some(Self(hour, minute, second, nano, FeelZone::new(offset)))
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

  /// Compares this time value with other time value and returns [Some] ([true])
  /// when both are equal. Otherwise returns [Some] ([false]).
  /// If any of compared values is not valid then [None] is returned.
  /// Times are compared using current date in local time zone.     
  pub fn equal(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    equal(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn before(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    before(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn before_or_equal(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    before_or_equal(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn after(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    after(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn after_or_equal(&self, other: &Self) -> Option<bool> {
    let today = FeelDate::today_local();
    after_or_equal(&FeelDateTime(today.clone(), self.clone()), &FeelDateTime(today, other.clone()))
  }

  pub fn between(&self, left: &Self, right: &Self, left_closed: bool, right_closed: bool) -> Option<bool> {
    let today = FeelDate::today_local();
    between(
      &FeelDateTime(today.clone(), self.clone()),
      &FeelDateTime(today.clone(), left.clone()),
      &FeelDateTime(today, right.clone()),
      left_closed,
      right_closed,
    )
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

/// FEEL date and time.
#[derive(Debug, Clone)]
pub struct FeelDateTime(FeelDate, FeelTime);

/// Implements `Display` trait for date and time.
impl std::fmt::Display for FeelDateTime {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}T{}", self.0, self.1)
  }
}

impl TryFrom<&str> for FeelDateTime {
  type Error = DmntkError;
  /// Converts string into [FeelDateTime].
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Some(captures) = RE_DATE_AND_TIME.captures(value) {
      if let Some(year_match) = captures.name("year") {
        if let Ok(mut year) = year_match.as_str().parse::<i32>() {
          if captures.name("sign").is_some() {
            year = -year;
          }
          if let Some(month_match) = captures.name("month") {
            if let Ok(month) = month_match.as_str().parse::<u8>() {
              if let Some(day_match) = captures.name("day") {
                if let Ok(day) = day_match.as_str().parse::<u8>() {
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
                              if is_valid_date(year, month, day) {
                                let date = FeelDate::new(year, month, day);
                                if let Some(zone) = FeelZone::from_captures(&captures) {
                                  if is_valid_time(hour, min, sec) {
                                    let time = FeelTime(hour, min, sec, nanos, zone);
                                    return Ok(FeelDateTime(date, time));
                                  }
                                }
                              }
                            }
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
    Err(err_invalid_date_time_literal(value))
  }
}

/// Implements `PartialEq` trait for parsing date and time.
impl std::cmp::PartialEq for FeelDateTime {
  fn eq(&self, other: &Self) -> bool {
    if let Some(true) = self.equal(other) {
      return true;
    }
    false
  }
}

impl std::ops::Sub<&FeelDateTime> for &FeelDateTime {
  type Output = Option<i64>;
  ///
  fn sub(self, rhs: &FeelDateTime) -> Option<i64> {
    subtract(self, rhs)
  }
}

impl TryFrom<FeelDateTime> for DateTime<FixedOffset> {
  type Error = DmntkError;
  ///
  fn try_from(value: FeelDateTime) -> Result<Self, Self::Error> {
    let me_date_tuple = value.0.as_tuple();
    let me_time_tuple = ((value.1).0 as u32, (value.1).1 as u32, (value.1).2 as u32, (value.1).3 as u32);
    let me_offset_opt = match &(value.1).4 {
      FeelZone::Utc => Some(0),
      FeelZone::Local => get_local_offset(me_date_tuple, me_time_tuple),
      FeelZone::Offset(offset) => Some(*offset),
      FeelZone::Zone(zone_name) => get_zone_offset(zone_name, me_date_tuple, me_time_tuple),
    };
    if let Some(me_offset) = me_offset_opt {
      if let Some(me_date) = date_time_offset(me_date_tuple, me_time_tuple, me_offset) {
        return Ok(me_date);
      }
    }
    Err(crate::temporal::date_time::errors::invalid_date_time_literal("TDB".to_string()))
  }
}

impl FeelDateTime {
  /// Creates date and time from provided [FeelDate] and [FeelTime] values.  
  pub fn new(date: FeelDate, time: FeelTime) -> Self {
    Self(date, time)
  }

  /// Creates UTC date and time from specified date and time values.
  pub fn utc(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8, nanosecond: u64) -> Self {
    Self(FeelDate::new(year, month, day), FeelTime::utc(hour, minute, second, nanosecond))
  }

  /// Creates local date and time from specified date and time values.
  pub fn local(year: i32, month: u8, day: u8, hour: u8, min: u8, sec: u8, nanos: u64) -> Self {
    Self(FeelDate::new(year, month, day), FeelTime::local(hour, min, sec, nanos))
  }

  /// Creates  date and time from specified date, time and offset values.
  pub fn offset(date: (i32, u8, u8), time: (u8, u8, u8, u64), offset: i32) -> Self {
    Self(FeelDate::new(date.0, date.1, date.2), FeelTime::offset(time.0, time.1, time.2, time.3, offset))
  }

  /// Returns the `Date` part from date and time value.
  pub fn date(&self) -> FeelDate {
    self.0.clone()
  }

  /// Returns the `Time` part from date and time value.
  pub fn time(&self) -> FeelTime {
    self.1.clone()
  }

  /// Compares this date and time value with other date and time value,
  /// returns [Some] ([true]) when both are equal. Otherwise returns [Some] ([false]).
  /// If any of compared values is not valid, the comparison can not evaluated and returns [None].    
  pub fn equal(&self, other: &FeelDateTime) -> Option<bool> {
    equal(self, other)
  }

  pub fn before(&self, other: &FeelDateTime) -> Option<bool> {
    before(self, other)
  }

  pub fn before_or_equal(&self, other: &FeelDateTime) -> Option<bool> {
    before_or_equal(self, other)
  }

  pub fn after(&self, other: &FeelDateTime) -> Option<bool> {
    after(self, other)
  }

  pub fn after_or_equal(&self, other: &FeelDateTime) -> Option<bool> {
    after_or_equal(self, other)
  }

  pub fn between(&self, left: &FeelDateTime, right: &FeelDateTime, left_closed: bool, right_closed: bool) -> Option<bool> {
    between(self, left, right, left_closed, right_closed)
  }

  pub fn ym_duration(&self, other: &FeelDateTime) -> FeelYearsAndMonthsDuration {
    self.0.ym_duration(&other.0)
  }

  pub fn ym_duration_1(&self, other: &FeelDate) -> FeelYearsAndMonthsDuration {
    self.0.ym_duration(other)
  }

  pub fn year(&self) -> i32 {
    self.0.year()
  }

  pub fn month(&self) -> u8 {
    self.0.month()
  }

  pub fn day(&self) -> u8 {
    self.0.day()
  }

  pub fn weekday(&self) -> Option<u32> {
    self.0.weekday()
  }

  pub fn hour(&self) -> u8 {
    self.1 .0
  }

  pub fn minute(&self) -> u8 {
    self.1 .1
  }

  pub fn second(&self) -> u8 {
    self.1 .2
  }

  pub fn feel_time_offset(&self) -> Option<i32> {
    feel_time_offset(self)
  }

  pub fn feel_time_zone(&self) -> Option<String> {
    feel_time_zone(self)
  }
}

/// Parses time literal.
fn parse_time_literal(s: &str) -> Result<FeelTime> {
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
                  if is_valid_time(hour, min, sec) {
                    return Ok(FeelTime(hour, min, sec, nanos, zone));
                  }
                }
              }
            }
          }
        }
      }
    }
  }
  Err(crate::temporal::time::errors::invalid_time_literal(s.to_string()))
}

fn equal(v1: &FeelDateTime, v2: &FeelDateTime) -> Option<bool> {
  if let Some(ordering) = compare(v1, v2) {
    return Some(ordering == Ordering::Equal);
  }
  None
}

fn before(v1: &FeelDateTime, v2: &FeelDateTime) -> Option<bool> {
  if let Some(ordering) = compare(v1, v2) {
    return Some(ordering == Ordering::Less);
  }
  None
}

fn before_or_equal(v1: &FeelDateTime, v2: &FeelDateTime) -> Option<bool> {
  if let Some(ordering) = compare(v1, v2) {
    return Some(ordering == Ordering::Less || ordering == Ordering::Equal);
  }
  None
}

fn after(v1: &FeelDateTime, v2: &FeelDateTime) -> Option<bool> {
  if let Some(ordering) = compare(v1, v2) {
    return Some(ordering == Ordering::Greater);
  }
  None
}

fn after_or_equal(v1: &FeelDateTime, v2: &FeelDateTime) -> Option<bool> {
  if let Some(ordering) = compare(v1, v2) {
    return Some(ordering == Ordering::Greater || ordering == Ordering::Equal);
  }
  None
}

fn between(value: &FeelDateTime, left: &FeelDateTime, right: &FeelDateTime, left_closed: bool, right_closed: bool) -> Option<bool> {
  let left_ok = if left_closed { after_or_equal(value, left) } else { after(value, left) };
  let right_ok = if right_closed { before_or_equal(value, right) } else { before(value, right) };
  if let Some((left_result, right_result)) = left_ok.zip(right_ok) {
    return Some(left_result && right_result);
  }
  None
}

fn compare(me: &FeelDateTime, other: &FeelDateTime) -> Option<Ordering> {
  let me_date_tuple = me.0.as_tuple();
  let me_time_tuple = ((me.1).0 as u32, (me.1).1 as u32, (me.1).2 as u32, (me.1).3 as u32);
  let me_offset_opt = match &(me.1).4 {
    FeelZone::Utc => Some(0),
    FeelZone::Local => get_local_offset(me_date_tuple, me_time_tuple),
    FeelZone::Offset(offset) => Some(*offset),
    FeelZone::Zone(zone_name) => get_zone_offset(zone_name, me_date_tuple, me_time_tuple),
  };
  let other_date_tuple = other.0.as_tuple();
  let other_time_tuple = ((other.1).0 as u32, (other.1).1 as u32, (other.1).2 as u32, (other.1).3 as u32);
  let other_offset_opt = match &(other.1).4 {
    FeelZone::Utc => Some(0),
    FeelZone::Local => get_local_offset(other_date_tuple, other_time_tuple),
    FeelZone::Offset(offset) => Some(*offset),
    FeelZone::Zone(zone_name) => get_zone_offset(zone_name, other_date_tuple, other_time_tuple),
  };
  if let Some((me_offset, other_offset)) = me_offset_opt.zip(other_offset_opt) {
    let me_date_opt = date_time_offset(me_date_tuple, me_time_tuple, me_offset);
    let other_date_opt = date_time_offset(other_date_tuple, other_time_tuple, other_offset);
    if let Some((me_date, other_date)) = me_date_opt.zip(other_date_opt) {
      return Some(me_date.cmp(&other_date));
    }
  }
  None
}

pub fn subtract(me: &FeelDateTime, other: &FeelDateTime) -> Option<i64> {
  let me_date_tuple = me.0.as_tuple();
  let me_time_tuple = ((me.1).0 as u32, (me.1).1 as u32, (me.1).2 as u32, (me.1).3 as u32);
  let me_offset_opt = match &(me.1).4 {
    FeelZone::Utc => Some(0),
    FeelZone::Local => get_local_offset(me_date_tuple, me_time_tuple),
    FeelZone::Offset(offset) => Some(*offset),
    FeelZone::Zone(zone_name) => get_zone_offset(zone_name, me_date_tuple, me_time_tuple),
  };
  let other_date_tuple = other.0.as_tuple();
  let other_time_tuple = ((other.1).0 as u32, (other.1).1 as u32, (other.1).2 as u32, (other.1).3 as u32);
  let other_offset_opt = match &(other.1).4 {
    FeelZone::Utc => Some(0),
    FeelZone::Local => get_local_offset(other_date_tuple, other_time_tuple),
    FeelZone::Offset(offset) => Some(*offset),
    FeelZone::Zone(zone_name) => get_zone_offset(zone_name, other_date_tuple, other_time_tuple),
  };
  if let Some((me_offset, other_offset)) = me_offset_opt.zip(other_offset_opt) {
    let me_date_opt = date_time_offset(me_date_tuple, me_time_tuple, me_offset);
    let other_date_opt = date_time_offset(other_date_tuple, other_time_tuple, other_offset);
    if let Some((me_date, other_date)) = me_date_opt.zip(other_date_opt) {
      return me_date.sub(other_date).num_nanoseconds();
    }
  }
  None
}

fn weekday(me: &FeelDateTime) -> Option<u32> {
  let me_date_tuple = me.0.as_tuple();
  let me_time_tuple = ((me.1).0 as u32, (me.1).1 as u32, (me.1).2 as u32, (me.1).3 as u32);
  let me_offset_opt = match &(me.1).4 {
    FeelZone::Utc => Some(0),
    FeelZone::Local => get_local_offset(me_date_tuple, me_time_tuple),
    FeelZone::Offset(offset) => Some(*offset),
    FeelZone::Zone(zone_name) => get_zone_offset(zone_name, me_date_tuple, me_time_tuple),
  };
  if let Some(me_offset) = me_offset_opt {
    if let Some(me_date) = date_time_offset(me_date_tuple, me_time_tuple, me_offset) {
      return Some(me_date.weekday().number_from_monday());
    }
  }
  None
}

fn feel_time_offset(me: &FeelDateTime) -> Option<i32> {
  let me_date_tuple = me.0.as_tuple();
  let me_time_tuple = ((me.1).0 as u32, (me.1).1 as u32, (me.1).2 as u32, (me.1).3 as u32);
  let me_offset_opt = match &(me.1).4 {
    FeelZone::Utc => Some(0),
    FeelZone::Local => None, // in FEEL semantic domain the local offset is treated as none
    FeelZone::Offset(offset) => Some(*offset),
    FeelZone::Zone(zone_name) => get_zone_offset(zone_name, me_date_tuple, me_time_tuple),
  };
  if let Some(me_offset) = me_offset_opt {
    return Some(me_offset);
  }
  None
}

fn feel_time_zone(me: &FeelDateTime) -> Option<String> {
  if let FeelZone::Zone(zone_name) = &(me.1).4 {
    return Some(zone_name.clone());
  }
  None
}

fn date_time_offset(date: (i32, u32, u32), time: (u32, u32, u32, u32), offset: i32) -> Option<DateTime<FixedOffset>> {
  if let LocalResult::Single(date_time) = FixedOffset::east(offset)
    .ymd_opt(date.0, date.1, date.2)
    .and_hms_nano_opt(time.0, time.1, time.2, time.3)
  {
    return Some(date_time);
  }
  None
}

/// Returns time offset (in seconds) between local time zone
/// and UTC time zone at specified date and time.
fn get_local_offset(date: (i32, u32, u32), time: (u32, u32, u32, u32)) -> Option<i32> {
  if let Some(naive_date) = NaiveDate::from_ymd_opt(date.0, date.1, date.2) {
    if let Some(naive_time) = NaiveTime::from_hms_nano_opt(time.0, time.1, time.2, time.3) {
      let naive_date_time = NaiveDateTime::new(naive_date, naive_time);
      return Some(Local.offset_from_utc_datetime(&naive_date_time).local_minus_utc());
    }
  }
  None
}

/// Returns time offset (in seconds) between named time zone
/// and UTC time zone at specified date and time.
fn get_zone_offset(zone_name: &str, date: (i32, u32, u32), time: (u32, u32, u32, u32)) -> Option<i32> {
  // try to build UTC date and time from specified values
  if let LocalResult::Single(utc) = Utc.ymd_opt(date.0, date.1, date.2).and_hms_nano_opt(time.0, time.1, time.2, time.3) {
    // try parse the time zone specified as text
    if let Ok(tz) = zone_name.parse::<chrono_tz::Tz>() {
      // build date and time in parsed time zone
      let zdt = tz.ymd(date.0, date.1, date.2).and_hms_nano(time.0, time.1, time.2, time.3);
      // calculate the time offset, the result is a chrono::Duration
      let offset: chrono::Duration = utc.with_timezone(&tz) - zdt;
      // return seconds
      return Some(offset.num_seconds() as i32);
    }
  }
  None
}

/// Converts the number of nanoseconds into textual form, the trailing zeros a stripped.
fn nanoseconds_to_string(nano: u64) -> String {
  let mut nanos = String::new();
  let mut non_zero = false;
  for ch in format!("{:09}", nano % NANOS_IN_SECOND).chars().rev() {
    if ch != '0' {
      non_zero = true;
    }
    if non_zero {
      nanos.push(ch);
    }
  }
  nanos.chars().rev().collect()
}

fn is_valid_time(hour: u8, minute: u8, second: u8) -> bool {
  hour < 24 && minute < 60 && second < 60
}

#[cfg(test)]
mod tests {
  use super::{get_local_offset, get_zone_offset, FeelDate, FeelDateTime, FeelTime, FeelZone};
  use std::convert::TryFrom;

  const SECONDS_IN_HOUR: i32 = 3_600;
  const SECONDS_IN_MIN: i32 = 60;

  fn eq_date(year: i32, month: u8, day: u8, s: &str) {
    assert_eq!(Ok(FeelDate::new(year, month, day)), FeelDate::try_from(s));
  }

  fn eq_time_loc(hour: u8, min: u8, sec: u8, s: &str) {
    let expected = FeelTime(hour, min, sec, 0, FeelZone::Local);
    let actual = s.parse::<FeelTime>().expect("should not fail");
    assert_eq!(Some(true), expected.equal(&actual));
  }

  fn eq_time_utc(hour: u8, min: u8, sec: u8, s: &str) {
    let expected = FeelTime(hour, min, sec, 0, FeelZone::Utc);
    let actual = s.parse::<FeelTime>().expect("should not fail");
    println!("{} {}", expected, actual);
    assert_eq!(Some(true), expected.equal(&actual));
  }

  fn eq_date_time_loc(date: (i32, u8, u8), time: (u8, u8, u8), s: &str) {
    let feel_date = FeelDate::new(date.0, date.1, date.2);
    let feel_time = FeelTime(time.0, time.1, time.2, 0, FeelZone::Local);
    let expected = FeelDateTime(feel_date, feel_time);
    let actual = FeelDateTime::try_from(s).expect("should not fail");
    assert_eq!(Some(true), expected.equal(&actual));
  }

  fn eq_date_time_utc(date: (i32, u8, u8), time: (u8, u8, u8), s: &str) {
    let feel_date = FeelDate::new(date.0, date.1, date.2);
    let feel_time = FeelTime(time.0, time.1, time.2, 0, FeelZone::Utc);
    let expected = FeelDateTime(feel_date, feel_time);
    let actual = FeelDateTime::try_from(s).expect("should not fail");
    assert_eq!(Some(true), expected.equal(&actual));
  }

  #[test]
  fn test_parse_date() {
    eq_date(2020, 9, 28, "2020-09-28");
  }

  #[test]
  fn test_parse_time() {
    eq_time_loc(18, 37, 9, "18:37:09");
    eq_time_utc(16, 37, 9, "16:37:09z");
    eq_time_utc(16, 37, 9, "16:37:09Z");
    eq_time_utc(16, 37, 9, "16:37:09@Etc/UTC");
    eq_time_utc(16, 37, 9, "18:37:09@Africa/Johannesburg");
    eq_time_utc(17, 37, 9, "17:37:09@Europe/London");

    // eq_time_utc(17, 37, 9, "10:37:09@America/Vancouver"); // summer time in Vancouver
    eq_time_utc(18, 37, 9, "10:37:09@America/Vancouver"); // winter time in Vancouver

    // eq_time_utc(17, 37, 9, "13:37:09@America/New_York"); // summer time in New York
    eq_time_utc(18, 37, 9, "13:37:09@America/New_York"); // winter time in New York

    eq_time_utc(17, 37, 9, "18:37:09@Europe/Warsaw");
  }

  #[test]
  fn test_parse_date_time() {
    eq_date_time_loc((2020, 9, 28), (16, 37, 9), "2020-09-28T16:37:09");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T16:37:09z");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T16:37:09Z");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T16:37:09@Etc/UTC");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T18:37:09@Africa/Johannesburg");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T17:37:09@Europe/London");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T09:37:09@America/Vancouver");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T12:37:09@America/New_York");
    eq_date_time_utc((2020, 9, 28), (16, 37, 9), "2020-09-28T18:37:09@Europe/Warsaw");
  }

  #[test]
  fn test_after_or_equal() {
    let v1 = "12:21:11".parse::<FeelTime>().expect("should not fail");
    let v2 = "12:21:12".parse::<FeelTime>().expect("should not fail");
    let v3 = "12:21:13".parse::<FeelTime>().expect("should not fail");
    assert_eq!(Some(false), v1.after_or_equal(&v2));
    assert_eq!(Some(true), v2.after_or_equal(&v2));
    assert_eq!(Some(true), v3.after_or_equal(&v2));
  }

  #[test]
  fn test_get_local_offset() {
    assert_eq!(Some(SECONDS_IN_HOUR), get_local_offset((2020, 10, 29), (9, 12, 3, 0)));
    assert_eq!(
      get_zone_offset("Europe/Warsaw", (2020, 6, 12), (9, 12, 3, 0)),
      get_local_offset((2020, 6, 12), (9, 12, 3, 0))
    );
  }

  #[test]
  fn test_get_zone_offset() {
    // winter time in Warsaw, offset = +01:00
    assert_eq!(Some(SECONDS_IN_HOUR), get_zone_offset("Europe/Warsaw", (2020, 10, 29), (9, 12, 3, 0)));
    // summer time in Warsaw, offset = +02:00
    assert_eq!(Some(2 * SECONDS_IN_HOUR), get_zone_offset("Europe/Warsaw", (2020, 6, 21), (11, 13, 49, 0)));
    // time in Moscow, offset = +03:00
    assert_eq!(Some(3 * SECONDS_IN_HOUR), get_zone_offset("Europe/Moscow", (2020, 10, 29), (9, 12, 3, 0)));
    // summer time in New York, offset = -04:00
    assert_eq!(Some(-4 * SECONDS_IN_HOUR), get_zone_offset("America/New_York", (2020, 6, 28), (12, 12, 3, 0)));
    // winter time in New York, offset = -05:00
    assert_eq!(Some(-5 * SECONDS_IN_HOUR), get_zone_offset("America/New_York", (2020, 11, 12), (18, 4, 33, 0)));
    // time in Kolkata, offset = +05:30
    assert_eq!(
      Some(5 * SECONDS_IN_HOUR + 30 * SECONDS_IN_MIN),
      get_zone_offset("Asia/Kolkata", (2020, 11, 12), (18, 4, 33, 0))
    );
    // no time change in Kolkata in summer, offset = +05:30
    assert_eq!(
      Some(5 * SECONDS_IN_HOUR + 30 * SECONDS_IN_MIN),
      get_zone_offset("Asia/Kolkata", (2020, 6, 8), (8, 0, 0, 0))
    );
    // time in Honolulu, offset = -10:00
    assert_eq!(Some(-10 * SECONDS_IN_HOUR), get_zone_offset("Pacific/Honolulu", (2020, 11, 12), (18, 4, 33, 0)));
    // no time change in Honolulu in summer, offset = -10:00
    assert_eq!(Some(-10 * SECONDS_IN_HOUR), get_zone_offset("Pacific/Honolulu", (2020, 6, 8), (8, 0, 0, 0)));
  }
}
