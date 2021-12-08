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

//! `FEEL` scope.

use crate::context::FeelContext;
use crate::values::Value;
use crate::Name;
use dmntk_common::Jsonify;
use std::cell::RefCell;
use std::collections::HashSet;

/// Creates a scope.
#[macro_export]
macro_rules! scope {
  () => {{
    Scope::default()
  }};
}

/// The `FEEL` scope.
#[derive(Debug)]
pub struct Scope {
  /// The stack of contexts.
  contexts: RefCell<Vec<FeelContext>>,
}

impl Default for Scope {
  /// Creates a default [Scope] containing single default [FeelContext].
  fn default() -> Self {
    Self {
      contexts: RefCell::new(vec![FeelContext::default()]),
    }
  }
}

impl From<FeelContext> for Scope {
  /// Creates a [Scope] from [FeelContext].
  fn from(context: FeelContext) -> Self {
    Self {
      contexts: RefCell::new(vec![context]),
    }
  }
}

impl std::fmt::Display for Scope {
  /// Converts this [Scope] to its textual representation.
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "[{}]",
      self.contexts.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", ")
    )
  }
}

impl Jsonify for Scope {
  /// Converts this [Scope] to its `JSON` representation.
  fn jsonify(&self) -> String {
    format!(
      "[{}]",
      self.contexts.borrow_mut().iter().map(|ctx| ctx.to_string()).collect::<Vec<String>>().join(", ")
    )
  }
}

impl Scope {
  /// Creates a new and empty [Scope].
  pub fn new() -> Self {
    Self {
      contexts: RefCell::new(vec![]),
    }
  }
  /// Pushes a context on the top of the scope stack.
  pub fn push(&self, ctx: FeelContext) {
    self.contexts.borrow_mut().push(ctx)
  }
  /// Takes and returns a context from the top of the stack.
  pub fn pop(&self) -> Option<FeelContext> {
    self.contexts.borrow_mut().pop()
  }
  /// Peeks a to context from the top of the stack.
  /// If the stack is empty, the default context is returned.
  pub fn peek(&self) -> FeelContext {
    //FIXME maybe returning a reference is enough???
    self.contexts.borrow_mut().last().map_or(FeelContext::default(), |ctx| ctx.clone())
  }
  /// Returns a vector of flattened keys in all contexts in scope.
  pub fn flatten_keys(&self) -> HashSet<String> {
    self
      .contexts
      .borrow_mut()
      .iter()
      .flat_map(|ctx| ctx.flatten_keys())
      .collect::<HashSet<String>>()
  }
  /// Returns a value for an entry specified by name.
  /// Entries are searched from the last to the first context
  /// (from top to bottom of scope stack).
  pub fn get_entry(&self, name: &Name) -> Option<Value> {
    for context in self.contexts.borrow_mut().iter().rev() {
      if let Some(value) = context.get_entry(name) {
        return Some(value.clone());
      }
    }
    None
  }
  ///
  pub fn search_deep(&self, names: &[Name]) -> Option<Value> {
    for context in self.contexts.borrow_mut().iter().rev() {
      if let Some(value) = context.search_deep(names) {
        return Some(value.clone());
      }
    }
    None
  }
  /// Sets a specified value for entry name in [FeelContext] placed on the top of the scope stack (last context).
  pub fn set_entry(&self, name: &Name, value: Value) {
    if let Some(context) = self.contexts.borrow_mut().last_mut() {
      context.set_entry(name, value);
    }
  }
  /// Sets a null value for entry name in [FeelContext] placed on the top of the scope stack (last context).
  pub fn insert_null(&self, name: Name) {
    if let Some(context) = self.contexts.borrow_mut().last_mut() {
      context.set_null(name);
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::context::FeelContext;
  use crate::values::Value;
  use crate::{scope, value_number, FeelNumber, Name, Scope};

  #[test]
  fn test_scope_default() {
    assert_eq!("[{}]", Scope::default().to_string());
    assert_eq!("[{}]", scope!().to_string());
  }

  #[test]
  fn test_scope_new() {
    assert_eq!("[]", Scope::new().to_string());
  }

  #[test]
  fn test_scope_single_empty_context() {
    let scope = Scope::new();
    let ctx = FeelContext::default();
    scope.push(ctx);
    assert_eq!("[{}]", scope.to_string());
    let scope: Scope = FeelContext::default().into();
    assert_eq!("[{}]", scope.to_string());
  }

  #[test]
  fn test_scope_multiple_empty_contexts() {
    let scope = Scope::new();
    scope.push(FeelContext::default());
    scope.push(FeelContext::default());
    scope.push(FeelContext::default());
    assert_eq!("[{}, {}, {}]", scope.to_string());
    let scope = Scope::default();
    scope.push(FeelContext::default());
    scope.push(FeelContext::default());
    scope.push(FeelContext::default());
    assert_eq!("[{}, {}, {}, {}]", scope.to_string());
  }

  #[test]
  fn test_scope_single_context() {
    let scope = Scope::default();
    assert_eq!("[{}]", scope.to_string());
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    scope.set_entry(&name_a, value_number!(495, 1));
    assert_eq!("[{a: 49.5}]", scope.to_string());
    scope.set_entry(&name_b, value_number!(50));
    assert_eq!("[{a: 49.5, b: 50}]", scope.to_string());
    scope.pop();
    assert_eq!("[]", scope.to_string());
  }

  #[test]
  fn test_scope_no_context() {
    let scope = Scope::new();
    assert_eq!("[]", scope.to_string());
    let name_a = Name::from("a");
    let name_b = Name::from("b");
    scope.set_entry(&name_a, value_number!(125, 2));
    assert_eq!("[]", scope.to_string());
    scope.set_entry(&name_b, value_number!(175, 2));
    assert_eq!("[]", scope.to_string());
    scope.pop();
    assert_eq!("[]", scope.to_string());
  }
}
