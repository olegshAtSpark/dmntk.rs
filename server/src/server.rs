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
use actix_web::{error, get, post, web, App, HttpResponse, HttpServer};
use dmntk_common::{DmntkError, Jsonify, Result};
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::Scope;
use dmntk_model::model::NamedElement;
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

impl<T: serde::Serialize> ToString for ResultDto<T> {
  /// Converts [ResultDto] to JSON string.
  fn to_string(&self) -> String {
    serde_json::to_string(self).unwrap_or_else(|_| "json conversion failed".to_string())
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

/// System information structure.
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

/// Result data sent back to caller after adding definitions.
#[derive(Debug, Serialize)]
pub struct AddDefinitionsResult {
  /// Namespace of added definitions.
  #[serde(rename = "namespace")]
  pub namespace: String,
  /// Name of added definitions.
  #[serde(rename = "name")]
  pub name: String,
}

/// Parameters for replacing DMN™ model definitions in workspace.
#[derive(Deserialize)]
pub struct ReplaceDefinitionsParams {
  /// Content of the DMN™ model, encoded in `Base64`.
  #[serde(rename = "content")]
  pub content: Option<String>,
}

/// Parameters for removing DMN™ model definitions from workspace.
#[derive(Deserialize)]
pub struct RemoveDefinitionsParams {
  /// Namespace of the definitions to be removed.
  #[serde(rename = "namespace")]
  pub namespace: Option<String>,
  /// Name of the definitions to be removed.
  #[serde(rename = "name")]
  pub name: Option<String>,
}

/// Operation status sent back to caller after request completion.
#[derive(Debug, Serialize)]
pub struct StatusResult {
  /// Operation status.
  #[serde(rename = "status")]
  pub status: String,
}

/// Parameters for evaluating invocable in DMN™ model definitions.
/// The format of input data is compatible with test cases
/// defined in [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
#[derive(Debug, Deserialize)]
pub struct TckEvaluateParams {
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

/// Parameters for evaluating invocable in DMN™ model definitions.
#[derive(Debug, Deserialize)]
struct EvaluateParams {
  /// Name of the model.
  #[serde(rename = "model")]
  model_name: Option<String>,
  /// Name of the invocable in model.
  #[serde(rename = "invocable")]
  invocable_name: Option<String>,
}

/// Handler for retrieving system information.
#[get("/system/info")]
async fn get_system_info() -> std::io::Result<Json<ResultDto<SystemInfoDto>>> {
  Ok(Json(ResultDto::data(SystemInfoDto::default())))
}

/// Handler for deleting all model definitions from workspace.
#[post("/definitions/clear")]
async fn post_definitions_clear(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<StatusResult>>> {
  if let Ok(mut workspace) = data.workspace.write() {
    match do_clear_definitions(&mut workspace) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_write_lock_failed())))
  }
}

/// Handler for adding model definitions to workspace.
#[post("/definitions/add")]
async fn post_definitions_add(params: Json<AddDefinitionsParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<AddDefinitionsResult>>> {
  if let Ok(mut workspace) = data.workspace.write() {
    match do_add_definitions(&mut workspace, &params.into_inner()) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_write_lock_failed())))
  }
}

/// Handler for replacing model definitions in workspace.
#[post("/definitions/replace")]
async fn post_definitions_replace(params: Json<ReplaceDefinitionsParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<StatusResult>>> {
  if let Ok(mut workspace) = data.workspace.write() {
    match do_replace_definitions(&mut workspace, &params.into_inner()) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_write_lock_failed())))
  }
}

/// Handler for removing model definitions from workspace.
#[post("/definitions/remove")]
async fn post_definitions_remove(params: Json<RemoveDefinitionsParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<StatusResult>>> {
  if let Ok(mut workspace) = data.workspace.write() {
    match do_remove_definitions(&mut workspace, &params.into_inner()) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_write_lock_failed())))
  }
}

/// Handler for deploying model definitions stashed in workspace.
#[post("/definitions/deploy")]
async fn post_definitions_deploy(data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<StatusResult>>> {
  if let Ok(mut workspace) = data.workspace.write() {
    match do_deploy_definitions(&mut workspace) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_write_lock_failed())))
  }
}

/// Handler for evaluating models with input values in format compatible with test cases
/// defined in [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
#[post("/tck/evaluate")]
async fn post_tck_evaluate(params: Json<TckEvaluateParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<OutputNodeDto>>> {
  if let Ok(workspace) = data.workspace.read() {
    match do_evaluate_tck(&workspace, &params.into_inner()) {
      Ok(response) => Ok(Json(ResultDto::data(response))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_workspace_read_lock_failed())))
  }
}

/// Handler for evaluating invocable in model.
///
/// Input values may be defined in `JSON` or `FEEL` context format.
/// Result is always in JSON format.
#[post("/evaluate/{model}/{invocable}")]
async fn post_evaluate(params: web::Path<EvaluateParams>, request_body: String, data: web::Data<ApplicationData>) -> HttpResponse {
  if let Ok(workspace) = data.workspace.read() {
    match do_evaluate(&workspace, &params.into_inner(), &request_body) {
      Ok(value) => HttpResponse::Ok()
        .content_type("application/json")
        .body(format!("{{\"data\":{}}}", value.jsonify())),
      Err(reason) => HttpResponse::Ok()
        .content_type("application/json")
        .body(ResultDto::<String>::error(reason).to_string()),
    }
  } else {
    HttpResponse::Ok()
      .content_type("application/json")
      .body(ResultDto::<String>::error(err_workspace_read_lock_failed()).to_string())
  }
}

