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

//! Builder for decision service evaluators.

use crate::builders::{build_variable_evaluator, Variable};
use crate::errors::*;
use crate::model_evaluator::ModelEvaluator;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, Evaluator, Name, Scope};
use dmntk_model::model::{DecisionService, Definitions, DmnElement, RequiredVariable};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

/// Type of closure that evaluates a decision service.
///
/// Fn(input_data, model evaluator, output data)
///
type DecisionServiceEvaluatorFn = Box<dyn Fn(&FeelContext, &ModelEvaluator, &mut FeelContext) -> Name>;

///
type DecisionServiceEvaluatorEntry = (Variable, DecisionServiceEvaluatorFn, Evaluator);

///
#[derive(Default)]
pub struct DecisionServiceEvaluator {
  evaluators: HashMap<String, DecisionServiceEvaluatorEntry>,
}

impl DecisionServiceEvaluator {
  /// Creates a new decision evaluator.
  pub fn build(&mut self, definitions: &Definitions, model_evaluator: Rc<ModelEvaluator>) -> Result<()> {
    for decision_service in definitions.decision_services() {
      let decision_service_id = decision_service.id().as_ref().ok_or_else(err_empty_identifier)?;
      let evaluator = build_decision_service_evaluator(decision_service, decision_service_id.clone(), Rc::clone(&model_evaluator))?;
      self.evaluators.insert(decision_service_id.to_owned(), evaluator);
    }
    Ok(())
  }
  /// Evaluates a decision service with specified identifier.
  pub fn evaluate(&self, id: &str, input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext) -> Option<Name> {
    self.evaluators.get(id).map(|entry| entry.1(input_data, model_evaluator, output_data))
  }
  /// Returns a decision service as function definition with specified identifier.
  pub fn evaluate_as_function_definition(&self, id: &str, input_data: &FeelContext, output_data: &mut FeelContext) {
    if let Some(entry) = self.evaluators.get(id) {
      let scope: Scope = input_data.clone().into();
      let function_definition = entry.2(&scope) as Value;
      let output_variable_name = entry.0.name.clone();
      output_data.set_entry(&output_variable_name, function_definition);
    }
  }
}

