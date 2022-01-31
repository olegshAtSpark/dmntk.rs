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

use self::errors::*;
use crate::DmntkError;
use std::convert::TryFrom;
use uriparse::{RelativeReference, URI};

/// Optional reference to an element.
pub type OptHRef = Option<HRef>;

/// Reference to an element using `href` attribute.
#[derive(Debug, Clone)]
pub struct HRef(String);

impl<'a> From<&'a HRef> for &'a str {
  /// Converts a reference to [HRef] into reference to str.
  fn from(value: &'a HRef) -> Self {
    &value.0
  }
}

impl<'a> From<&'a HRef> for String {
  /// Converts a reference to [HRef] into string
  fn from(value: &'a HRef) -> Self {
    value.0.clone()
  }
}

impl TryFrom<&str> for HRef {
  type Error = DmntkError;
  /// Tries to convert string into [HRef].
  fn try_from(value: &str) -> Result<Self, Self::Error> {
    if let Ok(relative_reference) = RelativeReference::try_from(value) {
      let s = relative_reference.to_string();
      return Ok(Self(if s.starts_with('#') { s.strip_prefix('#').unwrap().to_string() } else { s }));
    }
    if let Ok(uri) = URI::try_from(value) {
      return Ok(Self(uri.to_string()));
    }
    Err(err_invalid_reference(value))
  }
}

mod errors {
  use crate::DmntkError;

  /// Errors reported by [HRef](crate::href::HRef).
  struct HRefError(String);

  impl From<HRefError> for DmntkError {
    /// Converts into [DmntkError].
    fn from(e: HRefError) -> Self {
      DmntkError::new("HRefError", &e.0)
    }
  }

  /// Creates an invalid reference error.
  pub fn err_invalid_reference(s: &str) -> DmntkError {
    HRefError(format!("invalid reference '{}'", s)).into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::convert::TryFrom;

  fn assert_href(expected: &str, uri: &str) {
    let href = &HRef::try_from(uri).unwrap();
    let actual: &str = href.into();
    assert_eq!(expected, actual);
  }

  #[test]
  fn test_valid_references() {
    assert_href("", "");
    assert_href("ref", "#ref");
    assert_href(":alfa", ":alfa");
    assert_href("//beta/gamma", "//beta/gamma");
    assert_href("ee412cf7-4dc9-4555-ab90-61907cb5b10e", "#ee412cf7-4dc9-4555-ab90-61907cb5b10e");
    assert_href("_82032dc2-36a7-4477-9392-9921353c4b44", "#_82032dc2-36a7-4477-9392-9921353c4b44");
    assert_href("https://dmntk.io/examples/example1#model2", "https://dmntk.io/examples/example1#model2");
  }

  #[test]
  fn test_invalid_references() {
    assert!(HRef::try_from("##").is_err());
    assert_eq!("HRefError: invalid reference '##'", HRef::try_from("##").err().unwrap().to_string());
  }

  #[test]
  fn test_href_into_str() {
    let href = &HRef::try_from("#_c03e81bf-a53d-47c5-9135-189935765fdc").unwrap();
    let actual: &str = href.into();
    assert_eq!("_c03e81bf-a53d-47c5-9135-189935765fdc", actual);
  }

  #[test]
  fn test_href_into_string() {
    let href = &HRef::try_from("#_c03e81bf-a53d-47c5-9135-189935765fdc").unwrap();
    let actual: String = href.into();
    assert_eq!("_c03e81bf-a53d-47c5-9135-189935765fdc", actual);
  }

  #[test]
  fn test_href_debug() {
    let href = &HRef::try_from("#_c03e81bf-a53d-47c5-9135-189935765fdc").unwrap();
    let actual: String = format!("{:?}", href);
    assert_eq!(r#"HRef("_c03e81bf-a53d-47c5-9135-189935765fdc")"#, actual);
  }

  #[test]
  #[allow(clippy::redundant_clone)]
  fn test_href_clone() {
    let href_src = HRef::try_from("#_c03e81bf-a53d-47c5-9135-189935765fdc").unwrap();
    let href_dst = href_src.clone();
    let actual: String = (&href_dst).into();
    assert_eq!("_c03e81bf-a53d-47c5-9135-189935765fdc", actual);
  }
}
