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
  pub fn new(_workspace_dir: Option<&str>) -> Self {
    Self {
      definitions: vec![],
      definitions_by_namespace: HashMap::new(),
      definitions_by_name: HashMap::new(),
      model_evaluators_by_name: HashMap::new(),
    }
  }
  /// Deletes all definitions and model evaluators,
  /// switches a workspace to state `STASHING`.
  pub fn clear(&mut self) {
    self.clear_definitions();
    self.clear_model_evaluators();
  }
  /// Adds a definition to workspace, deletes all model evaluators,
  /// switches a workspace to state `STASHING`.
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
  /// Removes a definition from workspace, deletes all model evaluators,
  /// switches a workspace to state `STASHING`.
  pub fn remove(&mut self, namespace: &str, name: &str) {
    self.definitions_by_namespace.remove(namespace);
    self.definitions_by_name.remove(name);
    self.definitions.retain(|d| d.namespace() != namespace && d.name() != name);
    self.clear_model_evaluators();
  }
  /// Replaces a definition in workspace, deletes all model evaluators,
  /// switches a workspace to state `STASHING`.
  pub fn replace(&mut self, definitions: Definitions) -> Result<()> {
    self.remove(definitions.namespace(), definitions.name());
    self.add(definitions)
  }
  /// Creates model evaluators for all definitions in workspace,
  /// switches a workspace to state `DEPLOYED`.
  pub fn deploy(&mut self) -> Result<()> {
    self.clear_model_evaluators();
    for definitions in &self.definitions {
      match ModelEvaluator::new(definitions) {
        Ok(model_evaluator) => {
          let name = definitions.name().to_string();
          self.model_evaluators_by_name.insert(name, model_evaluator);
        }
        Err(_reason) => {
          //TODO prepare status report
        }
      }
    }
    Ok(())
  }
  /// Evaluates invocable (decision, business knowledge model or decision service) deployed in workspace.
  pub fn evaluate_invocable(&self, model_name: &str, invocable_name: &str, input_data: &FeelContext) -> Result<Value> {
    if let Some(model_evaluator) = self.model_evaluators_by_name.get(model_name) {
      Ok(model_evaluator.evaluate_invocable(invocable_name, input_data))
    } else {
      Err(err_model_evaluator_is_not_deployed(model_name))
    }
  }
  /// Utility function that deletes all definitions in workspace.
  fn clear_definitions(&mut self) {
    self.definitions_by_name.clear();
    self.definitions_by_namespace.clear();
    self.definitions.clear();
  }
  /// Utility function that deletes all model evaluators in workspace.
  fn clear_model_evaluators(&mut self) {
    self.model_evaluators_by_name.clear();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use dmntk_feel::Scope;

  fn assert_state(workspace: &Workspace, state: (usize, usize, usize, usize)) {
    assert_eq!(state.0, workspace.definitions.len());
    assert_eq!(state.1, workspace.definitions_by_namespace.len());
    assert_eq!(state.2, workspace.definitions_by_name.len());
    assert_eq!(state.3, workspace.model_evaluators_by_name.len());
  }

  #[test]
  fn test_states() {
    // create empty workspace, STAGING
    let mut workspace = Workspace::new(None);
    assert_state(&workspace, (0, 0, 0, 0));

    // add one model with definitions, STAGING
    let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001).unwrap();
    assert!(workspace.add(definitions).is_ok());
    assert_state(&workspace, (1, 1, 1, 0));

    // try to add the same model once again, STAGING
    let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001).unwrap();
    assert_eq!(
      Err(err_definitions_with_namespace_already_exists("https://dmntk.io/2_0001")),
      workspace.add(definitions)
    );
    assert_state(&workspace, (1, 1, 1, 0));

    // add another model to workspace, STAGING
    let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0002).unwrap();
    assert!(workspace.add(definitions).is_ok());
    assert_state(&workspace, (2, 2, 2, 0));

    // deploy these two models, DEPLOYED
    assert!(workspace.deploy().is_ok());
    assert_state(&workspace, (2, 2, 2, 2));

    // replace existing model with a new version, STAGING
    let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0002).unwrap();
    assert!(workspace.replace(definitions).is_ok());
    assert_state(&workspace, (2, 2, 2, 0));

    // deploy models, DEPLOYED
    assert!(workspace.deploy().is_ok());
    assert_state(&workspace, (2, 2, 2, 2));

    // remove model from workspace, STAGING
    let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0002).unwrap();
    workspace.remove(definitions.namespace(), definitions.name());
    assert_state(&workspace, (1, 1, 1, 0));

    // clear workspace, STAGING
    workspace.clear();
    assert_state(&workspace, (0, 0, 0, 0));
  }

  #[test]
  fn test_evaluate() {
    // create empty workspace
    let mut workspace = Workspace::new(None);
    assert_state(&workspace, (0, 0, 0, 0));

    // add one model with definitions
    let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001).unwrap();
    assert!(workspace.add(definitions).is_ok());
    assert_state(&workspace, (1, 1, 1, 0));

    // deploy model
    assert!(workspace.deploy().is_ok());
    assert_state(&workspace, (1, 1, 1, 1));

    // evaluate existing model and invocable
    let input_data = dmntk_feel_evaluator::evaluate_context(&Scope::default(), r#"{Full Name: "John Doe"}"#).unwrap();
    let value = workspace
      .evaluate_invocable("compliance-level-2-test-0001", "Greeting Message", &input_data)
      .unwrap();
    assert_eq!(r#""Hello John Doe""#, value.to_string());

    // evaluate non existing model
    let result = workspace.evaluate_invocable("compliance-level-2-test-0002", "Greeting Message", &input_data);
    assert_eq!(Err(err_model_evaluator_is_not_deployed("compliance-level-2-test-0002")), result);

    // evaluate non existing invocable
    let value = workspace
      .evaluate_invocable("compliance-level-2-test-0001", "Good bye message", &input_data)
      .unwrap();
    assert_eq!(r#"null(invocable with name 'Good bye message' not found)"#, value.to_string());
  }
}
