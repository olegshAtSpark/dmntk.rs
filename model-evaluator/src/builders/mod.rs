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

//! ???

mod business_knowledge_model;
mod decision;
mod decision_service;
mod decision_table;
mod input_data;
mod input_data_context;
mod item_definition;
mod item_definition_context;
mod item_definition_type;

use crate::errors::*;
pub use business_knowledge_model::BusinessKnowledgeModelEvaluator;
pub use decision::DecisionEvaluator;
pub use decision_service::DecisionServiceEvaluator;
use dmntk_common::{DmntkError, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::{Value, Values};
use dmntk_feel::{value_null, Evaluator, FeelType, Name, Scope};
use dmntk_model::model::{
  Context, DecisionTable, Expression, ExpressionInstance, FunctionDefinition, InformationItem, Invocation, ItemDefinition, ItemDefinitionType,
  LiteralExpression, NamedElement, Relation,
};
pub use input_data::InputDataEvaluator;
pub use input_data_context::InputDataContextEvaluator;
pub use item_definition::ItemDefinitionEvaluator;
pub use item_definition_context::ItemDefinitionContextEvaluator;
pub use item_definition_type::ItemDefinitionTypeEvaluator;

///
pub fn information_item_type(type_ref: &str, evaluator: &ItemDefinitionTypeEvaluator) -> Option<FeelType> {
  if let Some(feel_type) = type_ref_to_feel_type(type_ref) {
    match feel_type {
      FeelType::String => Some(FeelType::String),
      FeelType::Number => Some(FeelType::Number),
      FeelType::Boolean => Some(FeelType::Boolean),
      FeelType::Date => Some(FeelType::Date),
      FeelType::Time => Some(FeelType::Time),
      FeelType::DateTime => Some(FeelType::DateTime),
      FeelType::DaysAndTimeDuration => Some(FeelType::DaysAndTimeDuration),
      FeelType::YearsAndMonthsDuration => Some(FeelType::YearsAndMonthsDuration),
      _ => None,
    }
  } else {
    evaluator.eval(type_ref)
  }
}

///
fn item_definition_type(item_definition: &ItemDefinition) -> Result<ItemDefinitionType> {
  let feel_type = if let Some(type_ref) = item_definition.type_ref() {
    type_ref_to_feel_type(type_ref)
  } else {
    None
  };
  let condition = (
    item_definition.type_ref().is_some(),
    feel_type.is_some(),
    !item_definition.item_components().is_empty(),
    item_definition.is_collection(),
  );
  match condition {
    (_, true, false, false) => Ok(ItemDefinitionType::SimpleType(feel_type.unwrap())),
    (true, false, false, false) => Ok(ItemDefinitionType::ReferencedType(item_definition.type_ref().as_ref().unwrap().clone())),
    (false, false, true, false) => Ok(ItemDefinitionType::ComponentType),
    (_, true, false, true) => Ok(ItemDefinitionType::CollectionOfSimpleType(feel_type.unwrap())),
    (false, false, true, true) => Ok(ItemDefinitionType::CollectionOfComponentType),
    (true, false, false, true) => Ok(ItemDefinitionType::CollectionOfReferencedType(
      item_definition.type_ref().as_ref().unwrap().clone(),
    )),
    _ => Err(err_invalid_item_definition_type(item_definition.name())),
  }
}

///
fn type_ref_to_feel_type(type_ref: &str) -> Option<FeelType> {
  match type_ref.trim() {
    "string" => Some(FeelType::String),
    "number" => Some(FeelType::Number),
    "boolean" => Some(FeelType::Boolean),
    "date" => Some(FeelType::Date),
    "time" => Some(FeelType::Time),
    "dateTime" => Some(FeelType::DateTime),
    "dayTimeDuration" => Some(FeelType::DaysAndTimeDuration),
    "yearMonthDuration" => Some(FeelType::YearsAndMonthsDuration),
    _ => None,
  }
}

/// Type of closure that evaluates values from variable definition.
type VariableEvaluatorFn = Box<dyn Fn(&Value, &ItemDefinitionEvaluator) -> (Name, Value) + Send + Sync>;

///
#[derive(Clone)]
pub struct Variable {
  pub name: Name,
  pub type_ref: Option<String>,
}

impl TryFrom<&InformationItem> for Variable {
  type Error = DmntkError;
  ///
  fn try_from(value: &InformationItem) -> Result<Self, Self::Error> {
    let name = value.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
    let type_ref = value.type_ref().clone();
    Ok(Self { name, type_ref })
  }
}

impl Variable {
  ///
  fn feel_type(&self, item_definition_type_evaluator: &ItemDefinitionTypeEvaluator) -> FeelType {
    if let Some(type_ref) = &self.type_ref {
      information_item_type(type_ref, item_definition_type_evaluator).unwrap_or(FeelType::Any)
    } else {
      FeelType::Any
    }
  }
}

///
fn build_variable_evaluator(variable: &Variable) -> Result<VariableEvaluatorFn> {
  // prepare the variable name
  let variable_name = variable.name.clone();
  println!("AAA: variableName={:?}", variable_name);
  // if there is no type reference defined, the value is just returned as is
  if variable.type_ref.is_none() {
    return Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          return (variable_name.clone(), v.clone());
        }
      }
      (variable_name.clone(), value_null!())
    }));
  }
  // here the `variable.type_ref` must have some value, so unwrapping is safe
  // type_ref is either a simple type name of a name of item definition,
  // both cases are handled below
  let type_ref = variable.type_ref.as_ref().unwrap().clone();
  Ok(match type_ref.as_str() {
    "string" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        println!("BBB: context={:?}", ctx);
        println!("BBB: get entry={:?}", ctx.get_entry(&variable_name));
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::String(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "number" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::Number(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "boolean" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::Boolean(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "date" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::Date(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "time" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::Time(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "dateTime" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::DateTime(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "dayTimeDuration" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::DaysAndTimeDuration(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    "yearMonthDuration" => Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(v) = ctx.get_entry(&variable_name) {
          if let Value::YearsAndMonthsDuration(_) = v {
            return (variable_name.clone(), v.clone());
          }
        }
      }
      (variable_name.clone(), value_null!())
    }),
    _ => Box::new(move |value: &Value, item_definition_evaluator: &ItemDefinitionEvaluator| {
      if let Value::Context(ctx) = value {
        if let Some(entry_value) = ctx.get_entry(&variable_name) {
          let evaluated_value = item_definition_evaluator
            .eval(&type_ref, entry_value)
            .unwrap_or_else(|| value_null!("input data evaluator: item definition evaluator '{}' not found", type_ref));
          (variable_name.clone(), evaluated_value)
        } else {
          (variable_name.clone(), value_null!("no name {} in context {}", variable_name, ctx))
        }
      } else {
        (variable_name.clone(), value_null!("expected context, actual value is {}", value))
      }
    }),
  })
}

///
fn build_expression_instance_evaluator(scope: &Scope, expression_instance: &ExpressionInstance) -> Result<Evaluator> {
  match expression_instance {
    ExpressionInstance::Context(context) => build_context_evaluator(scope, context),
    ExpressionInstance::DecisionTable(decision_table) => build_decision_table_evaluator(scope, decision_table),
    ExpressionInstance::FunctionDefinition(function_definition) => build_function_definition_evaluator(scope, function_definition),
    ExpressionInstance::Invocation(invocation) => build_invocation_evaluator(scope, invocation),
    ExpressionInstance::LiteralExpression(literal_expression) => crate::builders::build_literal_expression_evaluator(scope, literal_expression),
    ExpressionInstance::Relation(relation) => build_relation_evaluator(scope, relation),
  }
}

///
fn build_context_evaluator(scope: &Scope, context: &Context) -> Result<Evaluator> {
  let mut entry_evaluators = vec![];
  scope.push(FeelContext::default());
  for context_entry in context.context_entries() {
    if let Some(variable) = &context_entry.variable {
      let name = variable.feel_name().as_ref().ok_or_else(err_empty_feel_name)?;
      let evaluator = build_expression_instance_evaluator(scope, &context_entry.value)?;
      scope.insert_null(name.clone());
      entry_evaluators.push((Some(name.clone()), evaluator));
    } else {
      let evaluator = build_expression_instance_evaluator(scope, &context_entry.value)?;
      entry_evaluators.push((None, evaluator));
    }
  }
  scope.pop();
  Ok(Box::new(move |scope: &Scope| {
    let mut evaluated_context = FeelContext::default();
    for (opt_name, evaluator) in &entry_evaluators {
      match opt_name {
        Some(name) => {
          let value = evaluator(scope);
          scope.set_entry(name, value.clone());
          evaluated_context.set_entry(name, value);
        }
        None => {
          return evaluator(scope);
        }
      }
    }
    Value::Context(evaluated_context)
  }))
}

///
fn build_decision_table_evaluator(scope: &Scope, decision_table: &DecisionTable) -> Result<Evaluator> {
  let decision_table_evaluator = decision_table::build_decision_table_evaluator(scope, decision_table)?;
  Ok(Box::new(move |scope: &Scope| decision_table_evaluator(scope)))
}

///
fn build_function_definition_evaluator(scope: &Scope, function_definition: &FunctionDefinition) -> Result<Evaluator> {
  let mut parameters = vec![];
  let body = function_definition.body().as_ref().ok_or_else(err_empty_function_body)?;
  let function_evaluator = build_expression_instance_evaluator(scope, body)?;
  for parameter in function_definition.formal_parameters() {
    let name = parameter.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
    let value_expression = parameter.value_expression().as_ref().ok_or_else(err_empty_value_expression)?;
    let evaluator = build_expression_instance_evaluator(scope, value_expression)?;
    parameters.push((name, evaluator));
  }
  Ok(Box::new(move |scope: &Scope| {
    let mut params_ctx = FeelContext::default();
    parameters.iter().for_each(|(name, evaluator)| params_ctx.set_entry(name, evaluator(scope)));
    if let Value::FunctionDefinition(_, body, result_type) = function_evaluator(scope) {
      scope.push(params_ctx);
      let value = body.evaluate(scope);
      scope.pop();
      result_type.coerced(&value)
    } else {
      value_null!("expected Value::FunctionDefinition in function definition evaluator")
    }
  }))
}

///
fn build_invocation_evaluator(scope: &Scope, invocation: &Invocation) -> Result<Evaluator> {
  let mut bindings = vec![];
  let function_evaluator = build_expression_instance_evaluator(scope, invocation.called_function())?;
  for binding in invocation.bindings() {
    if let Some(binding_formula) = binding.binding_formula() {
      let name = binding.parameter().feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
      let evaluator = build_expression_instance_evaluator(scope, binding_formula)?;
      bindings.push((name, evaluator));
    }
  }
  Ok(Box::new(move |scope: &Scope| {
    let mut params_ctx = FeelContext::default();
    bindings.iter().for_each(|(name, evaluator)| params_ctx.set_entry(name, evaluator(scope)));
    if let Value::FunctionDefinition(_, body, result_type) = function_evaluator(scope) {
      scope.push(params_ctx);
      let value = body.evaluate(scope);
      scope.pop();
      result_type.coerced(&value)
    } else {
      value_null!("expected Value::FunctionDefinition in invocation evaluator")
    }
  }))
}

///
fn build_literal_expression_evaluator(scope: &Scope, literal_expression: &LiteralExpression) -> Result<Evaluator> {
  let text = literal_expression.text().as_ref().ok_or_else(err_empty_literal_expression)?;
  let node = dmntk_feel_parser::parse_expression(scope, text, false)?;
  dmntk_feel_evaluator::prepare(&node)
}

///
fn build_relation_evaluator(scope: &Scope, relation: &Relation) -> Result<Evaluator> {
  let mut rows = vec![];
  for row in relation.rows() {
    let mut evaluators = vec![];
    for (i, element) in row.elements().iter().enumerate() {
      if let Some(column) = relation.columns().get(i) {
        let name = column.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone();
        let evaluator = build_expression_instance_evaluator(scope, element)?;
        evaluators.push((name, evaluator));
      }
    }
    rows.push(evaluators);
  }
  Ok(Box::new(move |scope: &Scope| {
    let mut results = vec![];
    for row in &rows {
      let mut evaluated_context = FeelContext::default();
      for (name, evaluator) in row {
        evaluated_context.set_entry(name, evaluator(scope));
      }
      results.push(Value::Context(evaluated_context));
    }
    Value::List(Values::new(results))
  }))
}

#[cfg(test)]
mod tests {
  use crate::builders::type_ref_to_feel_type;
  use dmntk_feel::FeelType;

  #[test]
  fn test_type_ref_to_feel_type() {
    assert_eq!(Some(FeelType::String), type_ref_to_feel_type("string"));
    assert_eq!(Some(FeelType::Number), type_ref_to_feel_type("number"));
    assert_eq!(Some(FeelType::Boolean), type_ref_to_feel_type("boolean"));
    assert_eq!(Some(FeelType::Date), type_ref_to_feel_type("date"));
    assert_eq!(Some(FeelType::Time), type_ref_to_feel_type("time"));
    assert_eq!(Some(FeelType::DateTime), type_ref_to_feel_type("dateTime"));
    assert_eq!(Some(FeelType::DaysAndTimeDuration), type_ref_to_feel_type("dayTimeDuration"));
    assert_eq!(Some(FeelType::YearsAndMonthsDuration), type_ref_to_feel_type("yearMonthDuration"));
    assert_eq!(None, type_ref_to_feel_type("text"));
  }
}
