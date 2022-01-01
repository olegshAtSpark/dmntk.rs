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

use crate::errors::*;
use crate::model_evaluator::ModelEvaluator;
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_model::model::{Definitions, DmnElement, NamedElement, RequiredVariable};

/// Evaluates a [BusinessKnowledgeModel] defined by specified name.
pub fn evaluate_business_knowledge_model_by_name(definitions: &Definitions, name: &str, ctx: &FeelContext) -> Result<Value> {
  if let Some(business_knowledge_model) = definitions.business_knowledge_model_by_name(name) {
    let id = business_knowledge_model.id().as_ref().ok_or_else(err_empty_identifier)?;
    let output_variable_name = business_knowledge_model.variable().feel_name().as_ref().ok_or_else(err_empty_feel_name)?;
    let model_evaluator = ModelEvaluator::new(definitions)?;
    let result = model_evaluator.evaluate_business_knowledge_model(id, ctx, output_variable_name);
    Ok(result)
  } else {
    Err(err_business_knowledge_model_with_name_not_found(name))
  }
}
