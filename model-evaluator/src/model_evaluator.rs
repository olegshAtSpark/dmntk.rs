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

use crate::builders::{
  BusinessKnowledgeModelEvaluator, DecisionEvaluator, DecisionServiceEvaluator, InputDataContextEvaluator, InputDataEvaluator, ItemDefinitionContextEvaluator,
  ItemDefinitionEvaluator, ItemDefinitionTypeEvaluator,
};
use dmntk_common::Result;
use dmntk_feel::context::FeelContext;
use dmntk_feel::values::Value;
use dmntk_feel::{value_null, Name};
use dmntk_model::model::Definitions;
use std::cell::{Ref, RefCell};
use std::rc::Rc;

/*
   # let input_data_evaluator = InputDataEvaluator::new(definitions)?;
   # let input_data_context_evaluator = InputDataContextEvaluator::new(definitions)?;
   # let item_definition_evaluator = ItemDefinitionEvaluator::new(definitions)?;
   let item_definition_context_evaluator = ItemDefinitionContextEvaluator::new(definitions)?;
   let item_definition_type_evaluator = ItemDefinitionTypeEvaluator::new(definitions)?;
   let business_knowledge_model_evaluator = BusinessKnowledgeModelEvaluator::new(definitions, &item_definition_type_evaluator)?;
   let decision_evaluator = DecisionEvaluator::new(      definitions,      &input_data_context_evaluator,      &item_definition_context_evaluator,      &item_definition_type_evaluator,    )?;
   let decision_service_evaluator = DecisionServiceEvaluator::new(definitions, &item_definition_type_evaluator, &input_data_evaluator, &decision_evaluator)?;
*/

///
#[derive(Default)]
pub struct ModelEvaluator {
  /// Input data evaluator.
  input_data_evaluator: RefCell<InputDataEvaluator>,
  /// Input data context evaluator.
  input_data_context_evaluator: RefCell<InputDataContextEvaluator>,
  /// Item definition evaluator.
  item_definition_evaluator: RefCell<ItemDefinitionEvaluator>,
  /// Item definition context evaluator.
  item_definition_context_evaluator: RefCell<ItemDefinitionContextEvaluator>,
  ///Item definition type evaluator.
  item_definition_type_evaluator: RefCell<ItemDefinitionTypeEvaluator>,
  /// Business knowledge model evaluator.
  business_knowledge_model_evaluator: RefCell<BusinessKnowledgeModelEvaluator>,
  /// Decision evaluator.
  decision_evaluator: RefCell<DecisionEvaluator>,
  /// Decision service evaluator.
  decision_service_evaluator: RefCell<DecisionServiceEvaluator>,
}

impl ModelEvaluator {
  /// Creates an instance of [ModelEvaluator].
  pub fn new(definitions: &Definitions) -> Result<Rc<Self>> {
    let model_evaluator = Rc::new(ModelEvaluator::default());
    model_evaluator.input_data_evaluator.borrow_mut().build(definitions)?;
    model_evaluator.input_data_context_evaluator.borrow_mut().build(definitions)?;
    model_evaluator.item_definition_evaluator.borrow_mut().build(definitions)?;
    model_evaluator.item_definition_context_evaluator.borrow_mut().build(definitions)?;
    model_evaluator.item_definition_type_evaluator.borrow_mut().build(definitions)?;
    model_evaluator
      .business_knowledge_model_evaluator
      .borrow_mut()
      .build(definitions, &model_evaluator)?;
    model_evaluator.decision_evaluator.borrow_mut().build(definitions, &model_evaluator)?;
    model_evaluator
      .decision_service_evaluator
      .borrow_mut()
      .build(definitions, Rc::clone(&model_evaluator))?;
    Ok(model_evaluator)
  }
  ///
  pub fn input_data_evaluator(&self) -> Ref<InputDataEvaluator> {
    self.input_data_evaluator.borrow()
  }
  ///
  pub fn input_data_context_evaluator(&self) -> Ref<InputDataContextEvaluator> {
    self.input_data_context_evaluator.borrow()
  }
  ///
  pub fn item_definition_context_evaluator(&self) -> Ref<ItemDefinitionContextEvaluator> {
    self.item_definition_context_evaluator.borrow()
  }
  ///
  pub fn item_definition_evaluator(&self) -> Ref<ItemDefinitionEvaluator> {
    self.item_definition_evaluator.borrow()
  }
  ///
  pub fn item_definition_type_evaluator(&self) -> Ref<ItemDefinitionTypeEvaluator> {
    self.item_definition_type_evaluator.borrow()
  }
  ///
  pub fn business_knowledge_model_evaluator(&self) -> Ref<BusinessKnowledgeModelEvaluator> {
    self.business_knowledge_model_evaluator.borrow()
  }
  ///
  pub fn decision_service_evaluator(&self) -> Ref<DecisionServiceEvaluator> {
    self.decision_service_evaluator.borrow()
  }
  ///
  pub fn decision_evaluator(&self) -> Ref<DecisionEvaluator> {
    self.decision_evaluator.borrow()
  }
  /// Evaluates a business knowledge model.
  pub fn evaluate_business_knowledge_model(&self, id: &str, input_data: &FeelContext, output_variable_name: &Name) -> Value {
    let mut evaluated_ctx = FeelContext::default();
    self.business_knowledge_model_evaluator().evaluate(id, input_data, self, &mut evaluated_ctx);
    if let Some(Value::FunctionDefinition(parameters, body, result_type)) = evaluated_ctx.get_entry(output_variable_name) {
      let mut parameters_ctx = FeelContext::default();
      for (name, _) in parameters {
        if let Some(value) = input_data.get_entry(name) {
          parameters_ctx.set_entry(name, value.to_owned());
        }
      }
      parameters_ctx.zip(&evaluated_ctx);
      let result = body.evaluate(&parameters_ctx.into());
      result_type.coerced(&result)
    } else {
      value_null!()
    }
  }
  /// Evaluates a decision.
  pub fn evaluate_decision(&self, id: &str, input_data: &FeelContext) -> Value {
    let mut evaluated_ctx = FeelContext::default();
    if let Some(output_variable_name) = self.decision_evaluator().evaluate(id, input_data, self, &mut evaluated_ctx) {
      if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
        output_value.clone()
      } else {
        value_null!()
      }
    } else {
      value_null!()
    }
  }
  /// Evaluates a decision service.
  pub fn evaluate_decision_service(&self, id: &str, input_data: &FeelContext) -> Value {
    let mut evaluated_ctx = FeelContext::default();
    if let Some(output_variable_name) = self.decision_service_evaluator().evaluate(id, input_data, self, &mut evaluated_ctx) {
      if let Some(output_value) = evaluated_ctx.get_entry(&output_variable_name) {
        output_value.clone()
      } else {
        value_null!()
      }
    } else {
      value_null!()
    }
  }
}
