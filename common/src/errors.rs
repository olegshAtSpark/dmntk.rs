/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * Common definitions.
 *
 * Copyright 2018-2021 Dariusz Depta Engos Software <dariusz.depta@engos.software>
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Definition of common error type.

/// Common result type
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
  fn test_new_dmntk_error() {
    assert_eq!(
      "ParserError: unexpected end of file",
      format!("{}", DmntkError::new("ParserError", "unexpected end of file"))
    );
  }
}
