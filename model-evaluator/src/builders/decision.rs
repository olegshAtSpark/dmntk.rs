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

//! Builder for decision evaluators.

use crate::builders::Variable;
use crate::errors::*;
use crate::model_evaluator::ModelEvaluator;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{Name, Scope};
use dmntk_model::model::{Decision, Definitions, DmnElement, KnowledgeRequirement, NamedElement, RequiredVariable};
use std::collections::HashMap;
use std::sync::Arc;

/// Type of closure that evaluates a decision.
///
/// `Fn(input data, model evaluator, output data)`
///
type DecisionEvaluatorFn = Box<dyn Fn(&FeelContext, &ModelEvaluator, &mut FeelContext) -> Name + Send + Sync>;

///
type DecisionEvaluatorEntry = (Variable, DecisionEvaluatorFn);

///
#[derive(Default)]
pub struct DecisionEvaluator {
  evaluators: HashMap<String, DecisionEvaluatorEntry>,
}

impl DecisionEvaluator {
  /// Creates a new decision evaluator.
  pub fn build(&mut self, definitions: &Definitions, model_evaluator: &ModelEvaluator) -> Result<()> {
    for decision in definitions.decisions() {
      let evaluator_entry = build_decision_evaluator(definitions, decision, model_evaluator)?;
      let decision_id = decision.id().as_ref().ok_or_else(err_empty_identifier)?;
      let decision_name = &decision.name().to_string();
      self.evaluators.insert(decision_id.to_owned(), evaluator_entry);
      model_evaluator.add_invocable_decision(decision_name, decision_id);
    }
    Ok(())
  }
  /// Evaluates a decision with specified identifier.
  pub fn evaluate(&self, decision_id: &str, input_data: &FeelContext, model_evaluator: &ModelEvaluator, evaluated_ctx: &mut FeelContext) -> Option<Name> {
    self
      .evaluators
      .get(decision_id)
      .map(|evaluator_entry| evaluator_entry.1(input_data, model_evaluator, evaluated_ctx))
  }
  /// Returns the name and type of the output variable of a decision with specified identifier.
  pub fn get_output_variable(&self, decision_id: &str) -> Option<&Variable> {
    self.evaluators.get(decision_id).map(|entry| &entry.0)
  }
}

