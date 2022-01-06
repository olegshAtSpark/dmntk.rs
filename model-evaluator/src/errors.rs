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

use dmntk_common::DmntkError;
use dmntk_feel::FeelType;

/// Errors related to model evaluation.
#[derive(Error, Debug)]
pub enum ModelEvaluatorError {
  #[error("business knowledge model with name `{0}` was not found")]
  BusinessKnowledgeModelWithNameNotFound(String),
  #[error("business knowledge model with reference `{0}` was not found")]
  BusinessKnowledgeModelWithReferenceNotFound(String),
  #[error("decision with name `{0}` was not found")]
  DecisionWithNameNotFound(String),
  #[error("decision service with name `{0}` was not found")]
  DecisionServiceWithNameNotFound(String),
  #[error("input data with identifier `{0}` has no type reference definition")]
  InputDataWithoutTypeReference(String),
  #[error("empty FEEL name")]
  EmptyFeelName,
  #[error("empty identifier")]
  EmptyIdentifier,
  #[error("empty decision logic in decision")]
  EmptyLiteralExpression,
  #[error("empty literal expression")]
  EmptyDecisionLogic,
  #[error("empty encapsulated logic in business knowledge model")]
  EmptyEncapsulatedLogic,
  #[error("invalid item definition type for `{0}`")]
  InvalidItemDefinitionType(String),
  #[error("unsupported FEEL type: {0}")]
  UnsupportedFeelType(String),
  #[error("empty FEEL type")]
  EmptyFeelType,
  #[error("empty reference")]
  EmptyReference,
  #[error("empty function definition body")]
  EmptyFunctionBody,
  #[error("empty value expression")]
  EmptyValueExpression,
  #[error("read lock failed with reason '{0}'")]
  ReadLockFailed(String),
  #[error("write lock failed with reason '{0}'")]
  WriteLockFailed(String),
}

impl From<ModelEvaluatorError> for DmntkError {
  fn from(e: ModelEvaluatorError) -> Self {
    DmntkError::new("ModelEvaluatorError", &e.to_string())
  }
}

pub fn err_business_knowledge_model_with_name_not_found(name: &str) -> DmntkError {
  ModelEvaluatorError::BusinessKnowledgeModelWithNameNotFound(name.to_string()).into()
}

pub fn err_business_knowledge_model_with_reference_not_found(reference: &str) -> DmntkError {
  ModelEvaluatorError::BusinessKnowledgeModelWithReferenceNotFound(reference.to_string()).into()
}

pub fn err_decision_with_name_not_found(name: &str) -> DmntkError {
  ModelEvaluatorError::DecisionWithNameNotFound(name.to_string()).into()
}

pub fn err_decision_service_with_name_not_found(name: &str) -> DmntkError {
  ModelEvaluatorError::DecisionServiceWithNameNotFound(name.to_string()).into()
}

pub fn err_input_data_without_type_reference(s: &str) -> DmntkError {
  ModelEvaluatorError::InputDataWithoutTypeReference(s.to_string()).into()
}

pub fn err_empty_feel_name() -> DmntkError {
  ModelEvaluatorError::EmptyFeelName.into()
}

pub fn err_empty_identifier() -> DmntkError {
  ModelEvaluatorError::EmptyIdentifier.into()
}

pub fn err_empty_literal_expression() -> DmntkError {
  ModelEvaluatorError::EmptyLiteralExpression.into()
}

pub fn err_empty_decision_logic() -> DmntkError {
  ModelEvaluatorError::EmptyDecisionLogic.into()
}

pub fn err_empty_encapsulated_logic() -> DmntkError {
  ModelEvaluatorError::EmptyEncapsulatedLogic.into()
}

pub fn err_invalid_item_definition_type(s: &str) -> DmntkError {
  ModelEvaluatorError::InvalidItemDefinitionType(s.to_string()).into()
}

pub fn err_unsupported_feel_type(feel_type: FeelType) -> DmntkError {
  ModelEvaluatorError::UnsupportedFeelType(feel_type.to_string()).into()
}

pub fn err_empty_feel_type() -> DmntkError {
  ModelEvaluatorError::EmptyFeelType.into()
}

pub fn err_empty_reference() -> DmntkError {
  ModelEvaluatorError::EmptyReference.into()
}

pub fn err_empty_function_body() -> DmntkError {
  ModelEvaluatorError::EmptyFunctionBody.into()
}

pub fn err_empty_value_expression() -> DmntkError {
  ModelEvaluatorError::EmptyValueExpression.into()
}

pub fn err_read_lock_failed(reason: impl ToString) -> DmntkError {
  ModelEvaluatorError::ReadLockFailed(reason.to_string()).into()
}

pub fn err_write_lock_failed(reason: impl ToString) -> DmntkError {
  ModelEvaluatorError::WriteLockFailed(reason.to_string()).into()
}
