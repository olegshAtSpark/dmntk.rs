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

//! Builder for item definition evaluators.

use crate::errors::{err_empty_feel_name, err_unsupported_feel_type};
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::{Value, Values};
use dmntk_feel::{value_null, AstNode, Evaluator, FeelType, Name, Scope};
use dmntk_model::model::{Definitions, ItemDefinition, ItemDefinitionType, NamedElement};
use std::collections::HashMap;

/// Type of closure that evaluates input data conformant with item definition.
type ItemDefinitionEvaluatorFn = Box<dyn Fn(&Value, &ItemDefinitionEvaluator) -> Value + Send + Sync>;

/// Item definition evaluator.
#[derive(Default)]
pub struct ItemDefinitionEvaluator {
  evaluators: HashMap<String, ItemDefinitionEvaluatorFn>,
}

impl ItemDefinitionEvaluator {
  /// Creates new item definition evaluator.
  pub fn build(&mut self, definitions: &Definitions) -> Result<()> {
    for item_definition in definitions.item_definitions() {
      let evaluator = build_item_definition_evaluator(item_definition)?;
      let type_ref = item_definition.name().to_string();
      self.evaluators.insert(type_ref, evaluator);
    }
    Ok(())
  }
  /// Evaluates item definition with specified type reference name.
  pub fn eval(&self, type_ref: &str, value: &Value) -> Option<Value> {
    self.evaluators.get(type_ref).map(|evaluator| evaluator(value, self))
  }
  /// Returns a reference to item definition with specified type reference name.
  pub fn get(&self, type_ref: &str) -> Option<&ItemDefinitionEvaluatorFn> {
    self.evaluators.get(type_ref)
  }
}

///
pub fn build_item_definition_evaluator(item_definition: &ItemDefinition) -> Result<ItemDefinitionEvaluatorFn> {
  // prepare optional allowed values evaluator
  let av_evaluator = build_allowed_values_evaluator(item_definition)?;
  // build item definition evaluator
  match super::item_definition_type(item_definition)? {
    ItemDefinitionType::SimpleType(feel_type) => build_simple_type_evaluator(feel_type, av_evaluator),
    ItemDefinitionType::ReferencedType(ref_type) => build_referenced_type_evaluator(ref_type),
    ItemDefinitionType::ComponentType => build_component_type_evaluator(item_definition),
    ItemDefinitionType::CollectionOfSimpleType(feel_type) => build_collection_of_simple_type_evaluator(feel_type, av_evaluator),
    ItemDefinitionType::CollectionOfReferencedType(ref_type) => build_collection_of_referenced_type_evaluator(ref_type, av_evaluator),
    ItemDefinitionType::CollectionOfComponentType => build_collection_of_component_type_evaluator(item_definition),
  }
}

///
fn build_allowed_values_evaluator(item_definition: &ItemDefinition) -> Result<Option<Evaluator>> {
  let mut av_evaluator = None;
  if let Some(unary_tests) = item_definition.allowed_values() {
    if let Some(text) = unary_tests.text() {
      let scope = Scope::default();
      let unary_tests_node = dmntk_feel_parser::parse_unary_tests(&scope, text, false)?;
      let node = AstNode::In(Box::new(AstNode::Name("?".into())), Box::new(unary_tests_node));
      av_evaluator = Some(dmntk_feel_evaluator::prepare(&node)?);
    }
  }
  Ok(av_evaluator)
}

///
fn check_allowed_values(value: Value, av_evaluator: Option<&Evaluator>) -> Value {
  if let Some(evaluator) = av_evaluator {
    let scope = Scope::default();
    scope.set_entry(&"?".into(), value.clone());
    if evaluator(&scope).is_true() {
      value
    } else {
      value_null!("value not allowed")
    }
  } else {
    value
  }
}

