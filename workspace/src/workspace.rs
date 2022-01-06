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

//! Container for DMN™ models.
//!
//! Workspace has two *virtual* states:
//! - `STASHING`: all model evaluators are deleted but model definitions can be freely modified,
//! - `DEPLOYED`: model evaluators are deployed, model definitions remain unmodified.
//!
//! ### Remarks
//!
//! **Importing rules** require, that the `namespace` attribute in [Definitions] is globally unique.
//! It is assumed, that attributes `Definitions.namespace` and `Definitions.name` are both
//! unique inside [Workspace]. In consequence, the same [Definitions] can be accessed using
//! either a `namespace` or `name` attribute, so there will be an error reported, when two definitions
//! deployed in a single workspace have the same `namespace` or `name` attributes.

use crate::errors::*;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_model::model::{Definitions, NamedElement};
use dmntk_model_evaluator::ModelEvaluator;
use std::collections::HashMap;
use std::sync::Arc;

/// Structure representing the container for DMN™ models.
pub struct Workspace {
  /// Collection of [Definitions] stashed in [Workspace].
  definitions: Vec<Arc<Definitions>>,
  /// Map of [Definitions] indexed by [Definitions].**namespace** attribute.
  definitions_by_namespace: HashMap<String, Arc<Definitions>>,
  /// Map of [Definitions] indexed by [Definitions].**name** attribute.
  definitions_by_name: HashMap<String, Arc<Definitions>>,
  /// Map of [ModelEvaluator] indexed by [Definitions].**name** attribute.
  model_evaluators_by_name: HashMap<String, Arc<ModelEvaluator>>,
}

impl Workspace {
  /// Creates an empty [Workspace].
  pub fn new() -> Self {
    Self {
      definitions: vec![],
      definitions_by_namespace: HashMap::new(),
      definitions_by_name: HashMap::new(),
      model_evaluators_by_name: HashMap::new(),
    }
  }
  /// Deletes all definitions and model evaluators.
  pub fn clear(&mut self) {
    self.clear_definitions();
    self.clear_model_evaluators();
  }
  ///
  pub fn add(&mut self, definitions: Definitions) -> Result<()> {
    let namespace = definitions.namespace().to_string();
    if self.definitions_by_namespace.contains_key(&namespace) {
      return Err(err_definitions_with_namespace_already_exists(&namespace));
    }
    let name = definitions.name().to_string();
    if self.definitions_by_name.contains_key(&name) {
      return Err(err_definitions_with_name_already_exists(&name));
    }
    let definitions_arc = Arc::new(definitions);
    self.definitions_by_namespace.insert(namespace, Arc::clone(&definitions_arc));
    self.definitions_by_name.insert(name, Arc::clone(&definitions_arc));
    self.definitions.push(definitions_arc);
    self.clear_model_evaluators();
    Ok(())
  }
  ///
  pub fn remove(&mut self, namespace: &str, name: &str) {
    self.definitions_by_namespace.remove(namespace);
    self.definitions_by_name.remove(name);
    self.definitions.retain(|d| d.namespace() != namespace && d.name() != name);
    self.clear_model_evaluators();
  }
  ///
  pub fn replace(&mut self, definitions: Definitions) -> Result<()> {
    self.remove(definitions.namespace(), definitions.name());
    self.add(definitions)
  }
  ///
  pub fn deploy(&mut self) -> Result<()> {
    self.clear_model_evaluators();
    for definitions in &self.definitions {
      let model_evaluator = ModelEvaluator::new(definitions)?;
      let name = definitions.name().to_string();
      self.model_evaluators_by_name.insert(name, model_evaluator);
    }
    Ok(())
  }
  /// Evaluates invocable (decision, business knowledge model or decision service) deployed in [Workspace].
  pub fn evaluate_invocable(&self, model_name: &str, invocable_name: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some(model_evaluator) = self.model_evaluators_by_name.get(model_name) {
      Ok(model_evaluator.evaluate_invocable(invocable_name, input_data))
    } else {
      Err(err_model_evaluator_not_deployed(model_name))
    }
  }
  /// Deletes all definitions stashed in workspace.
  fn clear_definitions(&mut self) {
    self.definitions_by_name.clear();
    self.definitions_by_namespace.clear();
    self.definitions.clear();
  }
  /// Deletes all model evaluators deployed in workspace.
  fn clear_model_evaluators(&mut self) {
    self.model_evaluators_by_name.clear();
  }
}
