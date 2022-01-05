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

//! Builder for business knowledge model evaluators.

use crate::builders::information_item_type;
use crate::errors::*;
use crate::model_evaluator::ModelEvaluator;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{FeelType, FunctionBody, Name, Scope};
use dmntk_model::model::{
  BusinessKnowledgeModel, Context, DecisionTable, Definitions, DmnElement, ExpressionInstance, FunctionDefinition, Invocation, LiteralExpression, NamedElement,
  Relation, RequiredVariable,
};
use std::collections::HashMap;
use std::rc::Rc;

/// Type of closure that evaluates business knowledge model.
///
/// Fn(input data, model evaluator, output data)
///
type BusinessKnowledgeModelEvaluatorFn = Box<dyn Fn(&FeelContext, &ModelEvaluator, &mut FeelContext)>;

/// Business knowledge model evaluator.
#[derive(Default)]
pub struct BusinessKnowledgeModelEvaluator {
  evaluators: HashMap<String, BusinessKnowledgeModelEvaluatorFn>,
}

impl BusinessKnowledgeModelEvaluator {
  /// Creates a new business knowledge model evaluator.
  pub fn build(&mut self, definitions: &Definitions, model_evaluator: &ModelEvaluator) -> Result<()> {
    for business_knowledge_model in definitions.business_knowledge_models() {
      let function_definition = business_knowledge_model
        .encapsulated_logic()
        .as_ref()
        .ok_or_else(err_empty_encapsulated_logic)?;
      let evaluator = build_business_knowledge_model_evaluator(business_knowledge_model, function_definition, model_evaluator)?;
      let business_knowledge_model_id = business_knowledge_model.id().as_ref().ok_or_else(err_empty_identifier)?;
      self.evaluators.insert(business_knowledge_model_id.to_owned(), evaluator);
    }
    Ok(())
  }
  /// Evaluates a business knowledge model with specified identifier.
  /// When a required business knowledge model is found, then its evaluator
  /// is executed, and the result is stored in `evaluated_ctx`.
  pub fn evaluate(&self, id: &str, input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext) {
    if let Some(evaluator) = self.evaluators.get(id) {
      evaluator(input_data, model_evaluator, output_data);
    }
  }
}

///
fn build_business_knowledge_model_evaluator(
  business_knowledge_model: &BusinessKnowledgeModel,
  function_definition: &FunctionDefinition,
  model_evaluator: &ModelEvaluator,
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let mut local_context = FeelContext::default();
  let mut formal_parameters = vec![];
  for information_item in function_definition.formal_parameters() {
    let feel_type = if let Some(type_ref) = information_item.type_ref() {
      information_item_type(type_ref, &model_evaluator.item_definition_type_evaluator()).ok_or_else(err_empty_feel_type)?
    } else {
      FeelType::Any
    };
    let feel_name = information_item.feel_name().as_ref().ok_or_else(err_empty_feel_name)?;
    formal_parameters.push((feel_name.clone(), feel_type.clone()));
    local_context.set_entry(feel_name, Value::FeelType(feel_type));
  }
  //TODO replace this long evaluation with single function that return Result!!! feel_name() -> Result<Name>
  let output_variable_name = business_knowledge_model
    .variable()
    .feel_name()
    .as_ref()
    .ok_or_else(err_empty_feel_name)?
    .clone();
  // output variable type
  let output_variable_type = if let Some(output_variable_type_ref) = business_knowledge_model.variable().type_ref().as_ref() {
    information_item_type(output_variable_type_ref, &model_evaluator.item_definition_type_evaluator()).unwrap_or(FeelType::Any)
  } else {
    FeelType::Any
  };
  let mut knowledge_requirements = vec![];
  for knowledge_requirement in business_knowledge_model.knowledge_requirements() {
    let href = knowledge_requirement.required_knowledge().as_ref().ok_or_else(err_empty_reference)?;
    knowledge_requirements.push(href.into());
  }
  if let Some(expression_instance) = function_definition.body() {
    let scope: Scope = local_context.into();
    build_expression_instance_evaluator(
      &scope,
      &formal_parameters,
      expression_instance,
      output_variable_name,
      output_variable_type,
      &knowledge_requirements,
    )
  } else {
    Ok(Box::new(move |_: &FeelContext, _: &ModelEvaluator, _: &mut FeelContext| ()))
  }
}

