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

/// Errors related to `FEEL` expression evaluator.
#[derive(Error, Debug)]
pub enum FeelEvaluatorError {
  #[error("expected FEEL context as an input")]
  NotAContext,
  #[error("expected positional or named parameter")]
  ExpectedPositionalOrNamedParameter,
  #[error("expected AstNode::ParameterName, actual node is {0}")]
  ExpectedAstNodeParameterName(String),
  #[error("expected AST node {0}, actual AST node is {1}")]
  ExpectedAstNode(String, String),
  #[error("unexpected AST node in evaluator builder {0}")]
  UnexpectedAstNode(String),
}

impl From<FeelEvaluatorError> for DmntkError {
  fn from(e: FeelEvaluatorError) -> Self {
    DmntkError::new("FeelEvaluatorError", &format!("{}", e))
  }
}

pub fn err_not_a_context() -> DmntkError {
  FeelEvaluatorError::NotAContext.into()
}

pub fn err_expected_positional_or_named_parameter() -> DmntkError {
  FeelEvaluatorError::ExpectedPositionalOrNamedParameter.into()
}

pub fn err_expected_ast_node_parameter_name(s: &str) -> DmntkError {
  FeelEvaluatorError::ExpectedAstNodeParameterName(s.to_string()).into()
}

pub fn err_expected_ast_node(expected: &str, actual: &str) -> DmntkError {
  FeelEvaluatorError::ExpectedAstNode(expected.to_string(), actual.to_string()).into()
}

pub fn err_unexpected_ast_node(s: &str) -> DmntkError {
  FeelEvaluatorError::UnexpectedAstNode(s.to_string()).into()
}
