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

extern crate dmntk_common;
extern crate dmntk_feel;
extern crate dmntk_feel_parser;
extern crate dmntk_model;
extern crate dmntk_recognizer;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate thiserror;

mod builders;
mod errors;
mod eval_bkm;
mod eval_dec;
mod eval_dec_service;
mod model_evaluator;

#[cfg(test)]
mod tests;

use dmntk_common::Result;
use dmntk_feel::values::Value;
use dmntk_feel::Scope;
pub use eval_bkm::evaluate_business_knowledge_model_by_name;
pub use eval_dec::evaluate_decision_by_name;
pub use eval_dec_service::eval_decision_service_by_name;

/// Evaluates the decision table.
pub fn evaluate_decision_table_from_text(_scope: &Scope, _input: &str) -> Result<Value> {
  unimplemented!()
}

/// Evaluates a decision table against specified context.
pub fn evaluate_decision_table_and_context(_decision_table_input: &str, _context_input: &str) -> Result<Value> {
  unimplemented!()
}

/// Evaluates all tests associated with decision table.
pub fn evaluate_decision_table_and_test(_input: &str, _sep: &str) -> Result<(bool, Value, Value)> {
  unimplemented!()
}
