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

//! Container for deployed DMN™ models.
//!
//! ### Remarks
//!
//! Importing rules require, that the `namespace` attribute in [Definitions] is globally unique.
//! We assume, that attributes `Definitions.namespace` and `Definitions.name` are both
//! unique inside [Workspace]. In consequence, the same [Definitions] can be accessed using
//! its either a `namespace` or `name` attribute, so there will be an error reported,
//! when two definitions deployed in a single workspace have the same `namespace` or `name`.

use crate::errors::*;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_model::model::{Definitions, DmnElement, NamedElement};
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default)]
pub struct Workspace {
  /// Collection of all [Definitions](Definition) deployed in this [Workspace].
  definitions: Vec<Arc<Definitions>>,
  /// Map of [Definitions](Definition) indexed by `Definitions.namespace` attribute.
  definitions_by_namespace: HashMap<String, Arc<Definitions>>,
  /// Map of [Definitions](Definition) indexed by `Definition.name` attribute.
  definitions_by_name: HashMap<String, Arc<Definitions>>,
  //TODO remove
  /// Mapping of definitions by definition's tag.
  definitions_by_tag: HashMap<String, Arc<Definitions>>, //TODO remove
  /// Mapping of definitions by definition's identifier.
  definitions_by_id: HashMap<String, Arc<Definitions>>, //TODO remove
}

impl Workspace {
  ///
  pub fn append_definitions(&mut self, tag: &str, definitions: Definitions) -> Result<(String, Option<String>, String)> {
    let definitions_namespace = definitions.namespace().to_string();
    if self.definitions_by_namespace.contains_key(&definitions_namespace) {
      return Err(err_definitions_with_namespace_already_exists(&definitions_namespace));
    }
    let definitions_name = definitions.name().to_string();
    let definitions_id = definitions.id().clone();
    let definitions_arc = Arc::new(definitions);
    if self.definitions_by_name.contains_key(&definitions_name) {
      return Err(err_definitions_with_name_already_exists(&definitions_name));
    }
    self.definitions_by_name.insert(definitions_name.to_string(), Arc::clone(&definitions_arc));
    if let Some(id) = &definitions_id {
      if self.definitions_by_id.contains_key(id) {
        return Err(err_definitions_with_id_already_exists(id.clone()));
      }
      self.definitions_by_id.insert(id.clone(), Arc::clone(&definitions_arc));
    }
    if self.definitions_by_tag.contains_key(tag) {
      return Err(err_definitions_with_tag_already_exists(tag.to_string()));
    }
    self.definitions_by_tag.insert(tag.to_string(), Arc::clone(&definitions_arc));
    self.definitions.push(definitions_arc);
    Ok((definitions_name, definitions_id, tag.to_string()))
  }
  ///
  pub fn replace_definitions(&mut self, tag: &str, definitions: Definitions) -> Result<(String, Option<String>, String)> {
    if let Some(index) = self.definitions.iter().position(|d| d.name() == definitions.name()) {
      let removed = self.definitions.remove(index);
      self.definitions_by_name.remove(&removed.name().to_string());
      if let Some(id) = removed.id() {
        self.definitions_by_id.remove(id);
      }
      self.definitions_by_tag.remove(&tag.to_string());
    }
    self.append_definitions(tag, definitions)
  }
  ///
  pub fn get_by_tag(&self, tag: &str) -> Option<&Definitions> {
    if let Some(definitions) = self.definitions_by_tag.get(tag) {
      Some(definitions)
    } else {
      None
    }
  }
  /// Deploys the definitions in workspace.
  /// After successful deployment, the following tuple is returned `(name, id, tag)`.
  pub fn deploy_definitions(&mut self, tag: &str, definitions: Definitions, replace_existing: bool) -> Result<(String, Option<String>, String)> {
    if replace_existing {
      self.replace_definitions(tag, definitions)
    } else {
      self.append_definitions(tag, definitions)
    }
  }
  /// Evaluates an artifact.
  pub fn evaluate_artifact(&self, ctx: &FeelContext, model_tag: &str, artifact_type: &str, artifact_name: &str) -> Result<Value> {
    if let Some(definitions) = self.get_by_tag(model_tag) {
      match artifact_type {
        "decision" => dmntk_evaluator::evaluate_decision_by_name(definitions, artifact_name, ctx),
        "bkm" => dmntk_evaluator::evaluate_business_knowledge_model_by_name(definitions, artifact_name, ctx),
        "decisionService" => dmntk_evaluator::eval_decision_service_by_name(definitions, artifact_name, ctx),
        _ => Err(err_invalid_invoked_artifact_name(artifact_type.to_string())),
      }
    } else {
      Err(err_artifact_not_found(artifact_type.to_owned(), artifact_name.to_owned()))
    }
  }
}
