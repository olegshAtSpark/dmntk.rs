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

use crate::dto::{InputNodeDto, OutputNodeDto, WrappedValue};
use crate::errors::*;
use actix_web::web::Json;
use actix_web::{get, post, web, App, HttpServer};
use dmntk_common::{DmntkError, Result};
use dmntk_feel::context::FeelContext;
use dmntk_workspace::Workspace;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use std::sync::Mutex;

const DMNTK_NAME: &str = env!("CARGO_PKG_NAME");
const DMNTK_VERSION: &str = env!("CARGO_PKG_VERSION");
const DMNTK_COPYRIGHT: &str = env!("CARGO_PKG_AUTHORS");

/// Shared workspace with decision model definitions.
struct ApplicationData {
  workspace: Mutex<Workspace>,
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

/// Parameters for deploying DMN™ model definitions from *.dmn files.
#[derive(Deserialize)]
pub struct DeployParams {
  /// The URI of the deployed DMN™ model.
  #[serde(rename = "source")]
  pub source: Option<String>,
  /// The content of the DMN™ model, encoded in `Base64`.
  #[serde(rename = "content")]
  pub content: Option<String>,
  /// Unique tag associated with the deployed model.
  #[serde(rename = "tag")]
  pub tag: Option<String>,
}

/// Output sent do caller after successful deployment.
#[derive(Serialize, Debug)]
pub struct DeployResult {
  /// Unique name of the deployed model.
  #[serde(rename = "name")]
  pub name: Option<String>,
  /// Unique identifier of the deployed model.
  #[serde(rename = "id")]
  pub id: Option<String>,
  /// Unique tag associated with the deployed model.
  #[serde(rename = "tag")]
  pub tag: Option<String>,
}

/// Parameters for evaluating decision artefact.
#[derive(Debug, Deserialize)]
pub struct EvaluateParams {
  /// Tag of the model where the artifact will be searched.
  #[serde(rename = "tag")]
  model_tag: Option<String>,
  /// Type of the artifact to be evaluated.
  #[serde(rename = "artifact")]
  artifact_type: Option<String>,
  /// Name of the artifact to be evaluated.
  #[serde(rename = "name")]
  artifact_name: Option<String>,
  /// Collection of input values.
  #[serde(rename = "input")]
  input_values: Option<Vec<InputNodeDto>>,
}

/// Handler for system information.
#[get("/info")]
async fn system_info() -> std::io::Result<Json<ResultDto<SystemInfoDto>>> {
  Ok(Json(ResultDto::data(SystemInfoDto::default())))
}

/// Handler for deploying DMN™ definitions.
#[post("/deploy-definitions")]
async fn deploy_definitions(params: Json<DeployParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<DeployResult>>> {
  if let Ok(mut workspace) = data.workspace.lock() {
    match deploy_definitions_in_workspace(&mut workspace, &params.into_inner(), false) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_can_not_lock_workspace())))
  }
}

/// Handler for redeploying DMN™ definitions.
#[post("/redeploy-definitions")]
async fn redeploy_definitions(params: Json<DeployParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<DeployResult>>> {
  if let Ok(mut workspace) = data.workspace.lock() {
    match deploy_definitions_in_workspace(&mut workspace, &params.into_inner(), true) {
      Ok(result) => Ok(Json(ResultDto::data(result))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_can_not_lock_workspace())))
  }
}

/// Handler for evaluating artifacts.
#[post("/evl")]
async fn evaluate_artifact(params: Json<EvaluateParams>, data: web::Data<ApplicationData>) -> std::io::Result<Json<ResultDto<OutputNodeDto>>> {
  if let Ok(mut workspace) = data.workspace.lock() {
    match evaluate_artifact_in_workspace(&mut workspace, &params.into_inner()) {
      Ok(response) => Ok(Json(ResultDto::data(response))),
      Err(reason) => Ok(Json(ResultDto::error(reason))),
    }
  } else {
    Ok(Json(ResultDto::error(err_can_not_lock_workspace())))
  }
}

/// Handler for 404 errors.
async fn not_found() -> std::io::Result<Json<ResultDto<()>>> {
  Ok(Json(ResultDto::error(err_endpoint_not_found())))
}

/// Starts the server.
pub async fn start_server(opt_host: Option<String>, opt_port: Option<String>) -> std::io::Result<()> {
  // create workspace and load all definitions
  let workspace = Workspace::default();
  let application_data = web::Data::new(ApplicationData {
    workspace: Mutex::new(workspace),
  });
  let address = get_server_address(opt_host, opt_port);
  println!("dmntk server {}", address);
  HttpServer::new(move || {
    App::new()
      .app_data(application_data.clone())
      .app_data(web::JsonConfig::default().limit(4 * 1024 * 1024))
      .service(system_info)
      .service(deploy_definitions)
      .service(redeploy_definitions)
      .service(evaluate_artifact)
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
fn deploy_definitions_in_workspace(workspace: &mut Workspace, params: &DeployParams, replace_existing: bool) -> Result<DeployResult> {
  if let Some(source) = &params.source {
    if let Some(content) = &params.content {
      if let Some(tag) = &params.tag {
        if let Ok(bytes) = base64::decode(content) {
          if let Ok(xml) = String::from_utf8(bytes) {
            let definitions = dmntk_model::parse(&xml, source)?;
            let (name, id, tag) = workspace.deploy_definitions(tag, definitions, replace_existing)?;
            Ok(DeployResult {
              name: Some(name),
              id,
              tag: Some(tag),
            })
          } else {
            Err(err_invalid_utf8_content())
          }
        } else {
          Err(err_invalid_base64_encoding())
        }
      } else {
        Err(err_missing_parameter("tag"))
      }
    } else {
      Err(err_missing_parameter("content"))
    }
  } else {
    Err(err_missing_parameter("source"))
  }
}

/// Evaluates the artifact specified in parameters and returns the result.
fn evaluate_artifact_in_workspace(workspace: &mut Workspace, params: &EvaluateParams) -> Result<OutputNodeDto, DmntkError> {
  if let Some(model_tag) = &params.model_tag {
    if let Some(artifact_type) = &params.artifact_type {
      if let Some(artifact_name) = &params.artifact_name {
        if let Some(input_values) = &params.input_values {
          // convert input data into context
          let ctx = FeelContext::try_from(WrappedValue::try_from(input_values)?.0)?;
          // evaluate artifact with specified name
          workspace.evaluate_artifact(&ctx, model_tag, artifact_type, artifact_name)?.try_into()
        } else {
          Err(err_missing_parameter("input"))
        }
      } else {
        Err(err_missing_parameter("name"))
      }
    } else {
      Err(err_missing_parameter("artifact"))
    }
  } else {
    Err(err_missing_parameter("tag"))
  }
}
