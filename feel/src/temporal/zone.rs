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

//! Implementation of FEEL timezone.

use regex::Captures;
use std::ops::{Div, Rem};

/// FEEL time zone.
#[derive(Debug, Clone, PartialEq)]
pub enum FeelZone {
  /// UTC time zone.
  Utc,
  /// Local time zone.
  Local,
  /// Time zone defined as an offset from UTC in seconds.
  Offset(i32),
  /// Time zone defined as a value from IANA database.
  Zone(String),
}

impl std::fmt::Display for FeelZone {
  /// Converts [FeelZone] into its text representation.
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
  /// Creates [FeelZone] based on offset from UTC in seconds.
  pub fn from_offset(offset: i32) -> Self {
    if offset != 0 {
      Self::Offset(offset)
    } else {
      Self::Utc
    }
  }
  /// Creates [FeelZone] from timezone captures taken from regular expression.
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
              if offset > 50_400 {
                return None;
              }
              if sign_match.as_str() == "-" {
                offset = -offset;
              }
              return Some(if offset != 0 { Self::Offset(offset) } else { Self::Utc });
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
  use crate::temporal::RE_TIME;

  macro_rules! assert_zone {
    ($zone:expr, $time:expr) => {
      assert_eq!($zone, FeelZone::from_captures(&RE_TIME.captures($time).unwrap()));
    };
  }

  #[test]
  fn test_format_offset() {
    assert_eq!("+05:00", FeelZone::from_offset(18_000).to_string());
    assert_eq!("+05:00", FeelZone::Offset(18_000).to_string());
    assert_eq!("-05:00", FeelZone::from_offset(-18_000).to_string());
    assert_eq!("-05:00", FeelZone::Offset(-18_000).to_string());
    assert_eq!("+05:00:01", FeelZone::from_offset(18_001).to_string());
    assert_eq!("+05:00:01", FeelZone::Offset(18_001).to_string());
    assert_eq!("-05:00:01", FeelZone::from_offset(-18_001).to_string());
    assert_eq!("-05:00:01", FeelZone::Offset(-18_001).to_string());
  }

  #[test]
  fn test_format_utc() {
    assert_eq!("Z", FeelZone::from_offset(0).to_string());
    assert_eq!("Z", FeelZone::Utc.to_string());
  }

  #[test]
  fn test_format_local() {
    assert_eq!("", FeelZone::Local.to_string());
  }

  #[test]
  fn test_from_captures() {
    assert_zone!(Some(FeelZone::Local), "00:00:00");
    assert_zone!(Some(FeelZone::Utc), "00:00:00Z");
    assert_zone!(Some(FeelZone::Utc), "00:00:00z");
    assert_zone!(Some(FeelZone::Utc), "00:00:00+00:00");
    assert_zone!(Some(FeelZone::Utc), "00:00:00-00:00");
    assert_zone!(Some(FeelZone::Utc), "00:00:00-00:00");
    assert_zone!(Some(FeelZone::Offset(18_000)), "00:00:00+05:00");
    assert_zone!(Some(FeelZone::Offset(18_001)), "00:00:00+05:00:01");
    assert_zone!(Some(FeelZone::Offset(18_060)), "00:00:00+05:01:00");
    assert_zone!(Some(FeelZone::Offset(-18_000)), "00:00:00-05:00");
    assert_zone!(Some(FeelZone::Offset(-18_001)), "00:00:00-05:00:01");
    assert_zone!(Some(FeelZone::Offset(-18_060)), "00:00:00-05:01:00");
    assert_zone!(Some(FeelZone::Offset(50_400)), "00:00:00+14:00");
    assert_zone!(Some(FeelZone::Offset(-50_400)), "00:00:00-14:00");
    assert_zone!(None, "00:00:00+14:01");
    assert_zone!(None, "00:00:00+14:00:01");
    assert_zone!(None, "00:00:00-14:01");
    assert_zone!(None, "00:00:00-14:00:01");
    assert_zone!(Some(FeelZone::Zone("Europe/Warsaw".to_string())), "00:00:00@Europe/Warsaw");
    assert_zone!(None, "00:00:00@abc/xyz");
  }

  #[test]
  fn test_eq() {
    assert!((FeelZone::Local == FeelZone::Local));
    assert!((FeelZone::Local != FeelZone::Utc));
    assert!((FeelZone::Local != FeelZone::Offset(18_400)));
    assert!((FeelZone::Local != FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Utc == FeelZone::Utc));
    assert!((FeelZone::Utc != FeelZone::Offset(18_400)));
    assert!((FeelZone::Utc != FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Offset(1) == FeelZone::Offset(1)));
    assert!((FeelZone::Offset(1) != FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Offset(1) != FeelZone::Offset(2)));
    assert!((FeelZone::Zone("Europe/Warsaw".to_string()) == FeelZone::Zone("Europe/Warsaw".to_string())));
    assert!((FeelZone::Zone("Europe/Warsaw".to_string()) != FeelZone::Zone("Australia/Sydney".to_string())));
  }

  #[test]
  fn test_debug() {
    assert_eq!(r#"Local"#, format!("{:?}", FeelZone::Local));
    assert_eq!(r#"Utc"#, format!("{:?}", FeelZone::Utc));
    assert_eq!(r#"Offset(18000)"#, format!("{:?}", FeelZone::Offset(18_000)));
    assert_eq!(r#"Zone("Europe/Warsaw")"#, format!("{:?}", FeelZone::Zone("Europe/Warsaw".to_string())));
  }
}
