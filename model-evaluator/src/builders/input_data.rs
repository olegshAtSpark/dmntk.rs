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

//! Input data evaluator.

use crate::builders::item_definition::ItemDefinitionEvaluator;
use crate::builders::{build_variable_evaluator, Variable, VariableEvaluatorFn};
use crate::errors::err_empty_identifier;
use dmntk_common::Result;
use dmntk_feel::values::Value;
use dmntk_feel::Name;
use dmntk_model::model::{Definitions, DmnElement, RequiredVariable};
use std::collections::HashMap;

///
type InputDataEvaluatorEntry = (Variable, VariableEvaluatorFn);

/// Input data evaluator.
#[derive(Default)]
pub struct InputDataEvaluator {
  evaluators: HashMap<String, InputDataEvaluatorEntry>,
}

impl InputDataEvaluator {
  /// Builds a new input data evaluator.
  pub fn build(&mut self, definitions: &Definitions) -> Result<()> {
    for input_data in definitions.input_data() {
      let input_data_id = input_data.id().as_ref().ok_or_else(err_empty_identifier)?;
      let variable = Variable::try_from(input_data.variable())?;
      let evaluator = build_variable_evaluator(&variable)?;
      self.evaluators.insert(input_data_id.to_owned(), (variable, evaluator));
    }
    Ok(())
  }
  /// Evaluates input data with specified identifier.
  pub fn evaluate(&self, input_data_id: &str, value: &Value, item_definition_evaluator: &ItemDefinitionEvaluator) -> Option<(Name, Value)> {
    self
      .evaluators
      .get(input_data_id)
      .map(|evaluator| evaluator.1(value, item_definition_evaluator))
  }
  /// Returns the name and type of the input variable of input data definition with specified identifier.
  pub fn get_input_variable(&self, input_data_id: &str) -> Option<&Variable> {
    self.evaluators.get(input_data_id).map(|entry| &entry.0)
  }
}

#[cfg(test)]
mod tests {
  use crate::builders::item_definition::ItemDefinitionEvaluator;
  use crate::builders::InputDataEvaluator;
  use dmntk_examples::input_data::*;
  use dmntk_feel::values::Value;
  use dmntk_feel::{value_number, FeelNumber, Name};

  /// Utility function for building input data evaluator from definitions,
  /// and item definition evaluator from definitions.
  fn build_evaluators(xml: &str, source: &str) -> (InputDataEvaluator, ItemDefinitionEvaluator) {
    let definitions = &dmntk_model::parse(xml, source).unwrap();
    let mut input_data_evaluator = InputDataEvaluator::default();
    input_data_evaluator.build(definitions).unwrap();
    let mut item_definitions_evaluator = ItemDefinitionEvaluator::default();
    item_definitions_evaluator.build(definitions).unwrap();
    (input_data_evaluator, item_definitions_evaluator)
  }

  #[test]
  fn _0001_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0001, "file:///0001.dmn");
    let context_str = r#"{Full Name: "John"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::String("John".to_string()))),
      input_data_evaluator.evaluate("_cba86e4d-e91c-46a2-9176-e9adf88e15db", &Value::Context(context), &item_definitions_evaluator)
    );
    let context_str = r#"{Full Name: "Phillip"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::String("Phillip".to_string()))),
      input_data_evaluator.evaluate("_cba86e4d-e91c-46a2-9176-e9adf88e15db", &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0001_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0001, "file:///0001.dmn");
    let context_str = r#"{Full Name: 50.0}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Full", "Name"]), Value::Null(None))),
      input_data_evaluator.evaluate("_cba86e4d-e91c-46a2-9176-e9adf88e15db", &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0002_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0002, "file:///0002.dmn");
    let context_str = r#"{Monthly Salary: 12000.00}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), value_number!(12000))),
      input_data_evaluator.evaluate("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluator)
    );
    let context_str = r#"{Monthly Salary: 8135.35}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), value_number!(813535, 2))),
      input_data_evaluator.evaluate("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0002_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0002, "file:///0002.dmn");
    let context_str = r#"{Monthly Salary: "12000.00"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Monthly", "Salary"]), Value::Null(None))),
      input_data_evaluator.evaluate("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0003_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0003, "file:///0003.dmn");
    let context_str = r#"{Is Affordable: true}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Boolean(true))),
      input_data_evaluator.evaluate("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluator)
    );
    let context_str = r#"{Is Affordable: false}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Boolean(false))),
      input_data_evaluator.evaluate("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0003_2() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0003, "file:///0003.dmn");
    let context_str = r#"{Is Affordable: "no"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    assert_eq!(
      Some((Name::new(&["Is", "Affordable"]), Value::Null(None))),
      input_data_evaluator.evaluate("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluator)
    );
  }

  #[test]
  fn _0103_1() {
    let (input_data_evaluator, item_definitions_evaluator) = build_evaluators(DMN_0103, "file:///0103.dmn");
    let context_str = r#"{Employment Status: "EMPLOYED"}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let name = Name::new(&["Employment", "Status"]);
    assert_eq!(
      Some((name, Value::String("EMPLOYED".to_string()))),
      input_data_evaluator.evaluate("_acfd4e1d-da0a-4842-aa35-ea50dd36fb01", &Value::Context(context), &item_definitions_evaluator)
    );
  }
}