///
fn build_decision_service_evaluator(
  decision_service: &DecisionService,
  decision_service_id: String,
  model_evaluator: Rc<ModelEvaluator>,
) -> Result<DecisionServiceEvaluatorEntry> {
  let output_variable = Variable::try_from(decision_service.variable())?;
  // prepare output variable name for this decision
  let output_variable_name = output_variable.name.clone();
  // prepare output variable type for this decision
  let output_variable_type = output_variable.feel_type(&model_evaluator.item_definition_type_evaluator());
  // prepare references to required input data
  let input_data_references: Vec<String> = decision_service.input_data().iter().map(|href| href.into()).collect();
  // prepare references to input decisions
  let input_decisions: Vec<String> = decision_service.input_decisions().iter().map(|href| href.into()).collect();
  // prepare references to encapsulated decisions
  let encapsulated_decisions: Vec<String> = decision_service.encapsulated_decisions().iter().map(|href| href.into()).collect();
  // prepare references to output decisions
  let output_decisions: Vec<String> = decision_service.output_decisions().iter().map(|href| href.into()).collect();
  // prepare a container for formal parameters
  let mut formal_parameters = vec![];
  // fills the list of formal parameters based on required input data
  // these parameters are placed before input parameters defined by input decisions
  for input_data_id in &input_data_references {
    if let Some(input_data_variable) = model_evaluator.input_data_evaluator().get_input_variable(input_data_id) {
      let parameter_name = input_data_variable.name.clone();
      let parameter_type = input_data_variable.feel_type(&model_evaluator.item_definition_type_evaluator());
      formal_parameters.push((parameter_name, parameter_type));
    }
  }
  // prepare evaluators from output variables of input decisions
  // these evaluators will evaluate results from input decisions and values from input data
  // simultaneously, fills the list of formal parameters based on output variables of input decisions
  let mut input_decision_results_evaluators = vec![];
  for decision_id in &input_decisions {
    if let Some(decision_output_variable) = model_evaluator.decision_evaluator().get_output_variable(decision_id) {
      let parameter_name = decision_output_variable.name.clone();
      let parameter_type = decision_output_variable.feel_type(&model_evaluator.item_definition_type_evaluator());
      formal_parameters.push((parameter_name, parameter_type));
      let evaluator = build_variable_evaluator(decision_output_variable)?;
      input_decision_results_evaluators.push(evaluator);
    }
  }
  // clone the output variable type for later use
  let output_variable_type_clone = output_variable_type.clone();
  // build decision service evaluator closure
  let decision_service_evaluator = Box::new(
    move |input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext| {
      // evaluate input decisions and store the results in separate context
      let mut input_decisions_results = FeelContext::default();
      input_decisions.iter().for_each(|id| {
        model_evaluator
          .decision_evaluator()
          .evaluate(id, input_data, model_evaluator, &mut input_decisions_results);
      });
      // now evaluate input data for encapsulated and output decisions and store them in separate context
      let mut evaluated_input_data = FeelContext::default();
      // first take values from evaluated input decisions...
      let input_decision_results_value = Value::Context(input_decisions_results);
      for evaluator in &input_decision_results_evaluators {
        let (name, value) = evaluator(&input_decision_results_value, &model_evaluator.item_definition_evaluator());
        evaluated_input_data.set_entry(&name, value);
      }
      // ...and then take values from provided input data
      let input_data_values = Value::Context(input_data.clone());
      for evaluator in &input_decision_results_evaluators {
        let (name, value) = evaluator(&input_data_values, &model_evaluator.item_definition_evaluator());
        evaluated_input_data.set_entry(&name, value);
      }
      // evaluate required inputs (from required input data references)
      let input_data_evaluator = &model_evaluator.borrow().input_data_evaluator();
      input_data_references.iter().for_each(|input_data_id| {
        if let Some((name, value)) = input_data_evaluator.evaluate(input_data_id, &input_data_values, &model_evaluator.item_definition_evaluator()) {
          evaluated_input_data.set_entry(&name, value);
        }
      });

      // prepare context for evaluated result data for this decision service
      let mut evaluated_ctx = FeelContext::default();
      // evaluate encapsulated decisions
      encapsulated_decisions.iter().for_each(|id| {
        model_evaluator
          .decision_evaluator()
          .evaluate(id, &evaluated_input_data, model_evaluator, &mut evaluated_ctx);
      });
      // evaluate output decisions
      let mut output_names = vec![];
      output_decisions.iter().for_each(|id| {
        if let Some(output_name) = model_evaluator
          .decision_evaluator()
          .evaluate(id, &evaluated_input_data, model_evaluator, &mut evaluated_ctx)
        {
          output_names.push(output_name);
        }
      });
      // prepare the result from this decision service
      if output_names.len() == 1 {
        if let Some(value) = evaluated_ctx.get_entry(&output_names[0]) {
          let single_result = value.to_owned();
          let coerced_single_result = output_variable_type.coerced(&single_result);
          output_data.set_entry(&output_variable_name, coerced_single_result);
        }
      } else {
        let mut output_ctx = FeelContext::default();
        output_names.iter().for_each(|output_name| {
          if let Some(value) = evaluated_ctx.get_entry(output_name) {
            output_ctx.set_entry(output_name, value.to_owned());
          }
        });
        let complex_result = Value::Context(output_ctx);
        let coerced_complex_result = output_variable_type.coerced(&complex_result);
        output_data.set_entry(&output_variable_name, coerced_complex_result);
      }
      output_variable_name.clone()
    },
  );
  // prepare function body for function definition to be built from decision service evaluator closure
  let body_evaluator = Box::new(move |scope: &Scope| {
    let input_data = scope.peek();
    let mut output_data = FeelContext::default();
    let opt_out_variable_name = model_evaluator
      .decision_service_evaluator()
      .evaluate(&decision_service_id, &input_data, &model_evaluator, &mut output_data);
    if let Some(out_variable_name) = opt_out_variable_name {
      if let Some(result_value) = output_data.get_entry(&out_variable_name) {
        return result_value.clone();
      }
    }
    value_null!()
  });
  let function_body = dmntk_feel::FunctionBody::DecisionService(Rc::new(body_evaluator));
  let function_definition = Value::FunctionDefinition(formal_parameters, function_body, output_variable_type_clone);
  let decision_service_as_function_definition_evaluator = Box::new(move |_: &Scope| function_definition.clone());
  // return decision service evaluator closure and evaluator of the decision service as function definition
  Ok((output_variable, decision_service_evaluator, decision_service_as_function_definition_evaluator))
}
