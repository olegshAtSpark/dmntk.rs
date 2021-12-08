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

use regex::Captures;
use std::ops::{Div, Rem};

/// FEEL time zones.
#[derive(Debug, Clone, PartialEq)]
pub enum FeelZone {
  /// UTC time zone.
  Utc,
  /// Local time zone.
  Local,
  /// Time zone defined as an offset from UTC.
  Offset(i32),
  /// Time zone defined as a value from IANA database.
  Zone(String),
}

impl std::fmt::Display for FeelZone {
  ///
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FeelZone::Utc => write!(f, "Z"),
      FeelZone::Local => write!(f, ""),
      FeelZone::Offset(offset) => {
        let hours = offset / 3_600;
        let minutes = offset.abs().rem(3_600).div(60);
        let seconds = offset.abs().rem(3_600).rem(60);
        if seconds > 0 {
          write!(f, "{:+03}:{:02}:{:02}", hours, minutes, seconds)
        } else {
          write!(f, "{:+03}:{:02}", hours, minutes)
        }
      }
      FeelZone::Zone(zone) => write!(f, "@{}", zone),
    }
  }
}

impl FeelZone {
  ///
  pub fn new(offset: i32) -> Self {
    if offset != 0 {
      Self::Offset(offset)
    } else {
      Self::Utc
    }
  }
  ///
  pub fn from_captures(captures: &Captures) -> Option<Self> {
    if captures.name("zulu").is_some() {
      return Some(FeelZone::Utc);
    }
    if let Some(sign_match) = captures.name("offSign") {
      if let Some(hours_match) = captures.name("offHours") {
        if let Ok(hours) = hours_match.as_str().parse::<i32>() {
          if let Some(minutes_match) = captures.name("offMinutes") {
            if let Ok(minutes) = minutes_match.as_str().parse::<i32>() {
              let mut offset = 3600 * hours + 60 * minutes;
              if let Some(seconds_match) = captures.name("offSeconds") {
                if let Ok(seconds) = seconds_match.as_str().parse::<i32>() {
                  offset += seconds;
                }
              }
              if sign_match.as_str() == "-" {
                offset = -offset;
              }
              if hours > 14 {
                // the hour magnitude is limited to at most 14
                return None;
              }
              return Some(FeelZone::new(offset));
            }
          }
        }
      }
    }
    if let Some(zone_match) = captures.name("zone") {
      return if zone_match.as_str().parse::<chrono_tz::Tz>().is_ok() {
        Some(FeelZone::Zone(zone_match.as_str().to_string()))
      } else {
        None
      };
    }
    Some(FeelZone::Local)
  }
}

#[cfg(test)]
mod tests {
  use crate::temporal::zone::FeelZone;

  #[test]
  fn test_format_offset() {
    assert_eq!("+05:00", FeelZone::new(18_000).to_string());
    assert_eq!("+05:00", FeelZone::Offset(18_000).to_string());
    assert_eq!("-05:00", FeelZone::new(-18_000).to_string());
    assert_eq!("-05:00", FeelZone::Offset(-18_000).to_string());
    assert_eq!("+05:00:01", FeelZone::new(18_001).to_string());
    assert_eq!("+05:00:01", FeelZone::Offset(18_001).to_string());
    assert_eq!("-05:00:01", FeelZone::new(-18_001).to_string());
    assert_eq!("-05:00:01", FeelZone::Offset(-18_001).to_string());
  }

  #[test]
  fn test_format_utc() {
    assert_eq!("Z", FeelZone::new(0).to_string());
    assert_eq!("Z", FeelZone::Utc.to_string());
  }

  #[test]
  fn test_format_local() {
    assert_eq!("", FeelZone::Local.to_string());
  }
}
