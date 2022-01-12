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

//! `FEEL` types.

use self::errors::*;
use crate::context::FeelContext;
use crate::names::Name;
use crate::value_null;
use crate::values::{Value, Values};
use dmntk_common::{DmntkError, Result};
use std::collections::BTreeMap;
use std::str::FromStr;

pub const FEEL_TYPE_NAME_ANY: &str = "Any";
pub const FEEL_TYPE_NAME_BOOLEAN: &str = "boolean";
pub const FEEL_TYPE_NAME_DATE: &str = "date";
pub const FEEL_TYPE_NAME_DATE_AND_TIME: &str = "date and time";
pub const FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION: &str = "days and time duration";
pub const FEEL_TYPE_NAME_NULL: &str = "Null";
pub const FEEL_TYPE_NAME_NUMBER: &str = "number";
pub const FEEL_TYPE_NAME_STRING: &str = "string";
pub const FEEL_TYPE_NAME_TIME: &str = "time";
pub const FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION: &str = "years and months duration";

#[derive(Debug, Clone, PartialEq)]
pub enum FeelType {
  /// Type representing any valid `FEEL` type.
  Any,
  /// Type representing a `boolean` value.
  Boolean,
  /// Type representing a `context` value.
  Context(
    /// Types of context entries.
    BTreeMap<Name, FeelType>,
  ),
  /// Type representing a `date` value.
  Date,
  /// Type representing a`date and time` value.
  DateTime,
  /// Type representing a `days and time duration` value.
  DaysAndTimeDuration,
  /// Type representing a `function` value.
  Function(
    /// List of types of the function's parameters.
    Vec<FeelType>,
    /// Type of the function's result.
    Box<FeelType>,
  ),
  /// Type representing a `list` of values.
  List(Box<FeelType>),
  /// Type representing a `null` value.
  Null,
  /// Type representing a `number` value.
  Number,
  /// Type representing a `range` values.
  Range(Box<FeelType>),
  /// Type representing a `string` value.
  String,
  /// Type representing a `time` value.
  Time,
  /// Type representing a `years and months duration` value.
  YearsAndMonthsDuration,
}

impl std::fmt::Display for FeelType {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      FeelType::Any => write!(f, "{}", FEEL_TYPE_NAME_ANY),
      FeelType::Boolean => write!(f, "{}", FEEL_TYPE_NAME_BOOLEAN),
      FeelType::Context(entries) => {
        let entries_str = entries
          .iter()
          .map(|(entry_name, entry_type)| format!("{}: {}", entry_name, entry_type))
          .collect::<Vec<String>>()
          .join(", ");
        write!(f, "context<{}>", entries_str)
      }
      FeelType::Date => write!(f, "{}", FEEL_TYPE_NAME_DATE),
      FeelType::DateTime => write!(f, "{}", FEEL_TYPE_NAME_DATE_AND_TIME),
      FeelType::DaysAndTimeDuration => write!(f, "{}", FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION),
      FeelType::Function(parameter_types, result_type) => {
        let parameter_types_str = parameter_types
          .iter()
          .map(|parameter_type| format!("{}", parameter_type))
          .collect::<Vec<String>>()
          .join(", ");
        let result_type_str = result_type.to_string();
        write!(f, "function<{}>->{}", parameter_types_str, result_type_str)
      }
      FeelType::List(item_type) => {
        write!(f, "list<{}>", item_type)
      }
      FeelType::Null => write!(f, "{}", FEEL_TYPE_NAME_NULL),
      FeelType::Number => write!(f, "{}", FEEL_TYPE_NAME_NUMBER),
      FeelType::Range(range_type) => {
        write!(f, "range<{}>", range_type)
      }
      FeelType::String => write!(f, "{}", FEEL_TYPE_NAME_STRING),
      FeelType::Time => write!(f, "{}", FEEL_TYPE_NAME_TIME),
      FeelType::YearsAndMonthsDuration => write!(f, "{}", FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION),
    }
  }
}

impl FromStr for FeelType {
  type Err = DmntkError;
  /// Converts a string to built-in type.
  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      FEEL_TYPE_NAME_ANY => Ok(Self::Any),
      FEEL_TYPE_NAME_BOOLEAN => Ok(Self::Boolean),
      FEEL_TYPE_NAME_DATE => Ok(Self::Date),
      FEEL_TYPE_NAME_DATE_AND_TIME => Ok(Self::DateTime),
      FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION => Ok(Self::DaysAndTimeDuration),
      FEEL_TYPE_NAME_NULL => Ok(Self::Null),
      FEEL_TYPE_NAME_NUMBER => Ok(Self::Number),
      FEEL_TYPE_NAME_STRING => Ok(Self::String),
      FEEL_TYPE_NAME_TIME => Ok(Self::Time),
      FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION => Ok(Self::YearsAndMonthsDuration),
      _ => Err(invalid_feel_type_name(s)),
    }
  }
}

impl From<&Name> for FeelType {
  /// Converts a `FEEL` name to built-in type.
  fn from(name: &Name) -> Self {
    if let Ok(feel_type) = Self::from_str(name.to_string().as_str()) {
      feel_type
    } else {
      FeelType::Any
    }
  }
}

/// Returns `true` when the specified type name is a built-in type.
pub fn is_built_in_type_name(name: &str) -> bool {
  matches!(
    name,
    FEEL_TYPE_NAME_ANY
      | FEEL_TYPE_NAME_BOOLEAN
      | FEEL_TYPE_NAME_DATE
      | FEEL_TYPE_NAME_DATE_AND_TIME
      | FEEL_TYPE_NAME_DAYS_AND_TIME_DURATION
      | FEEL_TYPE_NAME_NULL
      | FEEL_TYPE_NAME_NUMBER
      | FEEL_TYPE_NAME_STRING
      | FEEL_TYPE_NAME_TIME
      | FEEL_TYPE_NAME_YEARS_AND_MONTHS_DURATION
  )
}