/// Handler for 404 errors.
async fn not_found() -> std::io::Result<Json<ResultDto<()>>> {
  Ok(Json(ResultDto::error(err_endpoint_not_found())))
}

/// Starts the server.
pub async fn start_server(opt_host: Option<String>, opt_port: Option<String>) -> std::io::Result<()> {
  // create workspace and load all definitions
  let workspace = Workspace::new(None);
  let application_data = web::Data::new(ApplicationData {
    workspace: RwLock::new(workspace),
  });
  let address = get_server_address(opt_host, opt_port);
  println!("dmntk {}", address);
  HttpServer::new(move || {
    App::new()
      .app_data(application_data.clone())
      .app_data(web::JsonConfig::default().limit(4 * 1024 * 1024).error_handler(|err, _| {
        error::InternalError::from_response(
          "",
          HttpResponse::BadRequest()
            .content_type("application/json")
            .body(ResultDto::<String>::error(err_internal_error(&format!("{:?}", err))).to_string()),
        )
        .into()
      }))
      .service(get_system_info)
      .service(post_definitions_clear)
      .service(post_definitions_add)
      .service(post_definitions_replace)
      .service(post_definitions_remove)
      .service(post_definitions_deploy)
      .service(post_tck_evaluate)
      .service(post_evaluate)
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
#[inline(always)]
fn do_clear_definitions(workspace: &mut Workspace) -> Result<StatusResult> {
  workspace.clear();
  Ok(StatusResult {
    status: "definitions cleared".to_string(),
  })
}

///
#[inline(always)]
fn do_add_definitions(workspace: &mut Workspace, params: &AddDefinitionsParams) -> Result<AddDefinitionsResult> {
  if let Some(content) = &params.content {
    if let Ok(bytes) = base64::decode(content) {
      if let Ok(xml) = String::from_utf8(bytes) {
        match dmntk_model::parse(&xml) {
          Ok(definitions) => {
            let namespace = definitions.namespace().to_string();
            let name = definitions.name().to_string();
            workspace.add(definitions)?;
            Ok(AddDefinitionsResult { namespace, name })
          }
          Err(reason) => Err(reason),
        }
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

///
#[inline(always)]
fn do_replace_definitions(workspace: &mut Workspace, params: &ReplaceDefinitionsParams) -> Result<StatusResult> {
  if let Some(content) = &params.content {
    if let Ok(bytes) = base64::decode(content) {
      if let Ok(xml) = String::from_utf8(bytes) {
        match dmntk_model::parse(&xml) {
          Ok(definitions) => {
            workspace.add(definitions)?;
            Ok(StatusResult {
              status: "definitions replaced".to_string(),
            })
          }
          Err(reason) => Err(reason),
        }
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

///
#[inline(always)]
fn do_remove_definitions(workspace: &mut Workspace, params: &RemoveDefinitionsParams) -> Result<StatusResult> {
  if let Some(namespace) = &params.namespace {
    if let Some(name) = &params.name {
      workspace.remove(namespace, name);
      Ok(StatusResult {
        status: "definitions removed".to_string(),
      })
    } else {
      Err(err_missing_parameter("name"))
    }
  } else {
    Err(err_missing_parameter("namespace"))
  }
}

///
#[inline(always)]
fn do_deploy_definitions(workspace: &mut Workspace) -> Result<StatusResult> {
  match workspace.deploy() {
    Ok(()) => Ok(StatusResult {
      status: "definitions deployed".to_string(),
    }),
    Err(reason) => Err(reason),
  }
}

/// Evaluates the invocable in model and returns the result.
/// Input and output data format is compatible with
/// [Technology Compatibility Kit for DMN standard](https://github.com/dmn-tck/tck).
#[inline(always)]
fn do_evaluate_tck(workspace: &Workspace, params: &TckEvaluateParams) -> Result<OutputNodeDto, DmntkError> {
  if let Some(model_name) = &params.model_name {
    if let Some(invocable_name) = &params.invocable_name {
      if let Some(input_values) = &params.input_values {
        // convert input values into FEEL context
        let input_data = FeelContext::try_from(WrappedValue::try_from(input_values)?.0)?;
        // evaluate artifact with specified name
        workspace.evaluate_invocable(model_name, invocable_name, &input_data)?.try_into()
      } else {
        Err(err_missing_parameter("input"))
      }
    } else {
      Err(err_missing_parameter("invocable"))
    }
  } else {
    Err(err_missing_parameter("model"))
  }
}

/// Evaluates the artifact specified in parameters and returns the result.
#[inline(always)]
fn do_evaluate(workspace: &Workspace, params: &EvaluateParams, input: &str) -> Result<Value, DmntkError> {
  if let Some(model_name) = &params.model_name {
    if let Some(invocable_name) = &params.invocable_name {
      let input_data = dmntk_evaluator::evaluate_context(&Scope::default(), input)?;
      let value = workspace.evaluate_invocable(model_name, invocable_name, &input_data)?;
      Ok(value)
    } else {
      Err(err_missing_parameter("invocable"))
    }
  } else {
    Err(err_missing_parameter("model"))
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test() {
    assert_eq!(
      r#"{"errors":[{"details":"ServerError: unknown"}]}"#,
      ResultDto::<String>::error(err_internal_error("unknown")).to_string()
    );
  }
}
