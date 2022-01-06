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

use crate::dto::{InputNodeDto, OutputNodeDto, WrappedValue};
use crate::errors::*;
use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpServer};
use dmntk_common::{DmntkError, Result};
use dmntk_feel::context::FeelContext;
use dmntk_workspace::Workspace;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::RwLock;

const DMNTK_NAME: &str = env!("CARGO_PKG_NAME");
const DMNTK_VERSION: &str = env!("CARGO_PKG_VERSION");
const DMNTK_COPYRIGHT: &str = env!("CARGO_PKG_AUTHORS");

/// Shared workspace with decision model definitions.
struct ApplicationData {
  workspace: RwLock<Workspace>,
}

/// Data transfer object for an error.
#[derive(Serialize)]
pub struct ErrorDto {
  /// Error details.
  #[serde(rename = "details")]
  details: String,
}

/// Data transfer object for a result.
#[derive(Serialize)]
pub struct ResultDto<T> {
  /// Result containing data.
  #[serde(rename = "data", skip_serializing_if = "Option::is_none")]
  data: Option<T>,
  /// Result containing errors.
  #[serde(rename = "errors", skip_serializing_if = "Vec::is_empty")]
  errors: Vec<ErrorDto>,
}

impl<T> Default for ResultDto<T> {
  /// Creates default result structure.
  fn default() -> Self {
    Self { data: None, errors: vec![] }
  }
}

impl<T> ResultDto<T> {
  /// Creates [ResultDto] with some data inside.
  pub fn data(d: T) -> ResultDto<T> {
    ResultDto {
      data: Some(d),
      ..Default::default()
    }
  }
  /// Creates [ResultDto] with single error inside.
  pub fn error(err: DmntkError) -> ResultDto<T> {
    ResultDto {
      errors: vec![ErrorDto { details: format!("{}", err) }],
      ..Default::default()
    }
  }
}

/// Data transfer object for system information.
#[derive(Serialize)]
pub struct SystemInfoDto {
  /// System name.
  #[serde(rename = "name")]
  name: String,
  /// System version.
  #[serde(rename = "version")]
  version: String,
  /// Legal notice.
  #[serde(rename = "copyright")]
  copyright: String,
}

impl Default for SystemInfoDto {
  /// Creates default system information structure.
  fn default() -> Self {
    Self {
      name: DMNTK_NAME.to_string(),
      version: DMNTK_VERSION.to_string(),
      copyright: DMNTK_COPYRIGHT.to_string(),
    }
  }
}

/// Parameters for adding DMN™ model definitions to workspace.
#[derive(Deserialize)]
pub struct AddDefinitionsParams {
  /// Content of the DMN™ model, encoded in `Base64`.
  #[serde(rename = "content")]
  pub content: Option<String>,
}

/// Operation status sent back to caller after request completion.
#[derive(Debug, Serialize)]
pub struct StatusResult {
  /// Operation status.
  #[serde(rename = "status")]
  pub status: String,
}

/// Parameters for evaluating invocable in DMN™ model definitions.
#[derive(Debug, Deserialize)]
pub struct EvaluateParams {
  /// Name of the model where the invocable will be searched.
  #[serde(rename = "model")]
  model_name: Option<String>,
  /// Name of the invocable to be evaluated.
  #[serde(rename = "invocable")]
  invocable_name: Option<String>,
  /// Collection of input values.
  #[serde(rename = "input")]
  input_values: Option<Vec<InputNodeDto>>,
}

/// Handler for system information.
#[get("/info")]
async fn system_info() -> std::io::Result<Json<ResultDto<SystemInfoDto>>> {
  Ok(Json(ResultDto::data(SystemInfoDto::default())))
}

/// Handler for adding DMN™ definitions to workspace.
#[post("/definitions/add")]
async fn add_definitions(params: Json<AddDefinitionsParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<StatusResult>>> {
  if let Ok(mut workspace) = data.workspace.write() {
    match add_definitions_to_workspace(&mut workspace, &params.into_inner()) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_write_lock_failed())))
  }
}

/// Handler for replacing DMN™ definitions in workspace.
#[post("/definitions/replace")]
async fn replace_definitions(params: Json<AddDefinitionsParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<StatusResult>>> {
  if let Ok(mut workspace) = data.workspace.write() {
    match add_definitions_to_workspace(&mut workspace, &params.into_inner()) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_write_lock_failed())))
  }
}

/// Handler for evaluating artifacts.
#[post("/evaluate")]
async fn evaluate_invocable(params: Json<EvaluateParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<OutputNodeDto>>> {
  if let Ok(workspace) = data.workspace.read() {
    match evaluate_artifact_in_workspace(&workspace, &params.into_inner()) {
      Ok(response) => Ok(Json(ResultDto::data(response))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_read_lock_failed())))
  }
}

/// Handler for 404 errors.
async fn not_found() -> std::io::Result<Json<ResultDto<()>>> {
  Ok(Json(ResultDto::error(err_endpoint_not_found())))
}

/// Starts the server.
pub async fn start_server(opt_host: Option<String>, opt_port: Option<String>) -> std::io::Result<()> {
  // create workspace and load all definitions
  let workspace = Workspace::new();
  let application_data = web::Data::new(ApplicationData {
    workspace: RwLock::new(workspace),
  });
  let address = get_server_address(opt_host, opt_port);
  println!("dmntk {}", address);
  HttpServer::new(move || {
    App::new()
      .app_data(application_data.clone())
      .app_data(web::JsonConfig::default().limit(4 * 1024 * 1024))
      .service(system_info)
      .service(add_definitions)
      .service(replace_definitions)
      .service(evaluate_invocable)
      .default_service(web::route().to(not_found))
  })
  .bind(address)?
  .run()
  .await
}

///
fn get_server_address(opt_host: Option<String>, opt_port: Option<String>) -> String {
  let mut host: String = "127.0.0.1".to_string();
  if let Some(h) = opt_host {
    host = h;
  }
  let mut port: u16 = 22022;
  if let Some(p_str) = opt_port {
    if let Ok(p) = u16::from_str(&p_str) {
      port = p;
    }
  }
  format!("{}:{}", host, port)
}

///
fn add_definitions_to_workspace(workspace: &mut Workspace, params: &AddDefinitionsParams) -> Result<StatusResult> {
  if let Some(content) = &params.content {
    if let Ok(bytes) = base64::decode(content) {
      if let Ok(xml) = String::from_utf8(bytes) {
        let definitions = dmntk_model::parse(&xml)?;
        workspace.add(definitions)?;
        Ok(StatusResult { status: "added".to_string() })
      } else {
        Err(err_invalid_utf8_content())
      }
    } else {
      Err(err_invalid_base64_encoding())
    }
  } else {
    Err(err_missing_parameter("content"))
  }
}

/// Evaluates the artifact specified in parameters and returns the result.
fn evaluate_artifact_in_workspace(workspace: &Workspace, params: &EvaluateParams) -> Result<OutputNodeDto, DmntkError> {
  if let Some(model_name) = &params.model_name {
    if let Some(invocable_name) = &params.invocable_name {
      if let Some(input_values) = &params.input_values {
        // convert input values data into input data as a context
        let input_data = FeelContext::try_from(WrappedValue::try_from(input_values)?.0)?;
        // evaluate artifact with specified name
        workspace.evaluate_invocable(model_name, invocable_name, &input_data)?.try_into()
      } else {
        Err(err_missing_parameter("input"))
      }
    } else {
      Err(err_missing_parameter("name"))
    }
  } else {
    Err(err_missing_parameter("tag"))
  }
}
