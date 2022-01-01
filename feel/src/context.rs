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

//! `FEEL` context.

use self::errors::*;
use crate::names::{Name, QualifiedName};
use crate::strings::ToFeelString;
use crate::types::FeelType;
use crate::value_null;
use crate::values::Value;
use dmntk_common::{DmntkError, Jsonify};
use std::collections::{BTreeMap, HashSet};
use std::convert::TryFrom;
use std::ops::Deref;

/// Type alias for context entries.
type FeelContextEntries = BTreeMap<Name, Value>;

/// The `FEEL` context.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FeelContext(FeelContextEntries);

impl Deref for FeelContext {
  type Target = FeelContextEntries;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl TryFrom<Value> for FeelContext {
  type Error = DmntkError;
  /// Tries to convert a [Value] to its [FeelContext] representation.
  fn try_from(value: Value) -> Result<Self, Self::Error> {
    if let Value::Context(ctx) = value {
      Ok(ctx)
    } else {
      Err(value_is_not_a_context(&value))
    }
  }
}

impl From<FeelContext> for Value {
  /// Converts this [FeelContext] to its [Value] representation.
  fn from(ctx: FeelContext) -> Self {
    Value::Context(ctx)
  }
}

impl std::fmt::Display for FeelContext {
  ///
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| { format!(r#"{}: {}"#, name, value) })
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl ToFeelString for FeelContext {
  /// Converts [FeelContext] into `FEEL` string.
  fn to_feel_string(&self) -> String {
    format!(
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| {
          let name_str = format!("{}", name);
          let padded_name_str = match name_str.as_str() {
            "{" | "}" | ":" | "," => format!("\"{}\"", name_str),
            "\"" => "\"\\\"\"".to_string(),
            _ => name_str,
          };
          format!(r#"{}: {}"#, padded_name_str, value)
        })
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl Jsonify for FeelContext {
  /// Converts this [FeelContext] into its `JSON` representation.
  fn jsonify(&self) -> String {
    format!(
      "{{{}}}",
      self
        .0
        .iter()
        .map(|(name, value)| format!(r#""{}": {}"#, name, value.jsonify()))
        .collect::<Vec<String>>()
        .join(", ")
    )
  }
}

impl FeelContext {
  /// Checks if this [FeelContext] contains an entry pointed by [Name].
  pub fn contains_entry(&self, name: &Name) -> bool {
    self.0.contains_key(name)
  }
  /// Checks if this [FeelContext] contains an entry pointed by [QualifiedName](crate::names::QualifiedName).
  pub fn contains_entries(&self, qname: &QualifiedName) -> bool {
    self.contains_deep(qname.as_slice())
  }
  /// Sets a single value for specified entry name in this [FeelContext].
  pub fn set_entry(&mut self, name: &Name, value: Value) {
    self.0.insert(name.clone(), value);
  }
  /// Sets a null value for specified entry name in this [FeelContext].
  pub fn set_null(&mut self, name: Name) {
    self.0.insert(name, value_null!());
  }
  /// Returns a value of an entry specified by name.
  pub fn get_entry(&self, name: &Name) -> Option<&Value> {
    self.0.get(name)
  }
  /// Returns a list of key-value pairs.
  pub fn get_entries(&self) -> Vec<(&Name, &Value)> {
    self.0.iter().collect::<Vec<(&Name, &Value)>>()
  }
  /// Returns a first value contained by context.
  pub fn get_first(&self) -> Option<&Value> {
    self.0.values().take(1).next()
  }
  /// Returns the number of entries in this context.
  pub fn len(&self) -> usize {
    self.0.len()
  }
  /// Returns `true` when this context is empty.
  pub fn is_empty(&self) -> bool {
    self.0.is_empty()
  }
  ///
  pub fn zip(&mut self, other: &FeelContext) {
    for (name, value) in &other.0 {
      self.0.insert(name.clone(), value.clone());
    }
  }
  ///
  pub fn overrwrite(&mut self, other: &FeelContext) {
    for (name, value) in &other.0 {
      if self.0.contains_key(name) {
        self.0.insert(name.clone(), value.clone());
      }
    }
  }
  /// Creates an entry with a value for specified [QualifiedName](crate::names::QualifiedName).
  /// All non existing intermediary contexts will be created.
  pub fn create_entry(&mut self, qname: &QualifiedName, value: Value) {
    self.create_deep(qname.as_slice(), value);
  }
  /// Returns a list of flattened keys for this [FeelContext].
  pub fn flatten_keys(&self) -> HashSet<String> {
    let mut keys: HashSet<String> = HashSet::new();
    for (key, value) in self.0.iter() {
      keys.insert(key.into());
      if let Value::Context(sub_ctx) = value {
        let sub_keys = sub_ctx.flatten_keys();
        if !sub_keys.is_empty() {
          for sub_key in sub_keys {
            keys.insert(sub_key.clone());
            keys.insert(format!("{} . {}", key, sub_key));
          }
        }
      }
      if let Value::List(items) = value {
        for item in items.as_vec() {
          if let Value::Context(item_ctx) = item {
            let sub_keys = item_ctx.flatten_keys();
            if !sub_keys.is_empty() {
              for sub_key in sub_keys {
                keys.insert(sub_key.clone());
                keys.insert(format!("{} . {}", key, sub_key)); //TODO add test for this case, nothing happened after adding spaces
              }
            }
          }
        }
      }
      if let Value::FeelType(FeelType::Context(a)) = value {
        for name in a.keys() {
          let sub_key = name.to_string();
          keys.insert(sub_key.clone());
          keys.insert(format!("{} . {}", key, sub_key)); //TODO add test for this case, nothing happened after adding spaces
        }
      }
    }
    keys.iter().cloned().collect()
  }
  /// Searches for a value of an entry pointed by specified qualified name.
  pub fn search_entry<'search>(&'search self, qname: &'search QualifiedName) -> Option<&'search Value> {
    self.search_deep(qname.as_slice())
  }
  /// Deep check for a value pointed by slice of names.
  pub fn contains_deep(&self, names: &[Name]) -> bool {
    if names.is_empty() {
      return false;
    }
    let tail = &names[1..];
    if let Some(value) = self.0.get(&names[0]) {
      if tail.is_empty() {
        return true;
      }
      if let Value::Context(context) = value {
        return context.contains_deep(tail);
      }
    }
    false
  }
  /// Creates intermediary contexts when needed.
  pub fn create_deep(&mut self, names: &[Name], value: Value) {
    // if there are no names, then return
    if names.is_empty() {
      return;
    }
    let key = names[0].clone();
    let tail = &names[1..];
    // if tail is empty, then insert the value under
    // specified key in current context and return
    if tail.is_empty() {
      self.0.insert(key, value);
      return;
    }
    // if there is a context under the specified key,
    // then insert value to this context and return
    if let Some(Value::Context(sub_ctx)) = self.0.get_mut(&key) {
      sub_ctx.create_deep(tail, value);
      return;
    }
    // finally, when got to this point, insert a value
    // to newly created context
    let mut sub_ctx = FeelContext::default();
    sub_ctx.create_deep(tail, value);
    self.0.insert(key, sub_ctx.into());
  }
  /// Deep search for a value pointed by names.
  pub fn search_deep(&self, names: &[Name]) -> Option<&Value> {
    if !names.is_empty() {
      let tail = &names[1..];
      if let Some(value) = self.0.get(&names[0]) {
        if let Value::Context(ctx) = value {
          return if tail.is_empty() { Some(value) } else { ctx.search_deep(tail) };
        } else if tail.is_empty() {
          return Some(value);
        }
      }
    }
    None
  }
}

/// Definitions of context errors.
pub mod errors {
  use crate::values::Value;
  use dmntk_common::DmntkError;

  /// Context errors.
  #[derive(Debug, PartialEq)]
  enum ContextError {
    /// Used when converting a [Value] to [Context].
    ValueIsNotAContext(String),
  }

  impl From<ContextError> for DmntkError {
    fn from(e: ContextError) -> Self {
      DmntkError::new("ContextError", &format!("{}", e))
    }
  }

  impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        ContextError::ValueIsNotAContext(text) => {
          write!(f, "'{}' is not a value containing context", text)
        }
      }
    }
  }

  pub fn value_is_not_a_context(value: &Value) -> DmntkError {
    ContextError::ValueIsNotAContext(value.to_string()).into()
  }
}

#[cfg(test)]
mod tests {
  use crate::context::FeelContext;
  use crate::names::{Name, QualifiedName};
  use crate::values::Value;
  use crate::{value_number, FeelNumber, ToFeelString};
  use dmntk_common::Jsonify;

  #[test]
  fn test_context_default() {
    let ctx: FeelContext = Default::default();
    assert_eq!("{}", ctx.to_string());
    assert!(ctx.flatten_keys().is_empty());
  }

  #[test]
  fn test_context_to_string() {
    let name_a = Name::from("a");
    let name_x_y = Name::new(&["x", "y"]);
    let name_k_plus_l_minus_m = Name::new(&["k", "+", "l", "-", "m"]);
    let mut ctx: FeelContext = Default::default();
    ctx.set_entry(&name_a, value_number!(10));
    assert_eq!(r#"{a: 10}"#, ctx.to_string());
    ctx.set_entry(&name_x_y, Value::Boolean(true));
    assert_eq!(r#"{a: 10, x y: true}"#, ctx.to_string());
    ctx.set_entry(&name_k_plus_l_minus_m, Value::String("KLM".to_string()));
    assert_eq!(r#"{a: 10, k+l-m: "KLM", x y: true}"#, ctx.to_string());
  }

  #[test]
  fn test_context_to_feel_string() {
    let name_a = Name::from("a");
    let name_x_y = Name::new(&["x", "y"]);
    let name_k_plus_l_minus_m = Name::new(&["k", "+", "l", "-", "m"]);
    let mut ctx: FeelContext = Default::default();
    ctx.set_entry(&name_a, value_number!(10));
    assert_eq!(r#"{a: 10}"#, ctx.to_feel_string());
    ctx.set_entry(&name_x_y, Value::Boolean(true));
    assert_eq!(r#"{a: 10, x y: true}"#, ctx.to_feel_string());
    ctx.set_entry(&name_k_plus_l_minus_m, Value::String("KLM".to_string()));
    assert_eq!(r#"{a: 10, k+l-m: "KLM", x y: true}"#, ctx.to_feel_string());
    let mut ctx: FeelContext = Default::default();
    let name_left_bracket = Name::from("{");
    ctx.set_entry(&name_left_bracket, value_number!(1));
    assert_eq!(r#"{"{": 1}"#, ctx.to_feel_string());
    let name_right_bracket = Name::from("}");
    ctx.set_entry(&name_right_bracket, value_number!(2));
    assert_eq!(r#"{"{": 1, "}": 2}"#, ctx.to_feel_string());
    let name_colon = Name::from(":");
    ctx.set_entry(&name_colon, value_number!(3));
    assert_eq!(r#"{":": 3, "{": 1, "}": 2}"#, ctx.to_feel_string());
    let name_comma = Name::from(",");
    ctx.set_entry(&name_comma, value_number!(4));
    assert_eq!(r#"{",": 4, ":": 3, "{": 1, "}": 2}"#, ctx.to_feel_string());
    let name_double_quote = Name::from("\"");
    ctx.set_entry(&name_double_quote, value_number!(5));
    assert_eq!(r#"{"\"": 5, ",": 4, ":": 3, "{": 1, "}": 2}"#, ctx.to_feel_string());
  }

  #[test]
  fn test_context_to_json() {
    let name_a = Name::from("a");
    let name_x_y = Name::new(&["x", "y"]);
    let name_k_plus_l_minus_m = Name::new(&["k", "+", "l", "-", "m"]);
    let mut ctx: FeelContext = Default::default();
    ctx.set_entry(&name_a, value_number!(10));
    assert_eq!(r#"{"a": 10}"#, ctx.jsonify());
    ctx.set_entry(&name_x_y, Value::Boolean(true));
    assert_eq!(r#"{"a": 10, "x y": true}"#, ctx.jsonify());
    ctx.set_entry(&name_k_plus_l_minus_m, Value::String("KLM".to_string()));
    assert_eq!(r#"{"a": 10, "k+l-m": "KLM", "x y": true}"#, ctx.jsonify());
  }

  #[test]
  fn test_context_one_level() {
    let name_a = Name::from("a");
    let name_a_b = Name::new(&["a", "b"]);
    let name_a_b_c = Name::new(&["a", "b", "c"]);
    let qname_a = QualifiedName::new(&[&name_a]);
    let qname_a_b = QualifiedName::new(&[&name_a_b]);
    let qname_a_b_c = QualifiedName::new(&[&name_a_b_c]);
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_a, value_number!(10));
    assert_eq!("{a: 10}", ctx_a.to_string());
    assert!(ctx_a.contains_entry(&name_a));
    assert!(ctx_a.contains_entries(&qname_a));
    assert!(!ctx_a.contains_entry(&name_a_b));
    assert!(!ctx_a.contains_entries(&qname_a_b));
    assert!(!ctx_a.contains_entry(&name_a_b_c));
    assert!(!ctx_a.contains_entries(&qname_a_b_c));
    assert!(ctx_a.flatten_keys().contains("a"));
    assert_eq!("10", ctx_a.get_entry(&name_a).unwrap().to_string().as_str());
  }

  #[test]
  fn test_context_two_levels() {
    let name_married = Name::from("married");
    let name_age = Name::from("age");
    let name_b = Name::from("b");
    let name_x_y = Name::new(&["x", "y"]);
    let qname_married = QualifiedName::new(&[&name_married]);
    let qname_age = QualifiedName::new(&[&name_age]);
    let qname_b = QualifiedName::new(&[&name_b]);
    let qname_b_married = QualifiedName::new(&[&name_b, &name_married]);
    let qname_b_married_age = QualifiedName::new(&[&name_b, &name_married, &name_age]);
    let mut ctx_b: FeelContext = Default::default();
    ctx_b.set_entry(&name_married, Value::Boolean(true));
    assert_eq!("{married: true}", ctx_b.to_string());
    assert!(ctx_b.contains_entry(&name_married));
    assert!(ctx_b.contains_entries(&qname_married));
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_age, value_number!(49));
    ctx_a.set_entry(&name_x_y, Value::Boolean(true));
    ctx_a.set_entry(&name_b, ctx_b.into());
    assert_eq!("{age: 49, b: {married: true}, x y: true}", ctx_a.to_string());
    assert!(ctx_a.contains_entry(&name_age));
    assert!(ctx_a.contains_entry(&name_b));
    assert!(ctx_a.contains_entry(&name_x_y));
    assert!(ctx_a.contains_entries(&qname_age));
    assert!(ctx_a.contains_entries(&qname_b));
    assert!(ctx_a.contains_entries(&qname_b_married));
    assert!(!ctx_a.contains_entries(&qname_b_married_age));
    assert!(ctx_a.flatten_keys().contains("age"));
    assert!(ctx_a.flatten_keys().contains("b"));
    assert!(ctx_a.flatten_keys().contains("b . married"));
    assert_eq!("49", ctx_a.get_entry(&name_age).unwrap().to_string().as_str());
    assert_eq!("{married: true}", ctx_a.get_entry(&name_b).unwrap().to_string().as_str());
  }

  #[test]
  fn test_context_three_levels() {
    let name_car = Name::from("car");
    let name_married = Name::from("married");
    let name_age = Name::from("age");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let mut ctx_c: FeelContext = Default::default();
    ctx_c.set_entry(&name_car, Value::String("opel".to_string()));
    assert_eq!(r#"{car: "opel"}"#, ctx_c.to_string());
    assert!(ctx_c.contains_entry(&name_car));
    let mut ctx_b: FeelContext = Default::default();
    ctx_b.set_entry(&name_married, Value::Boolean(true));
    ctx_b.set_entry(&name_c, ctx_c.into());
    assert_eq!(r#"{c: {car: "opel"}, married: true}"#, ctx_b.to_string());
    assert!(ctx_b.contains_entry(&name_married));
    assert!(ctx_b.contains_entry(&name_c));
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_age, value_number!(49));
    ctx_a.set_entry(&name_b, ctx_b.into());
    assert_eq!(r#"{age: 49, b: {c: {car: "opel"}, married: true}}"#, ctx_a.to_string());
    assert!(ctx_a.contains_entry(&name_age));
    assert!(ctx_a.contains_entry(&name_b));
    assert!(ctx_a.flatten_keys().contains("age"));
    assert!(ctx_a.flatten_keys().contains("b"));
    assert!(ctx_a.flatten_keys().contains("b . c"));
    assert!(ctx_a.flatten_keys().contains("b . married"));
    assert!(ctx_a.flatten_keys().contains("b . c . car"));
  }

  #[test]
  fn test_context_search_entry() {
    let name_married = Name::from("married");
    let name_age = Name::from("age");
    let name_b = Name::from("b");
    let mut ctx_b: FeelContext = Default::default();
    ctx_b.set_entry(&name_married, Value::Boolean(true));
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_age, value_number!(49));
    ctx_a.set_entry(&name_b, ctx_b.into());
    let qn_empty = QualifiedName::new(&[]);
    assert!(ctx_a.search_entry(&qn_empty).is_none());
    let qn_b = QualifiedName::new(&[&name_b]);
    assert_eq!("{married: true}", ctx_a.search_entry(&qn_b).unwrap().to_string().as_str());
    let qn_b_married = QualifiedName::new(&[&name_b, &name_married]);
    assert_eq!("true", ctx_a.search_entry(&qn_b_married).unwrap().to_string().as_str());
  }

  #[test]
  fn test_context_search_entries() {
    // prepare names
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let name_d = Name::from("d");
    let name_e = Name::from("e");
    // prepare qualified names
    let qn_a = QualifiedName::new(&[&name_a]);
    let qn_b = QualifiedName::new(&[&name_b]);
    let qn_c = QualifiedName::new(&[&name_c]);
    let qn_a_b = QualifiedName::new(&[&name_a, &name_b]);
    let qn_a_c = QualifiedName::new(&[&name_a, &name_c]);
    let qn_b_c = QualifiedName::new(&[&name_b, &name_c]);
    let qn_b_e = QualifiedName::new(&[&name_b, &name_e]);
    let qn_b_d = QualifiedName::new(&[&name_b, &name_d]);
    let qn_b_d_a = QualifiedName::new(&[&name_b, &name_d, &name_a]);
    let qn_b_d_e = QualifiedName::new(&[&name_b, &name_d, &name_e]);
    // prepare contexts
    let mut ctx_c: FeelContext = Default::default();
    ctx_c.set_entry(&name_e, Value::String("e".to_string()));
    assert_eq!(r#"{e: "e"}"#, ctx_c.to_string());
    let mut ctx_b: FeelContext = Default::default();
    ctx_b.set_entry(&name_c, Value::String("c".to_string()));
    ctx_b.set_entry(&name_d, ctx_c.into());
    assert_eq!(r#"{c: "c", d: {e: "e"}}"#, ctx_b.to_string());
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_a, Value::String("a".to_string()));
    ctx_a.set_entry(&name_b, ctx_b.into());
    assert_eq!(r#"{a: "a", b: {c: "c", d: {e: "e"}}}"#, ctx_a.to_string());
    // test searching entries
    assert!(ctx_a.contains_entries(&qn_a));
    assert!(ctx_a.contains_entries(&qn_b));
    assert!(!ctx_a.contains_entries(&qn_c));
    assert!(!ctx_a.contains_entries(&qn_a_b));
    assert!(!ctx_a.contains_entries(&qn_a_c));
    assert!(ctx_a.contains_entries(&qn_b_c));
    assert!(!ctx_a.contains_entries(&qn_b_e));
    assert!(ctx_a.contains_entries(&qn_b_d));
    assert!(!ctx_a.contains_entries(&qn_b_d_a));
    assert!(ctx_a.contains_entries(&qn_b_d_e));
  }

  #[test]
  fn test_context_create_entry() {
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let name_d = Name::from("d");
    let qn_a = QualifiedName::new(&[&name_a]);
    let qn_b = QualifiedName::new(&[&name_b]);
    let qn_a_b = QualifiedName::new(&[&name_a, &name_b]);
    let qn_a_c = QualifiedName::new(&[&name_a, &name_c]);
    let qn_a_d = QualifiedName::new(&[&name_a, &name_d]);
    let qn_c_d = QualifiedName::new(&[&name_c, &name_d]);
    let qn_a_b_c = QualifiedName::new(&[&name_a, &name_b, &name_c]);
    let qn_a_b_c_d = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d]);
    let mut ctx: FeelContext = Default::default();
    ctx.create_entry(&qn_a_b_c_d, Value::Boolean(true));
    assert_eq!("{a: {b: {c: {d: true}}}}", ctx.to_string().as_str());
    assert_eq!("{b: {c: {d: true}}}", ctx.search_entry(&qn_a).unwrap().to_string().as_str());
    assert_eq!("{c: {d: true}}", ctx.search_entry(&qn_a_b).unwrap().to_string().as_str());
    assert_eq!("{d: true}", ctx.search_entry(&qn_a_b_c).unwrap().to_string().as_str());
    assert_eq!("true", ctx.search_entry(&qn_a_b_c_d).unwrap().to_string().as_str());
    let mut ctx: FeelContext = Default::default();
    ctx.create_entry(&qn_a, Value::Boolean(true));
    ctx.create_entry(&qn_b, Value::Boolean(false));
    ctx.create_entry(&qn_c_d, Value::String("deep".to_string()));
    assert_eq!(r#"{a: true, b: false, c: {d: "deep"}}"#, ctx.to_string().as_str());
    let mut ctx: FeelContext = Default::default();
    ctx.create_entry(&qn_a_b, Value::String("b".to_string()));
    ctx.create_entry(&qn_a_c, Value::String("c".to_string()));
    ctx.create_entry(&qn_a_d, Value::String("d".to_string()));
    assert_eq!(r#"{a: {b: "b", c: "c", d: "d"}}"#, ctx.to_string().as_str());
  }

  #[test]
  fn test_context_flatten_keys_one_level() {
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let mut ctx: FeelContext = Default::default();
    ctx.set_entry(&name_a, value_number!(1));
    ctx.set_entry(&name_b, value_number!(2));
    assert_eq!(r#"{a: 1, b: 2}"#, ctx.to_string());
    let keys = ctx.flatten_keys();
    assert_eq!(2, keys.len());
    assert!(keys.contains("a"));
    assert!(keys.contains("b"));
  }

  #[test]
  fn test_flatten_two_levels() {
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let mut ctx_b: FeelContext = Default::default();
    ctx_b.set_entry(&name_a, value_number!(10));
    ctx_b.set_entry(&name_b, value_number!(20));
    ctx_b.set_entry(&name_c, value_number!(30));
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_a, value_number!(1));
    ctx_a.set_entry(&name_b, value_number!(2));
    ctx_a.set_entry(&name_c, ctx_b.into());
    let keys = ctx_a.flatten_keys();
    assert_eq!(6, keys.len());
    assert!(keys.contains("a"));
    assert!(keys.contains("b"));
    assert!(keys.contains("c"));
    assert!(keys.contains("c . a"));
    assert!(keys.contains("c . b"));
    assert!(keys.contains("c . c"));
  }

  #[test]
  fn test_flatten_three_levels() {
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    let name_c = Name::from("c");
    let name_d = Name::from("d");
    let mut ctx_c: FeelContext = Default::default();
    ctx_c.set_entry(&name_a, value_number!(100));
    ctx_c.set_entry(&name_b, value_number!(200));
    ctx_c.set_entry(&name_c, value_number!(300));
    ctx_c.set_entry(&name_d, value_number!(400));
    let mut ctx_b: FeelContext = Default::default();
    ctx_b.set_entry(&name_a, value_number!(10));
    ctx_b.set_entry(&name_b, value_number!(20));
    ctx_b.set_entry(&name_c, value_number!(30));
    ctx_b.set_entry(&name_d, ctx_c.into());
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_a, value_number!(1));
    ctx_a.set_entry(&name_b, value_number!(2));
    ctx_a.set_entry(&name_c, ctx_b.into());
    let keys = ctx_a.flatten_keys();
    assert_eq!(16, keys.len());
    assert!(keys.contains("a"));
    assert!(keys.contains("b"));
    assert!(keys.contains("c"));
    assert!(keys.contains("d"));
    assert!(keys.contains("c . a"));
    assert!(keys.contains("c . b"));
    assert!(keys.contains("c . c"));
    assert!(keys.contains("c . d"));
    assert!(keys.contains("c . d . a"));
    assert!(keys.contains("c . d . b"));
    assert!(keys.contains("c . d . c"));
    assert!(keys.contains("c . d . d"));
    assert!(keys.contains("d . a"));
    assert!(keys.contains("d . b"));
    assert!(keys.contains("d . c"));
    assert!(keys.contains("d . d"));
  }

  #[test]
  fn test_flatten_names_with_additional_characters() {
    let name_a = Name::new(&["lorem", "ipsum", "dolor", "sit", "amet"]);
    let name_b = Name::new(&["b"]);
    let name_c = Name::new(&["now", "let", "'", "s", "go", "to", "the", "next", "paragraph"]);
    let name_d = Name::new(&["Curabitur", "rhoncus", "+", "sodales", "odio", "in", "fringilla"]);
    let mut ctx_b: FeelContext = Default::default();
    ctx_b.set_entry(&name_d, Value::Boolean(false));
    let mut ctx_a: FeelContext = Default::default();
    ctx_a.set_entry(&name_a, value_number!(1));
    ctx_a.set_entry(&name_b, value_number!(2));
    ctx_a.set_entry(&name_c, ctx_b.into());
    let keys = ctx_a.flatten_keys();
    assert_eq!(5, keys.len());
    assert!(keys.contains("b"));
    assert!(keys.contains("lorem ipsum dolor sit amet"));
    assert!(keys.contains("now let's go to the next paragraph"));
    assert!(keys.contains("now let's go to the next paragraph . Curabitur rhoncus+sodales odio in fringilla"));
    assert!(keys.contains("Curabitur rhoncus+sodales odio in fringilla"));
  }

  #[test]
  fn test_context_set_null() {
    let name_a = Name::from("a");
    let mut ctx: FeelContext = Default::default();
    ctx.set_null(name_a);
    assert_eq!(r#"{a: null}"#, ctx.to_string());
  }

  #[test]
  fn test_get_entries() {
    let mut ctx: FeelContext = Default::default();
    assert_eq!(0, ctx.get_entries().len());
    let name_a = Name::new(&["a"]);
    ctx.set_entry(&name_a, value_number!(1));
    assert_eq!(1, ctx.get_entries().len());
    assert_eq!(vec![(&name_a, &value_number!(1))], ctx.get_entries());
    let name_b = Name::new(&["b"]);
    ctx.set_entry(&name_b, value_number!(2));
    assert_eq!(2, ctx.get_entries().len());
    assert_eq!(vec![(&name_a, &value_number!(1)), (&name_b, &value_number!(2))], ctx.get_entries());
  }
}
