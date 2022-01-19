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

//! Definition of the common error type.

/// Common result type.
pub type Result<T, E = DmntkError> = std::result::Result<T, E>;

/// Common error definition used by all `DMNTK` components.
#[derive(Debug, PartialEq)]
pub struct DmntkError(String);

impl std::fmt::Display for DmntkError {
  /// Implementation of [Display](std::fmt::Display) trait for [DmntkError].
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

impl DmntkError {
  /// Creates a new [DmntkError] with specified `source` and `message`.
  pub fn new(source: &str, message: &str) -> Self {
    Self(format!("{}: {}", source, message))
  }
}

#[cfg(test)]
mod tests {
  use crate::errors::DmntkError;

  #[test]
  fn test_new() {
    assert_eq!(
      "ParserError: unexpected end of file",
      format!("{}", DmntkError::new("ParserError", "unexpected end of file"))
    );
  }

  #[test]
  fn test_debug() {
    assert_eq!(
      r#"DmntkError("ParserError: unexpected end of file")"#,
      format!("{:?}", DmntkError::new("ParserError", "unexpected end of file"))
    );
  }

  #[test]
  fn test_equal() {
    assert!((DmntkError::new("ParserError", "unexpected end of file") == DmntkError::new("ParserError", "unexpected end of file")));
  }

  #[test]
  fn test_not_equal() {
    assert!((DmntkError::new("ParserError", "unexpected end of files") != DmntkError::new("ParserError", "unexpected end of file")));
  }
}
