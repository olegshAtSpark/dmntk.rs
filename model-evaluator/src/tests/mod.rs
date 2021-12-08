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

use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::Scope;
use dmntk_model::model::Definitions;

mod compliance;

/// Utility function that creates a `FEEL` context from specified input expression.
pub fn context(input: &str) -> FeelContext {
  let scope = Scope::default();
  match dmntk_feel_parser::parse_context(&scope, input, false) {
    Ok(node) => match dmntk_feel_evaluator::prepare(&node) {
      Ok(evaluator) => match evaluator(&scope) {
        Value::Context(ctx) => ctx,
        other => panic!("ERROR: expected context value, actual value is: {}", other as Value),
      },
      Err(reason) => panic!("ERROR: building evaluator failed with reason: {}", reason),
    },
    Err(reason) => panic!("ERROR: parsing context failed with reason: {}", reason),
  }
}

/// Utility function that evaluates a `Decision` specified by name and compares the result.
fn assert_decision(definitions: &Definitions, name: &str, input_data: &FeelContext, expected: &str) {
  let actual = crate::evaluate_decision_by_name(definitions, name, input_data).unwrap().to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision does not match the expected value:\n  expected: {}\n    actual: {}\n",
    expected, actual
  );
}

/// Utility function that evaluates a `BusinessKnowledgeModel` specified by name and compares the result.
fn assert_business_knowledge_model(definitions: &Definitions, name: &str, input_data: &FeelContext, expected: &str) {
  let actual = crate::evaluate_business_knowledge_model_by_name(definitions, name, input_data)
    .unwrap()
    .to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the business knowledge model does not match the expected value:\n  expected: {}\n    actual: {}\n",
    expected, actual
  );
}

/// Utility function that evaluates a `DecisionService` specified by name and compares the result with expected value.
fn assert_decision_service(definitions: &Definitions, name: &str, input: &str, expected: &str) {
  let actual = crate::eval_decision_service_by_name(definitions, name, &context(input)).unwrap().to_string();
  assert_eq!(
    expected, actual,
    "Assertion error, actual value of the decision service does not match the expected value:\n  expected: {}\n    actual: {}\n",
    expected, actual
  );
}