impl FeelType {
  /// When a value appears in a certain context, it must be compatible
  /// with a type expected in that context, called the target type.
  /// After the type of the value is known, an implicit conversion
  /// from the type of the value to the target type can be performed.
  /// If an implicit conversion is mandatory but it cannot be performed,
  /// the result is null.
  ///
  /// There are several possible type conversions:
  ///
  /// - to singleton list:
  ///
  ///      When the type of the value is `T` and the target type is `List<T>`,
  ///      the simple value is converted to a singleton list.
  ///
  /// - from singleton list:
  ///
  ///      When the type of the value is List<T>, and the value is a singleton list
  ///      and the target type is T, the value is converted by unwrapping the first element.
  ///
  /// - conforms to:
  ///
  ///      When the type of the value is T1, the target type is T2, and T1 conforms to T2,
  ///      the value remains unchanged. Otherwise the result is null.
  ///
  /// All these conversion rules are implemented in this function.
  ///
  pub fn coerced(&self, actual_value: &Value) -> Value {
    // conforms to
    if actual_value.type_of().is_conformant(self) {
      return actual_value.clone();
    }
    match self {
      // to singleton list
      FeelType::List(target_type) => {
        if actual_value.type_of().is_conformant(target_type) {
          return Value::List(Values::new(vec![actual_value.clone()]));
        }
      }
      // from singleton list
      target_type => {
        if let FeelType::List(actual_type) = actual_value.type_of() {
          if actual_type.is_conformant(target_type) {
            if let Value::List(values) = actual_value {
              if values.len() == 1 {
                return values.as_vec()[0].clone();
              }
            }
          }
        }
      }
    }
    value_null!()
  }
  ///
  pub fn get_conformant_value(&self, actual_value: &Value) -> Value {
    let actual_type = actual_value.type_of();
    if actual_type.is_conformant(self) {
      if let Ok(expected_value) = self.get_value_checked(actual_value) {
        expected_value
      } else {
        value_null!("actual value {} does not match expected type {}", actual_value, self.to_string())
      }
    } else {
      value_null!(
        "expected type {} is not conformant with actual type {}",
        self.to_string(),
        actual_type.to_string()
      )
    }
  }
  /// Returns a new value cloned from provided value, and retrieved with type checking.
  pub fn get_value_checked(&self, value: &Value) -> Result<Value> {
    if let Value::Null(_) = value {
      // `null` value is conformant with all types
      return Ok(value_null!());
    }
    match self {
      FeelType::Any => {
        return Ok(value.clone());
      }
      FeelType::Boolean => {
        if let Value::Boolean(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Context(entries) => {
        if let Value::Context(context) = value {
          let mut result = FeelContext::default();
          for (name, entry_type) in entries {
            if let Some(entry_value) = context.get_entry(name) {
              result.set_entry(name, entry_type.get_value_checked(entry_value)?);
            }
          }
          return Ok(Value::Context(result));
        }
      }
      FeelType::Date => {
        if let Value::Date(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::DateTime => {
        if let Value::DateTime(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::DaysAndTimeDuration => {
        if let Value::DaysAndTimeDuration(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Function(_, _) => {
        if let Value::FunctionDefinition { .. } = value {
          return Ok(value.clone());
        }
      }
      FeelType::List(items_type) => {
        if let Value::List(items) = value {
          let mut result = vec![];
          for item in items.as_vec() {
            result.push(items_type.get_value_checked(item)?);
          }
          return Ok(Value::List(Values::new(result)));
        }
      }
      FeelType::Null => {
        if let Value::Null(_) = value {
          return Ok(value_null!());
        }
      }
      FeelType::Number => {
        if let Value::Number(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Range(_) => {
        if let Value::Range(_, _, _, _) = value {
          return Ok(value.clone());
        }
      }
      FeelType::String => {
        if let Value::String(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::Time => {
        if let Value::Time(_) = value {
          return Ok(value.clone());
        }
      }
      FeelType::YearsAndMonthsDuration => {
        if let Value::YearsAndMonthsDuration(_) = value {
          return Ok(value.clone());
        }
      }
    }
    Err(invalid_value_for_retrieving_using_feel_type(&self.to_string(), &value.to_string()))
  }

  /// Returns `true` when this type is a simple `FEEL` type.
  pub fn is_simple_built_in_type(&self) -> bool {
    matches!(
      self,
      Self::Any
        | Self::Boolean
        | Self::Date
        | Self::DateTime
        | Self::DaysAndTimeDuration
        | Self::Null
        | Self::Number
        | Self::String
        | Self::Time
        | Self::YearsAndMonthsDuration
    )
  }
  /// Creates a `list` type with specified items' type.
  pub fn list(items_type: &FeelType) -> FeelType {
    FeelType::List(Box::new(items_type.clone()))
  }
  /// Creates a `range` type with specified elements' type.
  pub fn range(elements_type: &FeelType) -> FeelType {
    FeelType::Range(Box::new(elements_type.clone()))
  }
  /// Creates a `context` type with specified entries.
  pub fn context(entries_types: &[(&Name, &FeelType)]) -> FeelType {
    FeelType::Context(entries_types.iter().map(|(name, typ)| ((*name).clone(), (*typ).clone())).collect())
  }
  /// Creates a `function` type with specified parameter types and result type.
  pub fn function(parameter_types: &[FeelType], result_type: &FeelType) -> FeelType {
    FeelType::Function(parameter_types.iter().map(|typ| (*typ).clone()).collect(), Box::new((*result_type).clone()))
  }
  ///
  pub fn zip(&self, other: &FeelType) -> FeelType {
    if self == other {
      self.clone()
    } else {
      FeelType::Any
    }
  }
  ///
  pub fn is_equivalent(&self, other: &FeelType) -> bool {
    return match other {
      FeelType::Any => matches!(self, FeelType::Any),
      FeelType::Boolean => matches!(self, FeelType::Boolean),
      FeelType::Context(entries_other) => {
        if let FeelType::Context(entries_self) = self {
          if entries_self.keys().len() == entries_other.len() {
            for (name, type_self) in entries_self {
              if let Some(type_other) = entries_other.get(name) {
                if !type_self.is_equivalent(type_other) {
                  return false;
                }
              } else {
                return false;
              }
            }
            return true;
          }
        }
        false
      }
      FeelType::Date => matches!(self, FeelType::Date),
      FeelType::DateTime => matches!(self, FeelType::DateTime),
      FeelType::DaysAndTimeDuration => matches!(self, FeelType::DaysAndTimeDuration),
      FeelType::Function(params_other, result_other) => {
        if let FeelType::Function(params_self, result_self) = self {
          if params_self.len() == params_other.len() {
            for (i, param_self) in params_self.iter().enumerate() {
              if !param_self.is_equivalent(&params_other[i]) {
                return false;
              }
              if !result_self.is_equivalent(result_other) {
                return false;
              }
            }
            return true;
          }
        }
        false
      }
      FeelType::List(type_other) => {
        if let FeelType::List(type_self) = self {
          type_self.is_equivalent(type_other)
        } else {
          false
        }
      }
      FeelType::Null => matches!(self, FeelType::Null),
      FeelType::Number => matches!(self, FeelType::Number),
      FeelType::Range(type_other) => {
        if let FeelType::Range(type_self) = self {
          type_self.is_equivalent(type_other)
        } else {
          false
        }
      }
      FeelType::String => matches!(self, FeelType::String),
      FeelType::Time => matches!(self, FeelType::Time),
      FeelType::YearsAndMonthsDuration => matches!(self, FeelType::YearsAndMonthsDuration),
    };
  }
  ///
  pub fn is_conformant(&self, other: &FeelType) -> bool {
    if self.is_equivalent(other) {
      return true;
    }
    if let FeelType::Null = self {
      return true;
    }
    if let FeelType::Any = other {
      return true;
    }
    match other {
      FeelType::List(type_other) => {
        if let FeelType::List(type_self) = self {
          return type_self.is_conformant(type_other);
        }
      }
      FeelType::Context(entries_other) => {
        if let FeelType::Context(entries_self) = self {
          for (name, type_other) in entries_other {
            if let Some(type_self) = entries_self.get(name) {
              if !type_self.is_conformant(type_other) {
                return false;
              }
            } else {
              return false;
            }
          }
          return true;
        }
        return false;
      }
      FeelType::Function(parameters_other, result_other) => {
        if let FeelType::Function(parameters_self, result_self) = self {
          if parameters_self.len() == parameters_other.len() {
            for (i, parameter_other) in parameters_other.iter().enumerate() {
              if !parameter_other.is_conformant(&parameters_self[i]) {
                return false;
              }
              if !result_self.is_conformant(result_other) {
                return false;
              }
            }
            return true;
          }
        }
        return false;
      }
      FeelType::Range(type_other) => {
        if let FeelType::Range(type_self) = self {
          return type_self.is_conformant(type_other);
        }
      }
      _ => {}
    }
    false
  }
}

/// Definitions of errors raised by `types` module.
mod errors {
  use dmntk_common::DmntkError;

  /// Definition of errors raised in `types` module.
  #[derive(Debug, PartialEq)]
  pub enum TypesError {
    /// Error raised when the specified text is not a valid `FEEL` type name.
    InvalidFeelTypeName(String),
    /// Error raised when the specified value is not valid for retrieving with type checking with `FEEL` type.
    InvalidValueForRetrievingUsingFeelType(String, String),
  }

  impl From<TypesError> for DmntkError {
    /// Converts `TypesError` into [DmntkError].
    fn from(e: TypesError) -> Self {
      DmntkError::new("TypesError", &e.to_string())
    }
  }

  impl std::fmt::Display for TypesError {
    /// Implements [Display](std::fmt::Display) trait for [TypesError].
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        TypesError::InvalidFeelTypeName(s) => {
          write!(f, "invalid FEEL type name: {}", s)
        }
        TypesError::InvalidValueForRetrievingUsingFeelType(s1, s2) => {
          write!(f, "invalid value for retrieving with type check, type = `{}`, value = `{}`", s1, s2)
        }
      }
    }
  }

  /// Creates an [InvalidFeelTypeName](TypesError::InvalidFeelTypeName) error.
  pub fn invalid_feel_type_name(s: &str) -> DmntkError {
    TypesError::InvalidFeelTypeName(s.to_owned()).into()
  }

  /// Creates an [InvalidValueForRetrievingUsingFeelType](TypesError::InvalidValueForRetrievingUsingFeelType) error.
  pub fn invalid_value_for_retrieving_using_feel_type(s1: &str, s2: &str) -> DmntkError {
    TypesError::InvalidValueForRetrievingUsingFeelType(s1.to_owned(), s2.to_owned()).into()
  }
}

#[cfg(test)]
mod tests {
  use crate::names::Name;
  use crate::types::is_built_in_type_name;
  use crate::types::FeelType::{self, Any, Number};
  use crate::values::{Value, Values};
  use crate::{value_number, FeelNumber};

  #[test]
  fn test_get_value_checked() {
    let t_any = FeelType::Any;
    let t_boolean = FeelType::Boolean;
    let v_null = Value::Null(None);
    let v_boolean = Value::Boolean(true);
    let v_string = Value::String("hello".to_owned());
    assert_eq!("null", t_any.get_value_checked(&v_null).unwrap().to_string());
    assert_eq!("true", t_any.get_value_checked(&v_boolean).unwrap().to_string());
    assert!(t_boolean.get_value_checked(&v_string).is_err());
    assert_eq!(
      r#"TypesError: invalid value for retrieving with type check, type = `boolean`, value = `"hello"`"#,
      format!("{}", t_boolean.get_value_checked(&v_string).err().unwrap()).as_str()
    );
  }

  #[test]
  fn test_type_equivalence() {
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let t_any = FeelType::Any;
    let t_boolean = FeelType::Boolean;
    let t_date = FeelType::Date;
    let t_date_time = FeelType::DateTime;
    let t_days_and_time_duration = FeelType::DaysAndTimeDuration;
    let t_null = FeelType::Null;
    let t_number = FeelType::Number;
    let t_string = FeelType::String;
    let t_time = FeelType::Time;
    let t_years_and_months_duration = FeelType::YearsAndMonthsDuration;
    let t_list = FeelType::List(Box::new(FeelType::Boolean));
    let t_list_b = FeelType::List(Box::new(FeelType::Number));
    let t_context = FeelType::context(&[(&name_a, &t_number)]);
    let t_context_a_b = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean)]);
    let t_context_a_b_c = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean), (&name_c, &t_string)]);
    let t_function = FeelType::function(&[Number, Number], &t_number);
    let t_function_b = FeelType::function(&[Number, Number], &t_boolean);
    let t_function_c = FeelType::function(&[Number], &t_string);
    let t_range = FeelType::range(&t_number);
    let t_range_b = FeelType::range(&t_date);
    // any
    assert!(t_any.is_equivalent(&t_any));
    assert!(!t_any.is_equivalent(&t_boolean));
    assert!(!t_any.is_equivalent(&t_context));
    assert!(!t_any.is_equivalent(&t_date));
    assert!(!t_any.is_equivalent(&t_date_time));
    assert!(!t_any.is_equivalent(&t_days_and_time_duration));
    assert!(!t_boolean.is_equivalent(&t_function));
    assert!(!t_boolean.is_equivalent(&t_list));
    assert!(!t_boolean.is_equivalent(&t_null));
    assert!(!t_boolean.is_equivalent(&t_number));
    assert!(!t_boolean.is_equivalent(&t_range));
    assert!(!t_boolean.is_equivalent(&t_string));
    assert!(!t_boolean.is_equivalent(&t_time));
    assert!(!t_boolean.is_equivalent(&t_years_and_months_duration));
    // boolean
    assert!(t_boolean.is_equivalent(&t_boolean));
    assert!(!t_boolean.is_equivalent(&t_any));
    assert!(!t_boolean.is_equivalent(&t_context));
    assert!(!t_boolean.is_equivalent(&t_date));
    assert!(!t_boolean.is_equivalent(&t_date_time));
    assert!(!t_boolean.is_equivalent(&t_days_and_time_duration));
    assert!(!t_boolean.is_equivalent(&t_function));
    assert!(!t_boolean.is_equivalent(&t_list));
    assert!(!t_boolean.is_equivalent(&t_null));
    assert!(!t_boolean.is_equivalent(&t_number));
    assert!(!t_boolean.is_equivalent(&t_range));
    assert!(!t_boolean.is_equivalent(&t_string));
    assert!(!t_boolean.is_equivalent(&t_time));
    assert!(!t_boolean.is_equivalent(&t_years_and_months_duration));
    // context
    assert!(t_context.is_equivalent(&t_context));
    assert!(t_context_a_b.is_equivalent(&t_context_a_b));
    assert!(t_context_a_b_c.is_equivalent(&t_context_a_b_c));
    assert!(!t_context.is_equivalent(&t_context_a_b));
    assert!(!t_context_a_b.is_equivalent(&t_context_a_b_c));
    assert!(!t_context_a_b.is_equivalent(&t_context_a_b_c));
    assert!(!t_context.is_equivalent(&t_any));
    assert!(!t_context.is_equivalent(&t_boolean));
    assert!(!t_context.is_equivalent(&t_date));
    assert!(!t_context.is_equivalent(&t_date_time));
    assert!(!t_context.is_equivalent(&t_days_and_time_duration));
    assert!(!t_context.is_equivalent(&t_function));
    assert!(!t_context.is_equivalent(&t_list));
    assert!(!t_context.is_equivalent(&t_null));
    assert!(!t_context.is_equivalent(&t_number));
    assert!(!t_context.is_equivalent(&t_range));
    assert!(!t_context.is_equivalent(&t_string));
    assert!(!t_context.is_equivalent(&t_time));
    assert!(!t_context.is_equivalent(&t_years_and_months_duration));
    // date
    assert!(t_date.is_equivalent(&t_date));
    assert!(!t_date.is_equivalent(&t_any));
    assert!(!t_date.is_equivalent(&t_boolean));
    assert!(!t_date.is_equivalent(&t_context));
    assert!(!t_date.is_equivalent(&t_date_time));
    assert!(!t_date.is_equivalent(&t_days_and_time_duration));
    assert!(!t_date.is_equivalent(&t_function));
    assert!(!t_date.is_equivalent(&t_list));
    assert!(!t_date.is_equivalent(&t_null));
    assert!(!t_date.is_equivalent(&t_number));
    assert!(!t_date.is_equivalent(&t_range));
    assert!(!t_date.is_equivalent(&t_string));
    assert!(!t_date.is_equivalent(&t_time));
    assert!(!t_date.is_equivalent(&t_years_and_months_duration));
    // date and time
    assert!(t_date_time.is_equivalent(&t_date_time));
    assert!(!t_date_time.is_equivalent(&t_any));
    assert!(!t_date_time.is_equivalent(&t_boolean));
    assert!(!t_date_time.is_equivalent(&t_context));
    assert!(!t_date_time.is_equivalent(&t_date));
    assert!(!t_date_time.is_equivalent(&t_function));
    assert!(!t_date_time.is_equivalent(&t_list));
    assert!(!t_date_time.is_equivalent(&t_null));
    assert!(!t_date_time.is_equivalent(&t_number));
    assert!(!t_date_time.is_equivalent(&t_range));
    assert!(!t_date_time.is_equivalent(&t_string));
    assert!(!t_date_time.is_equivalent(&t_time));
    assert!(!t_date_time.is_equivalent(&t_years_and_months_duration));
    // days and time duration
    assert!(t_days_and_time_duration.is_equivalent(&t_days_and_time_duration));
    assert!(!t_days_and_time_duration.is_equivalent(&t_any));
    assert!(!t_days_and_time_duration.is_equivalent(&t_boolean));
    assert!(!t_days_and_time_duration.is_equivalent(&t_context));
    assert!(!t_days_and_time_duration.is_equivalent(&t_date));
    assert!(!t_days_and_time_duration.is_equivalent(&t_date_time));
    assert!(!t_days_and_time_duration.is_equivalent(&t_function));
    assert!(!t_days_and_time_duration.is_equivalent(&t_list));
    assert!(!t_days_and_time_duration.is_equivalent(&t_null));
    assert!(!t_days_and_time_duration.is_equivalent(&t_number));
    assert!(!t_days_and_time_duration.is_equivalent(&t_range));
    assert!(!t_days_and_time_duration.is_equivalent(&t_string));
    assert!(!t_days_and_time_duration.is_equivalent(&t_time));
    assert!(!t_days_and_time_duration.is_equivalent(&t_years_and_months_duration));
    // function
    assert!(t_function.is_equivalent(&t_function));
    assert!(t_function_b.is_equivalent(&t_function_b));
    assert!(t_function_c.is_equivalent(&t_function_c));
    assert!(!t_function.is_equivalent(&t_function_b));
    assert!(!t_function.is_equivalent(&t_function_c));
    assert!(!t_function_b.is_equivalent(&t_function));
    assert!(!t_function_b.is_equivalent(&t_function_c));
    assert!(!t_function_c.is_equivalent(&t_function));
    assert!(!t_function_c.is_equivalent(&t_function_b));
    assert!(!t_function.is_equivalent(&t_any));
    assert!(!t_function.is_equivalent(&t_boolean));
    assert!(!t_function.is_equivalent(&t_context));
    assert!(!t_function.is_equivalent(&t_date));
    assert!(!t_function.is_equivalent(&t_date_time));
    assert!(!t_function.is_equivalent(&t_days_and_time_duration));
    assert!(!t_function.is_equivalent(&t_list));
    assert!(!t_function.is_equivalent(&t_null));
    assert!(!t_function.is_equivalent(&t_range));
    assert!(!t_function.is_equivalent(&t_string));
    assert!(!t_function.is_equivalent(&t_time));
    assert!(!t_function.is_equivalent(&t_years_and_months_duration));
    // list
    assert!(t_list_b.is_equivalent(&t_list_b));
    assert!(!t_list_b.is_equivalent(&t_list));
    assert!(!t_list.is_equivalent(&t_any));
    assert!(!t_list.is_equivalent(&t_boolean));
    assert!(!t_list.is_equivalent(&t_context));
    assert!(!t_list.is_equivalent(&t_date));
    assert!(!t_list.is_equivalent(&t_date_time));
    assert!(!t_list.is_equivalent(&t_days_and_time_duration));
    assert!(!t_list.is_equivalent(&t_function));
    assert!(!t_list.is_equivalent(&t_null));
    assert!(!t_list.is_equivalent(&t_range));
    assert!(!t_list.is_equivalent(&t_string));
    assert!(!t_list.is_equivalent(&t_time));
    assert!(!t_list.is_equivalent(&t_years_and_months_duration));
    // null
    assert!(t_null.is_equivalent(&t_null));
    assert!(!t_null.is_equivalent(&t_any));
    assert!(!t_null.is_equivalent(&t_boolean));
    assert!(!t_null.is_equivalent(&t_context));
    assert!(!t_null.is_equivalent(&t_date));
    assert!(!t_null.is_equivalent(&t_date_time));
    assert!(!t_null.is_equivalent(&t_days_and_time_duration));
    assert!(!t_null.is_equivalent(&t_function));
    assert!(!t_null.is_equivalent(&t_list));
    assert!(!t_null.is_equivalent(&t_range));
    assert!(!t_null.is_equivalent(&t_string));
    assert!(!t_null.is_equivalent(&t_time));
    assert!(!t_null.is_equivalent(&t_years_and_months_duration));
    // number
    assert!(t_number.is_equivalent(&t_number));
    assert!(!t_number.is_equivalent(&t_any));
    assert!(!t_number.is_equivalent(&t_boolean));
    assert!(!t_number.is_equivalent(&t_context));
    assert!(!t_number.is_equivalent(&t_date));
    assert!(!t_number.is_equivalent(&t_date_time));
    assert!(!t_number.is_equivalent(&t_days_and_time_duration));
    assert!(!t_number.is_equivalent(&t_function));
    assert!(!t_number.is_equivalent(&t_list));
    assert!(!t_number.is_equivalent(&t_null));
    assert!(!t_number.is_equivalent(&t_range));
    assert!(!t_number.is_equivalent(&t_string));
    assert!(!t_number.is_equivalent(&t_time));
    assert!(!t_number.is_equivalent(&t_years_and_months_duration));
    // range
    assert!(t_range.is_equivalent(&t_range));
    assert!(t_range_b.is_equivalent(&t_range_b));
    assert!(!t_range.is_equivalent(&t_any));
    assert!(!t_range.is_equivalent(&t_boolean));
    assert!(!t_range.is_equivalent(&t_context));
    assert!(!t_range.is_equivalent(&t_date));
    assert!(!t_range.is_equivalent(&t_date_time));
    assert!(!t_range.is_equivalent(&t_days_and_time_duration));
    assert!(!t_range.is_equivalent(&t_function));
    assert!(!t_range.is_equivalent(&t_list));
    assert!(!t_range.is_equivalent(&t_null));
    assert!(!t_range.is_equivalent(&t_number));
    assert!(!t_range.is_equivalent(&t_string));
    assert!(!t_range.is_equivalent(&t_time));
    assert!(!t_range.is_equivalent(&t_years_and_months_duration));
    // string
    assert!(t_string.is_equivalent(&t_string));
    assert!(!t_string.is_equivalent(&t_any));
    assert!(!t_string.is_equivalent(&t_boolean));
    assert!(!t_string.is_equivalent(&t_context));
    assert!(!t_string.is_equivalent(&t_date));
    assert!(!t_string.is_equivalent(&t_date_time));
    assert!(!t_string.is_equivalent(&t_days_and_time_duration));
    assert!(!t_string.is_equivalent(&t_function));
    assert!(!t_string.is_equivalent(&t_list));
    assert!(!t_string.is_equivalent(&t_null));
    assert!(!t_string.is_equivalent(&t_number));
    assert!(!t_string.is_equivalent(&t_range));
    assert!(!t_string.is_equivalent(&t_time));
    assert!(!t_string.is_equivalent(&t_years_and_months_duration));
    // time
    assert!(t_time.is_equivalent(&t_time));
    assert!(!t_time.is_equivalent(&t_any));
    assert!(!t_time.is_equivalent(&t_boolean));
    assert!(!t_time.is_equivalent(&t_context));
    assert!(!t_time.is_equivalent(&t_date));
    assert!(!t_time.is_equivalent(&t_date_time));
    assert!(!t_time.is_equivalent(&t_days_and_time_duration));
    assert!(!t_time.is_equivalent(&t_function));
    assert!(!t_time.is_equivalent(&t_list));
    assert!(!t_time.is_equivalent(&t_null));
    assert!(!t_time.is_equivalent(&t_number));
    assert!(!t_time.is_equivalent(&t_range));
    assert!(!t_time.is_equivalent(&t_string));
    assert!(!t_time.is_equivalent(&t_years_and_months_duration));
    // years and months duration
    assert!(t_years_and_months_duration.is_equivalent(&t_years_and_months_duration));
    assert!(!t_years_and_months_duration.is_equivalent(&t_any));
    assert!(!t_years_and_months_duration.is_equivalent(&t_boolean));
    assert!(!t_years_and_months_duration.is_equivalent(&t_context));
    assert!(!t_years_and_months_duration.is_equivalent(&t_date));
    assert!(!t_years_and_months_duration.is_equivalent(&t_date_time));
    assert!(!t_years_and_months_duration.is_equivalent(&t_days_and_time_duration));
    assert!(!t_years_and_months_duration.is_equivalent(&t_function));
    assert!(!t_years_and_months_duration.is_equivalent(&t_list));
    assert!(!t_years_and_months_duration.is_equivalent(&t_null));
    assert!(!t_years_and_months_duration.is_equivalent(&t_number));
    assert!(!t_years_and_months_duration.is_equivalent(&t_range));
    assert!(!t_years_and_months_duration.is_equivalent(&t_string));
    assert!(!t_years_and_months_duration.is_equivalent(&t_time));
  }

  #[test]
  fn test_type_conformance() {
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let t_any = FeelType::Any;
    let t_boolean = FeelType::Boolean;
    let t_date = FeelType::Date;
    let t_date_time = FeelType::DateTime;
    let t_days_and_time_duration = FeelType::DaysAndTimeDuration;
    let t_null = FeelType::Null;
    let t_number = FeelType::Number;
    let t_string = FeelType::String;
    let t_time = FeelType::Time;
    let t_years_and_months_duration = FeelType::YearsAndMonthsDuration;
    let t_list = FeelType::List(Box::new(FeelType::Boolean));
    let t_list_b = FeelType::List(Box::new(FeelType::Number));
    let t_context = FeelType::context(&[(&name_a, &t_number)]);
    let t_context_a_b = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean)]);
    let t_context_a_b_c = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean), (&name_c, &t_string)]);
    let t_function = FeelType::function(&[Number, Number], &t_number);
    let t_function_b = FeelType::function(&[Number, Number], &t_boolean);
    let t_function_c = FeelType::function(&[Number], &t_string);
    let t_function_d = FeelType::function(&[Any], &t_string);
    let t_range = FeelType::range(&t_number);
    let t_range_b = FeelType::range(&t_date);
    // any
    assert!(t_any.is_conformant(&t_any));
    assert!(!t_any.is_conformant(&t_boolean));
    assert!(!t_any.is_conformant(&t_context));
    assert!(!t_any.is_conformant(&t_date));
    assert!(!t_any.is_conformant(&t_date_time));
    assert!(!t_any.is_conformant(&t_days_and_time_duration));
    assert!(!t_boolean.is_conformant(&t_function));
    assert!(!t_boolean.is_conformant(&t_list));
    assert!(!t_boolean.is_conformant(&t_null));
    assert!(!t_boolean.is_conformant(&t_number));
    assert!(!t_boolean.is_conformant(&t_range));
    assert!(!t_boolean.is_conformant(&t_string));
    assert!(!t_boolean.is_conformant(&t_time));
    assert!(!t_boolean.is_conformant(&t_years_and_months_duration));
    // boolean
    assert!(t_boolean.is_conformant(&t_boolean));
    assert!(t_boolean.is_conformant(&t_any));
    assert!(!t_boolean.is_conformant(&t_context));
    assert!(!t_boolean.is_conformant(&t_date));
    assert!(!t_boolean.is_conformant(&t_date_time));
    assert!(!t_boolean.is_conformant(&t_days_and_time_duration));
    assert!(!t_boolean.is_conformant(&t_function));
    assert!(!t_boolean.is_conformant(&t_list));
    assert!(!t_boolean.is_conformant(&t_null));
    assert!(!t_boolean.is_conformant(&t_number));
    assert!(!t_boolean.is_conformant(&t_range));
    assert!(!t_boolean.is_conformant(&t_string));
    assert!(!t_boolean.is_conformant(&t_time));
    assert!(!t_boolean.is_conformant(&t_years_and_months_duration));
    // context
    assert!(t_context.is_conformant(&t_context));
    assert!(t_context_a_b.is_conformant(&t_context_a_b));
    assert!(t_context_a_b.is_conformant(&t_context));
    assert!(t_context_a_b_c.is_conformant(&t_context_a_b_c));
    assert!(t_context_a_b_c.is_conformant(&t_context_a_b));
    assert!(!t_context.is_conformant(&t_context_a_b));
    assert!(!t_context_a_b.is_conformant(&t_context_a_b_c));
    assert!(!t_context_a_b.is_conformant(&t_context_a_b_c));
    assert!(t_context.is_conformant(&t_any));
    assert!(!t_context.is_conformant(&t_boolean));
    assert!(!t_context.is_conformant(&t_date));
    assert!(!t_context.is_conformant(&t_date_time));
    assert!(!t_context.is_conformant(&t_days_and_time_duration));
    assert!(!t_context.is_conformant(&t_function));
    assert!(!t_context.is_conformant(&t_list));
    assert!(!t_context.is_conformant(&t_null));
    assert!(!t_context.is_conformant(&t_number));
    assert!(!t_context.is_conformant(&t_range));
    assert!(!t_context.is_conformant(&t_string));
    assert!(!t_context.is_conformant(&t_time));
    assert!(!t_context.is_conformant(&t_years_and_months_duration));
    // date
    assert!(t_date.is_conformant(&t_date));
    assert!(t_date.is_conformant(&t_any));
    assert!(!t_date.is_conformant(&t_boolean));
    assert!(!t_date.is_conformant(&t_context));
    assert!(!t_date.is_conformant(&t_date_time));
    assert!(!t_date.is_conformant(&t_days_and_time_duration));
    assert!(!t_date.is_conformant(&t_function));
    assert!(!t_date.is_conformant(&t_list));
    assert!(!t_date.is_conformant(&t_null));
    assert!(!t_date.is_conformant(&t_number));
    assert!(!t_date.is_conformant(&t_range));
    assert!(!t_date.is_conformant(&t_string));
    assert!(!t_date.is_conformant(&t_time));
    assert!(!t_date.is_conformant(&t_years_and_months_duration));
    // date and time
    assert!(t_date_time.is_conformant(&t_date_time));
    assert!(t_date_time.is_conformant(&t_any));
    assert!(!t_date_time.is_conformant(&t_boolean));
    assert!(!t_date_time.is_conformant(&t_context));
    assert!(!t_date_time.is_conformant(&t_date));
    assert!(!t_date_time.is_conformant(&t_function));
    assert!(!t_date_time.is_conformant(&t_list));
    assert!(!t_date_time.is_conformant(&t_null));
    assert!(!t_date_time.is_conformant(&t_number));
    assert!(!t_date_time.is_conformant(&t_range));
    assert!(!t_date_time.is_conformant(&t_string));
    assert!(!t_date_time.is_conformant(&t_time));
    assert!(!t_date_time.is_conformant(&t_years_and_months_duration));
    // days and time duration
    assert!(t_days_and_time_duration.is_conformant(&t_days_and_time_duration));
    assert!(t_days_and_time_duration.is_conformant(&t_any));
    assert!(!t_days_and_time_duration.is_conformant(&t_boolean));
    assert!(!t_days_and_time_duration.is_conformant(&t_context));
    assert!(!t_days_and_time_duration.is_conformant(&t_date));
    assert!(!t_days_and_time_duration.is_conformant(&t_date_time));
    assert!(!t_days_and_time_duration.is_conformant(&t_function));
    assert!(!t_days_and_time_duration.is_conformant(&t_list));
    assert!(!t_days_and_time_duration.is_conformant(&t_null));
    assert!(!t_days_and_time_duration.is_conformant(&t_number));
    assert!(!t_days_and_time_duration.is_conformant(&t_range));
    assert!(!t_days_and_time_duration.is_conformant(&t_string));
    assert!(!t_days_and_time_duration.is_conformant(&t_time));
    assert!(!t_days_and_time_duration.is_conformant(&t_years_and_months_duration));
    // function
    assert!(t_function.is_conformant(&t_function));
    assert!(t_function_b.is_conformant(&t_function_b));
    assert!(t_function_c.is_conformant(&t_function_c));
    assert!(!t_function.is_conformant(&t_function_b));
    assert!(!t_function.is_conformant(&t_function_c));
    assert!(!t_function_b.is_conformant(&t_function));
    assert!(!t_function_b.is_conformant(&t_function_c));
    assert!(!t_function_c.is_conformant(&t_function));
    assert!(!t_function_c.is_conformant(&t_function_b));
    assert!(t_function_d.is_conformant(&t_function_c));
    assert!(t_function.is_conformant(&t_any));
    assert!(!t_function.is_conformant(&t_boolean));
    assert!(!t_function.is_conformant(&t_context));
    assert!(!t_function.is_conformant(&t_date));
    assert!(!t_function.is_conformant(&t_date_time));
    assert!(!t_function.is_conformant(&t_days_and_time_duration));
    assert!(!t_function.is_conformant(&t_list));
    assert!(!t_function.is_conformant(&t_null));
    assert!(!t_function.is_conformant(&t_range));
    assert!(!t_function.is_conformant(&t_string));
    assert!(!t_function.is_conformant(&t_time));
    assert!(!t_function.is_conformant(&t_years_and_months_duration));
    // list
    assert!(t_list_b.is_conformant(&t_list_b));
    assert!(!t_list_b.is_conformant(&t_list));
    assert!(t_list.is_conformant(&t_any));
    assert!(!t_list.is_conformant(&t_boolean));
    assert!(!t_list.is_conformant(&t_context));
    assert!(!t_list.is_conformant(&t_date));
    assert!(!t_list.is_conformant(&t_date_time));
    assert!(!t_list.is_conformant(&t_days_and_time_duration));
    assert!(!t_list.is_conformant(&t_function));
    assert!(!t_list.is_conformant(&t_null));
    assert!(!t_list.is_conformant(&t_range));
    assert!(!t_list.is_conformant(&t_string));
    assert!(!t_list.is_conformant(&t_time));
    assert!(!t_list.is_conformant(&t_years_and_months_duration));
    // null
    assert!(t_null.is_conformant(&t_null));
    assert!(t_null.is_conformant(&t_any));
    assert!(t_null.is_conformant(&t_boolean));
    assert!(t_null.is_conformant(&t_context));
    assert!(t_null.is_conformant(&t_date));
    assert!(t_null.is_conformant(&t_date_time));
    assert!(t_null.is_conformant(&t_days_and_time_duration));
    assert!(t_null.is_conformant(&t_function));
    assert!(t_null.is_conformant(&t_list));
    assert!(t_null.is_conformant(&t_range));
    assert!(t_null.is_conformant(&t_string));
    assert!(t_null.is_conformant(&t_time));
    assert!(t_null.is_conformant(&t_years_and_months_duration));
    // number
    assert!(t_number.is_conformant(&t_number));
    assert!(t_number.is_conformant(&t_any));
    assert!(!t_number.is_conformant(&t_boolean));
    assert!(!t_number.is_conformant(&t_context));
    assert!(!t_number.is_conformant(&t_date));
    assert!(!t_number.is_conformant(&t_date_time));
    assert!(!t_number.is_conformant(&t_days_and_time_duration));
    assert!(!t_number.is_conformant(&t_function));
    assert!(!t_number.is_conformant(&t_list));
    assert!(!t_number.is_conformant(&t_null));
    assert!(!t_number.is_conformant(&t_range));
    assert!(!t_number.is_conformant(&t_string));
    assert!(!t_number.is_conformant(&t_time));
    assert!(!t_number.is_conformant(&t_years_and_months_duration));
    // range
    assert!(t_range.is_conformant(&t_range));
    assert!(t_range_b.is_conformant(&t_range_b));
    assert!(t_range.is_conformant(&t_any));
    assert!(!t_range.is_conformant(&t_boolean));
    assert!(!t_range.is_conformant(&t_context));
    assert!(!t_range.is_conformant(&t_date));
    assert!(!t_range.is_conformant(&t_date_time));
    assert!(!t_range.is_conformant(&t_days_and_time_duration));
    assert!(!t_range.is_conformant(&t_function));
    assert!(!t_range.is_conformant(&t_list));
    assert!(!t_range.is_conformant(&t_null));
    assert!(!t_range.is_conformant(&t_number));
    assert!(!t_range.is_conformant(&t_string));
    assert!(!t_range.is_conformant(&t_time));
    assert!(!t_range.is_conformant(&t_years_and_months_duration));
    // string
    assert!(t_string.is_conformant(&t_string));
    assert!(t_string.is_conformant(&t_any));
    assert!(!t_string.is_conformant(&t_boolean));
    assert!(!t_string.is_conformant(&t_context));
    assert!(!t_string.is_conformant(&t_date));
    assert!(!t_string.is_conformant(&t_date_time));
    assert!(!t_string.is_conformant(&t_days_and_time_duration));
    assert!(!t_string.is_conformant(&t_function));
    assert!(!t_string.is_conformant(&t_list));
    assert!(!t_string.is_conformant(&t_null));
    assert!(!t_string.is_conformant(&t_number));
    assert!(!t_string.is_conformant(&t_range));
    assert!(!t_string.is_conformant(&t_time));
    assert!(!t_string.is_conformant(&t_years_and_months_duration));
    // time
    assert!(t_time.is_conformant(&t_time));
    assert!(t_time.is_conformant(&t_any));
    assert!(!t_time.is_conformant(&t_boolean));
    assert!(!t_time.is_conformant(&t_context));
    assert!(!t_time.is_conformant(&t_date));
    assert!(!t_time.is_conformant(&t_date_time));
    assert!(!t_time.is_conformant(&t_days_and_time_duration));
    assert!(!t_time.is_conformant(&t_function));
    assert!(!t_time.is_conformant(&t_list));
    assert!(!t_time.is_conformant(&t_null));
    assert!(!t_time.is_conformant(&t_number));
    assert!(!t_time.is_conformant(&t_range));
    assert!(!t_time.is_conformant(&t_string));
    assert!(!t_time.is_conformant(&t_years_and_months_duration));
    // years and months duration
    assert!(t_years_and_months_duration.is_conformant(&t_years_and_months_duration));
    assert!(t_years_and_months_duration.is_conformant(&t_any));
    assert!(!t_years_and_months_duration.is_conformant(&t_boolean));
    assert!(!t_years_and_months_duration.is_conformant(&t_context));
    assert!(!t_years_and_months_duration.is_conformant(&t_date));
    assert!(!t_years_and_months_duration.is_conformant(&t_date_time));
    assert!(!t_years_and_months_duration.is_conformant(&t_days_and_time_duration));
    assert!(!t_years_and_months_duration.is_conformant(&t_function));
    assert!(!t_years_and_months_duration.is_conformant(&t_list));
    assert!(!t_years_and_months_duration.is_conformant(&t_null));
    assert!(!t_years_and_months_duration.is_conformant(&t_number));
    assert!(!t_years_and_months_duration.is_conformant(&t_range));
    assert!(!t_years_and_months_duration.is_conformant(&t_string));
    assert!(!t_years_and_months_duration.is_conformant(&t_time));
  }

  #[test]
  fn test_is_built_in_type_name() {
    assert!(is_built_in_type_name("Any"));
    assert!(is_built_in_type_name("Null"));
    assert!(is_built_in_type_name("number"));
    assert!(is_built_in_type_name("string"));
    assert!(is_built_in_type_name("boolean"));
    assert!(is_built_in_type_name("days and time duration"));
    assert!(is_built_in_type_name("years and months duration"));
    assert!(is_built_in_type_name("date"));
    assert!(is_built_in_type_name("time"));
    assert!(is_built_in_type_name("date and time"));
    assert!(!is_built_in_type_name("list"));
    assert!(!is_built_in_type_name("range"));
    assert!(!is_built_in_type_name("context"));
    assert!(!is_built_in_type_name("function"));
  }

  #[test]
  fn test_type_stringify() {
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let t_any = FeelType::Any;
    let t_null = FeelType::Null;
    let t_string = FeelType::String;
    let t_number = FeelType::Number;
    let t_boolean = FeelType::Boolean;
    let t_date = FeelType::Date;
    let t_time = FeelType::Time;
    let t_date_time = FeelType::DateTime;
    let t_dt_duration = FeelType::DaysAndTimeDuration;
    let t_ym_duration = FeelType::YearsAndMonthsDuration;
    let t_function_a = FeelType::function(&[Number, Number], &t_number);
    let t_function_b = FeelType::function(&[], &t_any);
    let t_function_c = FeelType::function(&[], &t_string);
    let t_list_a = FeelType::List(Box::new(FeelType::Boolean));
    let t_list_b = FeelType::List(Box::new(FeelType::Number));
    let t_range_a = FeelType::range(&t_number);
    let t_range_b = FeelType::range(&t_date);
    let t_context_a = FeelType::context(&[(&name_a, &t_number)]);
    let t_context_b = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean)]);
    let t_context_c = FeelType::context(&[(&name_a, &t_number), (&name_b, &t_boolean), (&name_c, &t_string)]);
    assert_eq!("Any", t_any.to_string());
    assert_eq!("Null", t_null.to_string());
    assert_eq!("number", t_number.to_string());
    assert_eq!("boolean", t_boolean.to_string());
    assert_eq!("date", t_date.to_string());
    assert_eq!("time", t_time.to_string());
    assert_eq!("date and time", t_date_time.to_string());
    assert_eq!("days and time duration", t_dt_duration.to_string());
    assert_eq!("years and months duration", t_ym_duration.to_string());
    assert_eq!("string", t_string.to_string());
    assert_eq!("function<number, number>->number", t_function_a.to_string());
    assert_eq!("function<>->Any", t_function_b.to_string());
    assert_eq!("function<>->string", t_function_c.to_string());
    assert_eq!("list<boolean>", t_list_a.to_string());
    assert_eq!("list<number>", t_list_b.to_string());
    assert_eq!("range<number>", t_range_a.to_string());
    assert_eq!("range<date>", t_range_b.to_string());
    assert_eq!("context<a: number>", t_context_a.to_string());
    assert_eq!("context<a: number, b: boolean>", t_context_b.to_string());
    assert_eq!("context<a: number, b: boolean, c: string>", t_context_c.to_string());
  }

  #[test]
  fn test_get_coerced() {
    let t_any = FeelType::Any;
    let t_number = FeelType::Number;
    let t_string = FeelType::String;
    let t_list_number = FeelType::List(Box::new(t_number.clone()));
    let t_list_string = FeelType::List(Box::new(t_string.clone()));
    let t_list_any = FeelType::List(Box::new(t_any));
    let v_number = value_number!(10, 0);
    let v_string = Value::String("a".into());
    let v_list_number_1 = Value::List(Values::new(vec![value_number!(1, 0)]));
    let v_list_number_2 = Value::List(Values::new(vec![value_number!(1, 0), value_number!(2, 0)]));
    let v_list_string_1 = Value::List(Values::new(vec![Value::String("A".to_string())]));
    assert_eq!(r#"[10]"#, t_list_number.coerced(&v_number).to_string());
    assert_eq!(r#"[10]"#, t_list_any.coerced(&v_number).to_string());
    assert_eq!(r#"1"#, t_number.coerced(&v_list_number_1).to_string());
    assert_eq!(r#"null"#, t_number.coerced(&v_list_number_2).to_string());
    assert_eq!(r#"null"#, t_number.coerced(&v_list_string_1).to_string());
    assert_eq!(r#"["a"]"#, t_list_string.coerced(&v_string).to_string());
    assert_eq!(r#"["a"]"#, t_list_any.coerced(&v_string).to_string());
    assert_eq!(r#""A""#, t_string.coerced(&v_list_string_1).to_string());
  }
}
