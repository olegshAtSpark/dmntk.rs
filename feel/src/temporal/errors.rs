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

//! Implementation of FEEL temporal errors.

use dmntk_common::DmntkError;

/// FEEL temporal error.
struct TemporalError(String);

impl From<TemporalError> for DmntkError {
  /// Converts temporal error into [DmntkError].
  fn from(e: TemporalError) -> Self {
    DmntkError::new("TemporalError", &e.0)
  }
}

pub fn err_invalid_time_literal(s: &str) -> DmntkError {
  TemporalError(format!("invalid time literal '{}'", s)).into()
}

pub fn err_invalid_date_time_literal(s: &str) -> DmntkError {
  TemporalError(format!("invalid date and time literal '{}'", s)).into()
}

pub fn err_invalid_years_and_months_duration_literal(s: &str) -> DmntkError {
  TemporalError(format!("invalid years and months literal '{}'", s)).into()
}