///
fn build_simple_type_evaluator(feel_type: FeelType, av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
  ///
  fn build_string_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::String(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!("expected type 'string', actual type is '{}' in value '{}'", value.type_of(), value)
      }
    })
  }
  ///
  fn build_number_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Number(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!("expected type 'number', actual type is '{}' in value '{}'", value.type_of(), value)
      }
    })
  }
  ///
  fn build_boolean_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Boolean(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!("expected type 'boolean', actual type is '{}' in value '{}'", value.type_of(), value)
      }
    })
  }
  ///
  fn build_date_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Date(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!("expected type 'date', actual type is '{}' in value '{}'", value.type_of(), value)
      }
    })
  }
  ///
  fn build_time_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::Time(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!("expected type 'time', actual type is '{}' in value '{}'", value.type_of(), value)
      }
    })
  }
  ///
  fn build_date_time_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::DateTime(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!("expected type 'date and time', actual type is '{}' in value '{}'", value.type_of(), value)
      }
    })
  }
  ///
  fn build_dt_duration_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::DaysAndTimeDuration(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!(
          "expected type 'days and time duration', actual type is '{}' in value '{}'",
          value.type_of(),
          value
        )
      }
    })
  }
  ///
  fn build_ym_duration_evaluator(av_evaluator: Option<Evaluator>) -> ItemDefinitionEvaluatorFn {
    Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::YearsAndMonthsDuration(_) = value {
        check_allowed_values(value.to_owned(), av_evaluator.as_ref())
      } else {
        value_null!(
          "expected type 'years and months duration', actual type is '{}' in value '{}'",
          value.type_of(),
          value
        )
      }
    })
  }
  match feel_type {
    FeelType::String => Ok(build_string_evaluator(av_evaluator)),
    FeelType::Number => Ok(build_number_evaluator(av_evaluator)),
    FeelType::Boolean => Ok(build_boolean_evaluator(av_evaluator)),
    FeelType::Date => Ok(build_date_evaluator(av_evaluator)),
    FeelType::Time => Ok(build_time_evaluator(av_evaluator)),
    FeelType::DateTime => Ok(build_date_time_evaluator(av_evaluator)),
    FeelType::DaysAndTimeDuration => Ok(build_dt_duration_evaluator(av_evaluator)),
    FeelType::YearsAndMonthsDuration => Ok(build_ym_duration_evaluator(av_evaluator)),
    _ => Err(err_unsupported_feel_type(feel_type)),
  }
}

///
fn build_referenced_type_evaluator(ref_type: String) -> Result<ItemDefinitionEvaluatorFn> {
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    evaluators.eval(&ref_type, value).unwrap_or_else(|| value_null!("no evaluator"))
  }))
}

///
fn build_component_type_evaluator(item_definition: &ItemDefinition) -> Result<ItemDefinitionEvaluatorFn> {
  let mut component_evaluators: Vec<(Name, ItemDefinitionEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    component_evaluators.push((
      component_item_definition.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone(),
      build_item_definition_evaluator(component_item_definition)?,
    ));
  }
  let av_evaluator = build_allowed_values_evaluator(item_definition)?;
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    if let Value::Context(ctx) = value {
      let mut evaluated_ctx = FeelContext::default();
      for (component_name, component_evaluator) in &component_evaluators {
        if let Some(component_value) = ctx.get_entry(component_name) {
          evaluated_ctx.set_entry(component_name, component_evaluator(component_value, evaluators));
        } else {
          return value_null!("item definition evaluator (Component): name not found: {} in context: {}", component_name, ctx);
        }
      }
      check_allowed_values(Value::Context(evaluated_ctx), av_evaluator.as_ref())
    } else {
      value_null!("item definition evaluator (Component): expected context value, actual value is: {}", value)
    }
  }))
}

