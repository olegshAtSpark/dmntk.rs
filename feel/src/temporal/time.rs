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

/// Definitions of time errors.
pub mod errors {
  use dmntk_common::DmntkError;

  /// Time errors.
  #[derive(Debug, PartialEq)]
  enum FeelTimeError {
    InvalidTimeLiteral(String),
  }

  //TODO https://github.com/EngosSoftware/dmntk/issues/1
  impl From<FeelTimeError> for DmntkError {
    fn from(e: FeelTimeError) -> Self {
      DmntkError::new("FeelTimeError", &format!("{}", e))
    }
  }

  impl std::fmt::Display for FeelTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        FeelTimeError::InvalidTimeLiteral(literal) => {
          write!(f, "invalid time literal: {}", literal)
        }
      }
    }
  }

  pub fn invalid_time_literal(literal: String) -> DmntkError {
    FeelTimeError::InvalidTimeLiteral(literal).into()
  }
}