///
fn build_expression_instance_evaluator(
  scope: &Scope,
  formal_parameters: &[(Name, FeelType)],
  expression_instance: &ExpressionInstance,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: &[String],
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  match expression_instance {
    ExpressionInstance::Context(context) => build_context_evaluator(
      scope,
      formal_parameters,
      context,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
    ),
    ExpressionInstance::DecisionTable(decision_table) => build_decision_table_evaluator(
      scope,
      formal_parameters,
      decision_table,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
    ),
    ExpressionInstance::FunctionDefinition(function_definition) => build_function_definition_evaluator(
      scope,
      formal_parameters,
      function_definition,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
    ),
    ExpressionInstance::Invocation(invocation) => build_invocation_evaluator(
      scope,
      formal_parameters,
      invocation,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
    ),
    ExpressionInstance::LiteralExpression(literal_expression) => build_literal_expression_evaluator(
      scope,
      formal_parameters,
      literal_expression,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
    ),
    ExpressionInstance::Relation(relation) => build_relation_evaluator(
      scope,
      formal_parameters,
      relation,
      output_variable_name,
      output_variable_type,
      knowledge_requirements,
    ),
  }
}

///
fn build_context_evaluator(
  scope: &Scope,
  formal_parameters: &[(Name, FeelType)],
  context: &Context,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: &[String],
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let evaluator = crate::builders::build_context_evaluator(scope, context)?;
  let function = Value::FunctionDefinition(formal_parameters.to_owned(), FunctionBody::Context(Rc::new(evaluator)), output_variable_type);
  build_evaluator(output_variable_name, function, knowledge_requirements)
}

///
fn build_decision_table_evaluator(
  scope: &Scope,
  formal_parameters: &[(Name, FeelType)],
  decision_table: &DecisionTable,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: &[String],
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let evaluator = crate::builders::build_decision_table_evaluator(scope, decision_table)?;
  let function = Value::FunctionDefinition(
    formal_parameters.to_owned(),
    FunctionBody::DecisionTable(Rc::new(evaluator)),
    output_variable_type,
  );
  build_evaluator(output_variable_name, function, knowledge_requirements)
}

///
fn build_function_definition_evaluator(
  scope: &Scope,
  formal_parameters: &[(Name, FeelType)],
  function_definition: &FunctionDefinition,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: &[String],
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let evaluator = crate::builders::build_function_definition_evaluator(scope, function_definition)?;
  let function = Value::FunctionDefinition(
    formal_parameters.to_owned(),
    FunctionBody::DecisionTable(Rc::new(evaluator)),
    output_variable_type,
  );
  build_evaluator(output_variable_name, function, knowledge_requirements)
}

///
fn build_invocation_evaluator(
  scope: &Scope,
  formal_parameters: &[(Name, FeelType)],
  invocation: &Invocation,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: &[String],
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let evaluator = crate::builders::build_invocation_evaluator(scope, invocation)?;
  let function = Value::FunctionDefinition(
    formal_parameters.to_owned(),
    FunctionBody::DecisionTable(Rc::new(evaluator)),
    output_variable_type,
  );
  build_evaluator(output_variable_name, function, knowledge_requirements)
}

///
fn build_literal_expression_evaluator(
  scope: &Scope,
  formal_parameters: &[(Name, FeelType)],
  literal_expression: &LiteralExpression,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: &[String],
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let evaluator = crate::builders::build_literal_expression_evaluator(scope, literal_expression)?;
  let function = Value::FunctionDefinition(
    formal_parameters.to_owned(),
    FunctionBody::LiteralExpression(Rc::new(evaluator)),
    output_variable_type,
  );
  build_evaluator(output_variable_name, function, knowledge_requirements)
}

///
fn build_relation_evaluator(
  scope: &Scope,
  formal_parameters: &[(Name, FeelType)],
  relation: &Relation,
  output_variable_name: Name,
  output_variable_type: FeelType,
  knowledge_requirements: &[String],
) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let evaluator = crate::builders::build_relation_evaluator(scope, relation)?;
  let function = Value::FunctionDefinition(
    formal_parameters.to_owned(),
    FunctionBody::LiteralExpression(Rc::new(evaluator)),
    output_variable_type,
  );
  build_evaluator(output_variable_name, function, knowledge_requirements)
}

///
fn build_evaluator(name: Name, function: Value, knowledge_requirements: &[String]) -> Result<BusinessKnowledgeModelEvaluatorFn> {
  let requirements = knowledge_requirements.to_owned();
  Ok(Box::new(
    move |input_data: &FeelContext, model_evaluator: &ModelEvaluator, output_data: &mut FeelContext| {
      requirements.iter().for_each(|id| {
        //TODO refactor: call either business knowledge model or decision service, not both!
        model_evaluator
          .business_knowledge_model_evaluator()
          .evaluate(id, input_data, model_evaluator, output_data);
        model_evaluator
          .decision_service_evaluator()
          .evaluate(id, input_data, model_evaluator, output_data);
      });
      output_data.set_entry(&name, function.clone())
    },
  ))
}
