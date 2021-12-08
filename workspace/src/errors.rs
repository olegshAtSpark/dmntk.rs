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

//! Errors reported by the workspace.

use dmntk_common::DmntkError;

/// Errors related with operating on workspaces.
#[derive(Error, Debug)]
pub enum WorkspaceError {
  #[error("artifact '{0}' with name '{1}' was not found")]
  ArtifactNotFound(String, String),
  #[error("'{0}' is not a valid name of invoked artifact")]
  InvalidInvokedArtifactName(String),
  #[error("definitions with name '{0}' already exist in workspace")]
  DefinitionsWithNameAlreadyExist(String),
  #[error("definitions with identifier '{0}' already exist in workspace")]
  DefinitionsWithIdAlreadyExist(String),
  #[error("definitions with tag '{0}' already exist in workspace")]
  DefinitionsWithTagAlreadyExist(String),
}

impl From<WorkspaceError> for DmntkError {
  fn from(e: WorkspaceError) -> Self {
    DmntkError::new("WorkspaceError", &e.to_string())
  }
}

pub fn err_artifact_not_found(artifact: String, name: String) -> DmntkError {
  WorkspaceError::ArtifactNotFound(artifact, name).into()
}

pub fn err_invalid_invoked_artifact_name(name: String) -> DmntkError {
  WorkspaceError::InvalidInvokedArtifactName(name).into()
}

pub fn err_definitions_with_name_already_exists(name: String) -> DmntkError {
  WorkspaceError::DefinitionsWithNameAlreadyExist(name).into()
}

pub fn err_definitions_with_id_already_exists(name: String) -> DmntkError {
  WorkspaceError::DefinitionsWithIdAlreadyExist(name).into()
}

pub fn err_definitions_with_tag_already_exists(name: String) -> DmntkError {
  WorkspaceError::DefinitionsWithTagAlreadyExist(name).into()
}
