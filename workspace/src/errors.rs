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

//! Errors reported by workspace.

use dmntk_common::DmntkError;

/// Errors reported by workspace.
#[derive(Error, Debug)]
enum WorkspaceError {
  #[error("model evaluator for definitions '{0}' is not deployed")]
  ModelEvaluatorNotDeployed(String),
  #[error("definitions with namespace '{0}' already exist in workspace")]
  DefinitionsWithNamespaceAlreadyExist(String),
  #[error("definitions with name '{0}' already exist in workspace")]
  DefinitionsWithNameAlreadyExist(String),
}

impl From<WorkspaceError> for DmntkError {
  /// Converts a workspace error into [DmntkError].
  fn from(e: WorkspaceError) -> Self {
    DmntkError::new("WorkspaceError", &e.to_string())
  }
}

pub fn err_model_evaluator_not_deployed(definitions_name: &str) -> DmntkError {
  WorkspaceError::ModelEvaluatorNotDeployed(definitions_name.to_string()).into()
}

pub fn err_definitions_with_namespace_already_exists(definitions_namespace: &str) -> DmntkError {
  WorkspaceError::DefinitionsWithNamespaceAlreadyExist(definitions_namespace.to_string()).into()
}

pub fn err_definitions_with_name_already_exists(definitions_name: &str) -> DmntkError {
  WorkspaceError::DefinitionsWithNameAlreadyExist(definitions_name.to_string()).into()
}
