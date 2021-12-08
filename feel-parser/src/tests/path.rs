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

use super::accept;
use crate::dmntk_feel::context::FeelContext;
use crate::dmntk_feel::values::Value;
use crate::dmntk_feel::Name;
use crate::lalr::TokenType::StartTextualExpression;
use dmntk_feel::{scope, value_null, value_number, FeelNumber, Scope};

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    r#"1.first"#,
    r#"
       Path
       ├─ Numeric
       │  └─ `1.`
       └─ Name
          └─ `first`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  scope.set_entry(&"Manager".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    r#"Manager.Name"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Name
          └─ `Name`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  scope.set_entry(&"Manager".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    r#"Manager.Address.Street"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager`
       └─ Name
          └─ `Address.Street`
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
    r#"(Manager.Address).Street"#,
    r#"
       Path
       ├─ Name
       │  └─ `Manager.Address`
       └─ Name
          └─ `Street`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  scope.set_entry(&"Manager".into(), value_null!());
  scope.set_entry(&"Address".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    r#"Manager.Address.Street"#,
    r#"
       Path
       ├─ Path
       │  ├─ Name
       │  │  └─ `Manager`
       │  └─ Name
       │     └─ `Address`
       └─ Name
          └─ `Street`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  let mut ctx_a: FeelContext = Default::default();
  ctx_a.set_entry(&"Street".into(), Value::String("Alt Hof Str.".to_string()));
  let mut ctx_b: FeelContext = Default::default();
  ctx_b.set_entry(&Name::from("Address"), Value::Context(ctx_a));
  scope.set_entry(&"Manager".into(), Value::Context(ctx_b));
  accept(
    &scope,
    StartTextualExpression,
    r#"Manager.Address.Street"#,
    r#"
       Path
       ├─ Path
       │  ├─ Name
       │  │  └─ `Manager`
       │  └─ Name
       │     └─ `Address`
       └─ Name
          └─ `Street`
    "#,
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  let mut ctx_1 = FeelContext::default();
  ctx_1.set_entry(&"principal".into(), value_number!(60000));
  scope.set_entry(&"loan".into(), Value::Context(ctx_1));
  accept(
    &scope,
    StartTextualExpression,
    r#"(loan.principal)"#,
    r#"
       Path
       ├─ Name
       │  └─ `loan`
       └─ Name
          └─ `principal`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  let mut ctx_1 = FeelContext::default();
  ctx_1.set_entry(&"principal".into(), value_number!(60000));
  ctx_1.set_entry(&"rate".into(), value_number!(375, 4));
  ctx_1.set_entry(&"termMonths".into(), value_number!(360));
  scope.set_entry(&"loan".into(), Value::Context(ctx_1));
  accept(
    &scope,
    StartTextualExpression,
    r#"(loan.principal) + (loan.rate)"#,
    r#"
       Add
       ├─ Path
       │  ├─ Name
       │  │  └─ `loan`
       │  └─ Name
       │     └─ `principal`
       └─ Path
          ├─ Name
          │  └─ `loan`
          └─ Name
             └─ `rate`
    "#,
    false,
  );
}