///
fn build_decision_evaluator(definitions: &Definitions, decision: &Decision, model_evaluator: &ModelEvaluator) -> Result<DecisionEvaluatorEntry> {
  // acquire all needed evaluators
  let item_definition_type_evaluator = model_evaluator.item_definition_type_evaluator()?;
  let item_definition_context_evaluator = model_evaluator.item_definition_context_evaluator()?;
  let input_data_context_evaluator = model_evaluator.input_data_context_evaluator()?;
  // get output variable
  let output_variable = Variable::try_from(decision.variable())?;
  // prepare output variable name for this decision
  let output_variable_name = output_variable.name.clone();
  // prepare output variable type for this decision
  let output_variable_type = output_variable.feel_type(&item_definition_type_evaluator);
  // prepare expression instance for this decision
  let expression_instance = decision.decision_logic().as_ref().ok_or_else(err_empty_decision_logic)?;
  let mut ctx = FeelContext::default();
  // bring into context the variables from this decision's knowledge requirements
  bring_knowledge_requirements_into_context(definitions, decision.knowledge_requirements(), &mut ctx)?;
  // bring into context the variables from information requirements
  for information_requirement in decision.information_requirements() {
    // bring into context the variable from required decision
    if let Some(href) = information_requirement.required_decision() {
      if let Some(required_decision) = definitions.decision_by_id(href.into()) {
        let output_variable_name = required_decision.variable().feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
        ctx.set_null(output_variable_name);
        // bring into context the variables from this required decision's knowledge requirements
        bring_knowledge_requirements_into_context(definitions, required_decision.knowledge_requirements(), &mut ctx)?;
      }
    }
    // bring into context the variable from required input
    if let Some(href) = information_requirement.required_input() {
      //TODO checked unused returned type
      let _ = input_data_context_evaluator.eval(href.into(), &mut ctx, &item_definition_context_evaluator);
    }
  }
  // prepare a scope and build expression instance evaluator
  let scope: Scope = ctx.into();
  let evaluator = crate::builders::build_expression_instance_evaluator(&scope, expression_instance)?;
  // prepare references to required knowledge, decisions and input data
  let mut required_knowledge_references: Vec<String> = vec![];
  let mut required_decision_references: Vec<String> = vec![];
  let mut required_input_data_references: Vec<String> = vec![];
  for knowledge_requirement in decision.knowledge_requirements() {
    if let Some(href) = knowledge_requirement.required_knowledge() {
      required_knowledge_references.push(href.into());
    }
  }
  for information_requirement in decision.information_requirements() {
    if let Some(href) = information_requirement.required_decision() {
      required_decision_references.push(href.into())
    }
    if let Some(href) = information_requirement.required_input() {
      required_input_data_references.push(href.into())
    }
  }
  // build decision evaluator closure
  let decision_evaluator = Box::new(
    move |input_data_ctx: &FeelContext, model_evaluator: &ModelEvaluator, output_data_ctx: &mut FeelContext| {
      // acquire all evaluators needed
      if let Ok(business_knowledge_model_evaluator) = model_evaluator.business_knowledge_model_evaluator() {
        if let Ok(decision_service_evaluator) = model_evaluator.decision_service_evaluator() {
          if let Ok(decision_evaluator) = model_evaluator.decision_evaluator() {
            if let Ok(input_data_evaluator) = model_evaluator.input_data_evaluator() {
              if let Ok(item_definition_evaluator) = model_evaluator.item_definition_evaluator() {
                // prepare context containing values from required knowledge and required decisions
                let mut required_knowledge_ctx: FeelContext = Default::default();
                // evaluate required knowledge as values from business knowledge models
                required_knowledge_references.iter().for_each(|business_knowledge_model_identifier| {
                  business_knowledge_model_evaluator.evaluate(
                    business_knowledge_model_identifier,
                    input_data_ctx,
                    model_evaluator,
                    &mut required_knowledge_ctx,
                  )
                });
                // evaluate required knowledge as decision service function definitions
                required_knowledge_references.iter().for_each(|decision_service_id| {
                  decision_service_evaluator.evaluate_as_function_definition(decision_service_id, input_data_ctx, &mut required_knowledge_ctx)
                });
                // evaluate required decisions as values from decisions
                required_decision_references.iter().for_each(|decision_identifier| {
                  decision_evaluator.evaluate(decision_identifier, input_data_ctx, model_evaluator, &mut required_knowledge_ctx);
                });
                // values from required knowledge may be overridden by input data
                required_knowledge_ctx.overwrite(input_data_ctx);
                // prepare context containing values from required input data
                let mut required_input_ctx: FeelContext = Default::default();
                let input_data = Value::Context(input_data_ctx.clone());
                required_input_data_references.iter().for_each(|input_data_id| {
                  if let Some((name, value)) = input_data_evaluator.evaluate(input_data_id, &input_data, &item_definition_evaluator) {
                    required_input_ctx.set_entry(&name, value);
                  }
                });
                required_input_ctx.zip(&required_knowledge_ctx);
                // place the result under the name of the output variable
                let scope: Scope = required_input_ctx.into();
                let decision_result = evaluator(&scope);
                let coerced_decision_result = output_variable_type.coerced(&decision_result);
                output_data_ctx.set_entry(&output_variable_name, coerced_decision_result);
              }
            }
          }
        }
      }
      // return the name of the output variable
      output_variable_name.clone()
    },
  );
  // return the output variable, and decision evaluator closure
  Ok((output_variable, decision_evaluator))
}

///
fn bring_knowledge_requirements_into_context(
  definitions: &Definitions,
  knowledge_requirements: &[Arc<KnowledgeRequirement>],
  ctx: &mut FeelContext,
) -> Result<()> {
  for knowledge_requirement in knowledge_requirements {
    let href = knowledge_requirement.required_knowledge().as_ref().ok_or_else(err_empty_reference)?;
    let required_knowledge_id: &str = href.into();
    if let Some(business_knowledge_model) = definitions.business_knowledge_model_by_id(required_knowledge_id) {
      let output_variable_name = business_knowledge_model
        .variable()
        .feel_name()
        .as_ref()
        .ok_or_else(err_empty_feel_name)?
        .clone();
      ctx.set_null(output_variable_name);
      bring_knowledge_requirements_into_context(definitions, business_knowledge_model.knowledge_requirements(), ctx)?;
    } else if let Some(decision_service) = definitions.decision_service_by_id(required_knowledge_id) {
      let output_variable_name = decision_service.variable().feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
      ctx.set_null(output_variable_name);
    } else {
      return Err(err_business_knowledge_model_with_reference_not_found(required_knowledge_id));
    }
  }
  Ok(())
}
