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

//! Input data context evaluator.

use crate::builders::{type_ref_to_feel_type, ItemDefinitionContextEvaluator};
use crate::errors::{err_empty_feel_name, err_empty_identifier, err_input_data_without_type_reference, err_unsupported_feel_type};
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::FeelType;
use dmntk_model::model::{Definitions, DmnElement, InputData, NamedElement, RequiredVariable};
use std::collections::HashMap;

/// Type of closure that evaluates input data context.
type InputDataContextEvaluatorFn = Box<dyn Fn(&mut FeelContext, &ItemDefinitionContextEvaluator) -> FeelType>;

/// Input data context evaluator.
#[derive(Default)]
pub struct InputDataContextEvaluator {
  evaluators: HashMap<String, InputDataContextEvaluatorFn>,
}

impl InputDataContextEvaluator {
  /// Creates a new input data context evaluator.
  pub fn build(&mut self, definitions: &Definitions) -> Result<()> {
    for input_data in definitions.input_data() {
      let input_data_id = input_data.id().as_ref().ok_or_else(err_empty_identifier)?;
      let evaluator = input_data_context_evaluator(input_data)?;
      self.evaluators.insert(input_data_id.to_owned(), evaluator);
    }
    Ok(())
  }
  /// Evaluates input data context with specified identifier.
  pub fn eval(&self, input_data_id: &str, ctx: &mut FeelContext, item_definition_context_evaluator: &ItemDefinitionContextEvaluator) -> FeelType {
    if let Some(evaluator) = self.evaluators.get(input_data_id) {
      evaluator(ctx, item_definition_context_evaluator)
    } else {
      FeelType::Any
    }
  }
}

///
pub fn input_data_context_evaluator(input_data: &InputData) -> Result<InputDataContextEvaluatorFn> {
  let type_ref = input_data
    .variable()
    .type_ref()
    .as_ref()
    .ok_or_else(|| err_input_data_without_type_reference(input_data.name()))?
    .clone();
  let name = input_data.variable().feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
  if let Some(feel_type) = type_ref_to_feel_type(&type_ref) {
    if matches!(
      feel_type,
      FeelType::String
        | FeelType::Number
        | FeelType::Boolean
        | FeelType::Date
        | FeelType::Time
        | FeelType::DateTime
        | FeelType::DaysAndTimeDuration
        | FeelType::YearsAndMonthsDuration
    ) {
      Ok(Box::new(move |ctx: &mut FeelContext, _: &ItemDefinitionContextEvaluator| {
        ctx.set_entry(&name, Value::FeelType(feel_type.clone()));
        feel_type.clone()
      }))
    } else {
      Err(err_unsupported_feel_type(feel_type))
    }
  } else {
    Ok(Box::new(move |ctx: &mut FeelContext, evaluator: &ItemDefinitionContextEvaluator| {
      evaluator.eval(&type_ref, &name, ctx)
    }))
  }
}

#[cfg(test)]
mod tests {
  use crate::builders::input_data_context::InputDataContextEvaluator;
  use crate::builders::ItemDefinitionContextEvaluator;
  use dmntk_examples::input_data::*;
  use dmntk_feel::context::FeelContext;
  use dmntk_feel::FeelType;

  /// Utility function for building input data context evaluator from definitions,
  /// and item definition context evaluator from definitions.
  fn build_evaluators(xml: &str) -> (InputDataContextEvaluator, ItemDefinitionContextEvaluator) {
    let definitions = &dmntk_model::parse(xml).unwrap();
    let mut input_data_context_evaluator = InputDataContextEvaluator::default();
    input_data_context_evaluator.build(definitions).unwrap();
    let mut item_definition_context_evaluator = ItemDefinitionContextEvaluator::default();
    item_definition_context_evaluator.build(definitions).unwrap();
    (input_data_context_evaluator, item_definition_context_evaluator)
  }

  #[test]
  fn _0001_1() {
    let (evaluator, item_definition_context_evaluator) = build_evaluators(DMN_0001);
    let expected_type = FeelType::String;
    let mut ctx = FeelContext::default();
    let actual_type = evaluator.eval("_cba86e4d-e91c-46a2-9176-e9adf88e15db", &mut ctx, &item_definition_context_evaluator);
    assert_eq!(expected_type, actual_type);
    assert_eq!("{Full Name: type(string)}", ctx.to_string());
  }
  /*
    #[test]
    fn _0001_2() {
      let definitions = &dmntk_model::parse(DMN_0001).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Full Name : 50.0 }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Full", "Name"]), Value::Null(None))),
        input_data_evaluators.eval("_cba86e4d-e91c-46a2-9176-e9adf88e15db", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0002_1() {
      let definitions = &dmntk_model::parse(DMN_0002).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Monthly Salary : 12000.00 }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Monthly", "Salary"]), Value::Number(12000.0))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
      let context_str = r#"{ Monthly Salary : 8135.35 }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Monthly", "Salary"]), Value::Number(8135.35))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0002_2() {
      let definitions = &dmntk_model::parse(DMN_0002).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Monthly Salary : "12000.00" }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Monthly", "Salary"]), Value::Null(None))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0003_1() {
      let definitions = &dmntk_model::parse(DMN_0003).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Is Affordable : true }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Is", "Affordable"]), Value::Boolean(true))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
      let context_str = r#"{ Is Affordable : false }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Is", "Affordable"]), Value::Boolean(false))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0003_2() {
      let definitions = &dmntk_model::parse(DMN_0003).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Is Affordable : "no" }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      assert_eq!(
        Some((Name::new(&["Is", "Affordable"]), Value::Null(None))),
        input_data_evaluators.eval("_b7a53bad-7a5b-4033-841d-5db6b25834ad", &Value::Context(context), &item_definitions_evaluators)
      );
    }

    #[test]
    fn _0103_1() {
      let definitions = &dmntk_model::parse(DMN_0103).unwrap();
      let input_data_evaluators = InputDataEvaluator::new(definitions).unwrap();
      let item_definitions_evaluators = ItemDefinitionEvaluator::new(definitions).unwrap();
      let context_str = r#"{ Employment Status : "EMPLOYED" }"#;
      let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
      let name = Name::new(&["Employment", "Status"]);
      assert_eq!(
        Some((name, Value::String("EMPLOYED".to_string()))),
        input_data_evaluators.eval("_acfd4e1d-da0a-4842-aa35-ea50dd36fb01", &Value::Context(context), &item_definitions_evaluators)
      );
    }

  */
}
