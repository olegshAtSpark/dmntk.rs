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

//! `FEEL` or `DMN` function body definition.

use crate::values::Value;
use crate::{Evaluator, Scope};
use std::sync::Arc;

/// Type alias of the closure that evaluates `FEEL` or `DMN` function body into [Value].
pub type FunctionBodyEvaluator = Arc<Evaluator>;

/// Function body may be defined in `FEEL` or `DMN` in many ways.
/// This enum is the representation of all of these cases.
#[derive(Derivative, Clone)]
#[derivative(Debug, PartialEq)]
pub enum FunctionBody {
  /// Function body created from context defined in `DMN` model.
  Context(#[derivative(Debug = "ignore", PartialEq = "ignore")] FunctionBodyEvaluator),
  /// Function body created from `FEEL` textual expression defined in `DMN` model.
  LiteralExpression(#[derivative(Debug = "ignore", PartialEq = "ignore")] FunctionBodyEvaluator),
  /// Function body created from decision table defined in `DMN` model.
  DecisionTable(#[derivative(Debug = "ignore", PartialEq = "ignore")] FunctionBodyEvaluator),
  /// Function body created from decision service defined in `DMN` model.
  DecisionService(#[derivative(Debug = "ignore", PartialEq = "ignore")] FunctionBodyEvaluator),
  /// Function body created from externally defined function (`Java`, `PMML`).
  External(#[derivative(Debug = "ignore", PartialEq = "ignore")] FunctionBodyEvaluator),
}

impl FunctionBody {
  /// Evaluates function body, takes a [Scope] as input and returns evaluated [Value].
  pub fn evaluate(&self, scope: &Scope) -> Value {
    match self {
      FunctionBody::Context(evaluator) => evaluator(scope),
      FunctionBody::LiteralExpression(evaluator) => evaluator(scope),
      FunctionBody::DecisionTable(evaluator) => evaluator(scope),
      FunctionBody::DecisionService(evaluator) => evaluator(scope),
      FunctionBody::External(evaluator) => evaluator(scope),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::values::Value;
  use crate::{value_number, FeelNumber, FunctionBody, Scope};
  use std::sync::Arc;

  #[test]
  fn _0001() {
    let scope = &Scope::default();
    let fun_body = FunctionBody::Context(Arc::new(Box::new(|_: &Scope| value_number!(1))));
    assert_eq!(value_number!(1), fun_body.evaluate(scope));
  }

  #[test]
  fn _0002() {
    let scope = &Scope::default();
    let fun_body = FunctionBody::LiteralExpression(Arc::new(Box::new(|_: &Scope| value_number!(2))));
    assert_eq!(value_number!(2), fun_body.evaluate(scope));
  }

  #[test]
  fn _0003() {
    let scope = &Scope::default();
    let fun_body = FunctionBody::DecisionTable(Arc::new(Box::new(|_: &Scope| value_number!(3))));
    assert_eq!(value_number!(3), fun_body.evaluate(scope));
  }

  #[test]
  fn _0004() {
    let scope = &Scope::default();
    let fun_body = FunctionBody::DecisionService(Arc::new(Box::new(|_: &Scope| value_number!(4))));
    assert_eq!(value_number!(4), fun_body.evaluate(scope));
  }

  #[test]
  fn _0005() {
    let scope = &Scope::default();
    let fun_body = FunctionBody::External(Arc::new(Box::new(|_: &Scope| value_number!(5))));
    assert_eq!(value_number!(5), fun_body.evaluate(scope));
  }
}