///
fn build_collection_of_simple_type_evaluator(feel_type: FeelType, av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
  ///
  fn build_string_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::String(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected string");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  ///
  fn build_number_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::Number(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected number");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  ///
  fn build_boolean_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::Boolean(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected boolean");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  ///
  fn build_date_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::Date(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected date");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  ///
  fn build_time_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::Time(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected time");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  ///
  fn build_date_and_time_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::DateTime(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected date and time");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  ///
  fn build_days_and_time_duration_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::DaysAndTimeDuration(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected days and time duration");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  ///
  fn build_months_and_years_duration_evaluator(av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
    Ok(Box::new(move |value: &Value, _: &ItemDefinitionEvaluator| {
      if let Value::List(values) = value {
        let mut evaluated_values = Values::default();
        for item_value in values.as_vec() {
          if let Value::YearsAndMonthsDuration(_) = item_value {
            evaluated_values.add(item_value.clone());
          } else {
            return value_null!("item definition evaluator (CollectionOfSimpleType): expected months and years duration");
          }
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("item definition evaluator (CollectionOfSimpleType): expected list")
      }
    }))
  }
  // build evaluator based on FEEL type
  match feel_type {
    FeelType::String => build_string_evaluator(av_evaluator),
    FeelType::Number => build_number_evaluator(av_evaluator),
    FeelType::Boolean => build_boolean_evaluator(av_evaluator),
    FeelType::Date => build_date_evaluator(av_evaluator),
    FeelType::Time => build_time_evaluator(av_evaluator),
    FeelType::DateTime => build_date_and_time_evaluator(av_evaluator),
    FeelType::DaysAndTimeDuration => build_days_and_time_duration_evaluator(av_evaluator),
    FeelType::YearsAndMonthsDuration => build_months_and_years_duration_evaluator(av_evaluator),
    _ => Err(err_unsupported_feel_type(feel_type)),
  }
}

///
fn build_collection_of_referenced_type_evaluator(type_ref: String, av_evaluator: Option<Evaluator>) -> Result<ItemDefinitionEvaluatorFn> {
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    if let Value::List(values) = value {
      let mut evaluated_values = Values::default();
      if let Some(evaluator) = evaluators.get(&type_ref) {
        for item_value in values.as_vec() {
          evaluated_values.add(evaluator(item_value, evaluators));
        }
        check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
      } else {
        value_null!("no evaluator defined for type reference '{}'", type_ref)
      }
    } else {
      value_null!("expected list, actual type is '{}' in value '{}'", value.type_of(), value)
    }
  }))
}

///
fn build_collection_of_component_type_evaluator(item_definition: &ItemDefinition) -> Result<ItemDefinitionEvaluatorFn> {
  let mut component_evaluators: Vec<(Name, ItemDefinitionEvaluatorFn)> = vec![];
  for component_item_definition in item_definition.item_components() {
    component_evaluators.push((
      component_item_definition.feel_name().as_ref().ok_or_else(err_empty_feel_name)?.clone(),
      build_item_definition_evaluator(component_item_definition)?,
    ));
  }
  let av_evaluator = build_allowed_values_evaluator(item_definition)?;
  Ok(Box::new(move |value: &Value, evaluators: &ItemDefinitionEvaluator| {
    if let Value::List(values) = value {
      let mut evaluated_values = Values::default();
      for item_value in values.as_vec() {
        if let Value::Context(ctx) = item_value {
          let mut evaluated_ctx = FeelContext::default();
          for (component_name, component_evaluator) in &component_evaluators {
            if let Some(component_value) = ctx.get_entry(component_name) {
              evaluated_ctx.set_entry(component_name, component_evaluator(component_value, evaluators));
            } else {
              return value_null!("name '{}' not found in context '{}'", component_name, ctx);
            }
          }
          evaluated_values.add(Value::Context(evaluated_ctx))
        } else {
          return value_null!("expected context, actual type is '{}' in value '{}'", item_value.type_of(), item_value);
        }
      }
      check_allowed_values(Value::List(evaluated_values), av_evaluator.as_ref())
    } else {
      value_null!("expected list, actual type is '{}' in value '{}'", value.type_of(), value)
    }
  }))
}

#[cfg(test)]
mod tests {
  use crate::builders::ItemDefinitionEvaluator;
  use dmntk_examples::item_definition::*;
  use dmntk_feel::context::FeelContext;
  use dmntk_feel::values::{Value, Values};
  use dmntk_feel::{value_null, value_number, FeelDate, FeelDateTime, FeelDaysAndTimeDuration, FeelNumber, FeelTime, FeelYearsAndMonthsDuration, Name};

  /// Utility function for building item definition evaluator from definitions.
  fn build_evaluator(xml: &str) -> ItemDefinitionEvaluator {
    let mut evaluator = ItemDefinitionEvaluator::default();
    evaluator.build(&dmntk_model::parse(xml).unwrap()).unwrap();
    evaluator
  }

  #[test]
  fn test_evaluate_input_data_0101_1() {
    let evaluator = build_evaluator(DMN_0101);
    let context_str = r#"{ Customer Name : "Whistler" }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Customer", "Name"])).unwrap();
    assert_eq!(Value::String("Whistler".to_string()), evaluator.eval("tCustomerName", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0101_2() {
    let evaluator = build_evaluator(DMN_0101);
    let context_str = r#"{ Customer Name : 12000 }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Customer", "Name"])).unwrap();
    assert_eq!(
      value_null!("expected type 'string', actual type is 'number' in value '12000'"),
      evaluator.eval("tCustomerName", value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0102_1() {
    let evaluator = build_evaluator(DMN_0102);
    let context_str = r#"{ Monthly Salary : 12000.00 }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Monthly", "Salary"])).unwrap();
    assert_eq!(value_number!(12_000), evaluator.eval("tMonthlySalary", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0102_2() {
    let evaluator = build_evaluator(DMN_0102);
    let context_str = r#"{ Monthly Salary : true }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Monthly", "Salary"])).unwrap();
    assert_eq!(
      value_null!("expected type 'number', actual type is 'boolean' in value 'true'"),
      evaluator.eval("tMonthlySalary", value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0103_1() {
    let evaluator = build_evaluator(DMN_0103);
    let context_str = r#"{ Is Affordable : true }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Is", "Affordable"])).unwrap();
    assert_eq!(Value::Boolean(true), evaluator.eval("tIsAffordable", value).unwrap());
    let context_str = r#"{ Is Affordable : false }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Is", "Affordable"])).unwrap();
    assert_eq!(Value::Boolean(false), evaluator.eval("tIsAffordable", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0103_2() {
    let evaluator = build_evaluator(DMN_0103);
    let context_str = r#"{ Is Affordable : "Yes" }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Is", "Affordable"])).unwrap();
    assert_eq!(
      value_null!(r#"expected type 'boolean', actual type is 'string' in value '"Yes"'"#),
      evaluator.eval("tIsAffordable", value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0104_1() {
    let evaluator = build_evaluator(DMN_0104);
    let context_str = r#"{ Birthday : date("1982-04-12") }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Birthday"])).unwrap();
    assert_eq!(Value::Date(FeelDate::new(1982, 4, 12)), evaluator.eval("tBirthday", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0105_1() {
    let evaluator = build_evaluator(DMN_0105);
    let context_str = r#"{ Delivery Time : time("18:35:23") }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Delivery", "Time"])).unwrap();
    assert_eq!(
      Value::Time(FeelTime::new_hms_opt(18, 35, 23, 0).unwrap()),
      evaluator.eval("tDeliveryTime", value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0106_1() {
    let evaluator = build_evaluator(DMN_0106);
    let context_str = r#"{ Appointment : date and time("2021-10-12T18:35:23") }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Appointment"])).unwrap();
    assert_eq!(
      Value::DateTime(FeelDateTime::new(FeelDate::new(2021, 10, 12), FeelTime::new_hms_opt(18, 35, 23, 0).unwrap())),
      evaluator.eval("tAppointment", value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0107_1() {
    let evaluator = build_evaluator(DMN_0107);
    let context_str = r#"{ Course Duration : duration("P2DT3H") }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Course", "Duration"])).unwrap();
    assert_eq!(
      Value::DaysAndTimeDuration(FeelDaysAndTimeDuration::default().second(183600).build()),
      evaluator.eval("tCourseDuration", value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0108_1() {
    let evaluator = build_evaluator(DMN_0108);
    let context_str = r#"{ Growth Duration : duration("P2Y5M") }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Growth", "Duration"])).unwrap();
    assert_eq!(
      Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::new_ym(2, 5)),
      evaluator.eval("tGrowthDuration", value).unwrap()
    );
  }

  #[test]
  fn test_evaluate_input_data_0201_1() {
    let evaluator = build_evaluator(DMN_0201);
    let context_str = r#"{ Customer Name : "Bloomberg" }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Customer", "Name"])).unwrap();
    assert_eq!(Value::String("Bloomberg".to_string()), evaluator.eval("tCustomerName", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0202_1() {
    let evaluator = build_evaluator(DMN_0202);
    let context_str = r#"{ Monthly Salary : 12000.00 }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Monthly", "Salary"])).unwrap();
    assert_eq!(value_number!(12_000), evaluator.eval("tMonthlySalary", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0301_1() {
    let evaluator = build_evaluator(DMN_0301);
    let context_str = r#"{ Loan : { principal: 10, rate: 60, termMonths: 28 } }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Loan"])).unwrap();
    let mut ctx = FeelContext::default();
    ctx.set_entry(&"principal".into(), value_number!(10));
    ctx.set_entry(&"rate".into(), value_number!(60));
    ctx.set_entry(&"termMonths".into(), value_number!(28));
    let expected = Value::Context(ctx);
    assert_eq!(expected, evaluator.eval("tLoan", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0401_1() {
    let evaluator = build_evaluator(DMN_0401);
    let context_str = r#"{ Items : ["Mercury", "Venus", "Earth", "Mars"] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      Value::String("Mercury".to_string()),
      Value::String("Venus".to_string()),
      Value::String("Earth".to_string()),
      Value::String("Mars".to_string()),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0402_1() {
    let evaluator = build_evaluator(DMN_0402);
    let context_str = r#"{ Items : [9000.00, 10000.00, 11000.00, 12000.00, 13000.00] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      value_number!(9_000),
      value_number!(10_000),
      value_number!(11_000),
      value_number!(12_000),
      value_number!(13_000),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0403_1() {
    let evaluator = build_evaluator(DMN_0403);
    let context_str = r#"{ Items : [true, false, false, true, true] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      Value::Boolean(true),
      Value::Boolean(false),
      Value::Boolean(false),
      Value::Boolean(true),
      Value::Boolean(true),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0404_1() {
    let evaluator = build_evaluator(DMN_0404);
    let context_str = r#"{ Items : [date("2021-10-10"), date("2021-10-11"), date("2021-10-12")] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      Value::Date(FeelDate::new(2021, 10, 10)),
      Value::Date(FeelDate::new(2021, 10, 11)),
      Value::Date(FeelDate::new(2021, 10, 12)),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0405_1() {
    let evaluator = build_evaluator(DMN_0405);
    let context_str = r#"{ Items : [time("12:21:35"), time("12:21:36"), time("12:21:37")] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      Value::Time(FeelTime::new_hms_opt(12, 21, 35, 0).unwrap()),
      Value::Time(FeelTime::new_hms_opt(12, 21, 36, 0).unwrap()),
      Value::Time(FeelTime::new_hms_opt(12, 21, 37, 0).unwrap()),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0406_1() {
    let evaluator = build_evaluator(DMN_0406);
    let context_str = r#"{ Items : [date and time("2021-10-10T21:23:18"), date and time("2021-10-11T12:18:59")] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      Value::DateTime(FeelDateTime::new(FeelDate::new(2021, 10, 10), FeelTime::new_hms_opt(21, 23, 18, 0).unwrap())),
      Value::DateTime(FeelDateTime::new(FeelDate::new(2021, 10, 11), FeelTime::new_hms_opt(12, 18, 59, 0).unwrap())),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0407_1() {
    let evaluator = build_evaluator(DMN_0407);
    let context_str = r#"{ Items : [duration("P2DT3H")] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![Value::DaysAndTimeDuration(
      FeelDaysAndTimeDuration::default().second(183600).build(),
    )]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0408_1() {
    let evaluator = build_evaluator(DMN_0408);
    let context_str = r#"{ Items : [duration("P2Y3M"), duration("P2Y4M")] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::new_ym(2, 3)),
      Value::YearsAndMonthsDuration(FeelYearsAndMonthsDuration::new_ym(2, 4)),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0501_1() {
    let evaluator = build_evaluator(DMN_0501);
    let context_str = r#"{ Items : ["Mercury", "Venus", "Earth"] }"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let expected = Value::List(Values::new(vec![
      Value::String("Mercury".to_string()),
      Value::String("Venus".to_string()),
      Value::String("Earth".to_string()),
    ]));
    assert_eq!(expected, evaluator.eval("tItems", value).unwrap());
  }

  #[test]
  fn test_evaluate_input_data_0601_1() {
    let evaluator = build_evaluator(DMN_0601);
    let context_str = r#"{Items:[{number:1,name:"One",manager:"John"},{number:2,name:"Two",manager:"Mike"},{number:3,name:"Three",manager:"Bob"}]}"#;
    let context = dmntk_feel_evaluator::evaluate_context(&Default::default(), context_str).unwrap();
    let value = context.get_entry(&Name::new(&["Items"])).unwrap();
    let mut ctx_1 = FeelContext::default();
    ctx_1.set_entry(&"number".into(), value_number!(1));
    ctx_1.set_entry(&"name".into(), Value::String("One".to_string()));
    ctx_1.set_entry(&"manager".into(), Value::String("John".to_string()));
    let mut ctx_2 = FeelContext::default();
    ctx_2.set_entry(&"number".into(), value_number!(2));
    ctx_2.set_entry(&"name".into(), Value::String("Two".to_string()));
    ctx_2.set_entry(&"manager".into(), Value::String("Mike".to_string()));
    let mut ctx_3 = FeelContext::default();
    ctx_3.set_entry(&"number".into(), value_number!(3));
    ctx_3.set_entry(&"name".into(), Value::String("Three".to_string()));
    ctx_3.set_entry(&"manager".into(), Value::String("Bob".to_string()));
    let expected = Value::List(Values::new(vec![Value::Context(ctx_1), Value::Context(ctx_2), Value::Context(ctx_3)]));
    assert_eq!(Some(expected), evaluator.eval("tItems", value));
  }
}
