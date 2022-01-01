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

//! Errors reported by the workspace.

use dmntk_common::DmntkError;
use thiserror::Error;

/// Errors related with operating on workspaces.
#[derive(Error, Debug)]
pub enum WorkspaceError {
  #[error("endpoint not found")]
  EndpointNotFound,
  #[error("missing parameter '{0}'")]
  MissingParameter(String),
  #[error("invalid Base64 encoding")]
  InvalidBase64Encoding,
  #[error("invalid UTF-8 content")]
  InvalidUtf8Content,
  #[error("can not lock workspace")]
  CanNotLockWorkspace,
}

impl From<WorkspaceError> for DmntkError {
  fn from(e: WorkspaceError) -> Self {
    DmntkError::new("WorkspaceError", &e.to_string())
  }
}

pub fn err_endpoint_not_found() -> DmntkError {
  WorkspaceError::EndpointNotFound.into()
}

pub fn err_missing_parameter(name: &str) -> DmntkError {
  WorkspaceError::MissingParameter(name.to_string()).into()
}

pub fn err_invalid_base64_encoding() -> DmntkError {
  WorkspaceError::InvalidBase64Encoding.into()
}

pub fn err_invalid_utf8_content() -> DmntkError {
  WorkspaceError::InvalidUtf8Content.into()
}

pub fn err_can_not_lock_workspace() -> DmntkError {
  WorkspaceError::CanNotLockWorkspace.into()
}
