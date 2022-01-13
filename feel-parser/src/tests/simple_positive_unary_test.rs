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

use super::accept;
use crate::dmntk_feel::values::Value;
use crate::dmntk_feel::Name;
use crate::lalr::TokenType::StartTextualExpression;
use dmntk_feel::context::FeelContext;
use dmntk_feel::{scope, value_null, value_number, FeelNumber, Scope};

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#"<2"#,
    r#"
       UnaryLt
       └─ Numeric
          └─ `2.`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" <= 12.465"#,
    r#"
       UnaryLe
       └─ Numeric
          └─ `12.465`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" > 50"#,
    r#"
       UnaryGt
       └─ Numeric
          └─ `50.`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#" >= time("10:23")"#,
    r#"
       UnaryGe
       └─ FunctionInvocation
          ├─ Name
          │  └─ `time`
          └─ PositionalParameters
             └─ String
                └─ `10:23`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  let mut ctx: FeelContext = Default::default();
  let name_power = Name::from("power");
  ctx.set_entry(&name_power, value_number!(2805, 1));
  scope.set_entry(&"engine".into(), Value::Context(ctx));
  accept(
    &scope,
    StartTextualExpression,
    r#" >= engine.power"#,
    r#"
       UnaryGe
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  scope.set_entry(&"engine".into(), value_null!());
  scope.set_entry(&"power".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    r#" >= engine.power"#,
    r#"
       UnaryGe
       └─ QualifiedName
          ├─ Name
          │  └─ `engine`
          └─ Name
             └─ `power`
    "#,
    false,
  );
}

#[test]
#[should_panic]
fn _0007() {
  let scope = scope!();
  accept(&scope, StartTextualExpression, r#" < null"#, r#""#, false);
}
