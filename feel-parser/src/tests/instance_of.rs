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
use crate::dmntk_feel::context::FeelContext;
use crate::dmntk_feel::values::Value;
use crate::dmntk_feel::Name;
use crate::lalr::TokenType::StartTextualExpression;
use dmntk_feel::{scope, value_null, value_number, FeelNumber, Scope};

#[test]
fn _0001() {
  let scope = scope!();
  scope.set_entry(&"Person".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Person instance of number",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FeelType
          └─ number
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  scope.set_entry(&"Person".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Person instance of function<string,string,number>->string",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FunctionType
          ├─ ParameterTypes
          │  ├─ FeelType
          │  │  └─ string
          │  ├─ FeelType
          │  │  └─ string
          │  └─ FeelType
          │     └─ number
          └─ FeelType
             └─ string
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  scope.set_entry(&"Person".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Person instance of function<string>->string",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FunctionType
          ├─ ParameterTypes
          │  └─ FeelType
          │     └─ string
          └─ FeelType
             └─ string
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  scope.set_entry(&"Person".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Person instance of function<>->number",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ FunctionType
          ├─ ParameterTypes
          │  └─ (empty)
          └─ FeelType
             └─ number
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  scope.set_entry(&"Numbers".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Numbers instance of list<number>",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ ListType
          └─ FeelType
             └─ number
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  scope.set_entry(&"Person".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Person instance of context<name:string,age:number>",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ ContextType
          ├─ ContextTypeEntry
          │  ├─ Name
          │  │  └─ `name`
          │  └─ FeelType
          │     └─ string
          └─ ContextTypeEntry
             ├─ Name
             │  └─ `age`
             └─ FeelType
                └─ number
    "#,
    false,
  );
}

#[test]
fn _0006_1() {
  let scope = scope!();
  scope.set_entry(&"Person".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "  \nPerson \r instance of \t context  <   name:  string ,  age  : number >  ",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Person`
       └─ ContextType
          ├─ ContextTypeEntry
          │  ├─ Name
          │  │  └─ `name`
          │  └─ FeelType
          │     └─ string
          └─ ContextTypeEntry
             ├─ Name
             │  └─ `age`
             └─ FeelType
                └─ number
    "#,
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  scope.set_entry(&"Numbers".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Numbers instance of range<number>",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ RangeType
          └─ FeelType
             └─ number
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  scope.set_entry(&"Power".into(), value_number!(255, 1));
  let mut ctx: FeelContext = Default::default();
  let name_power = Name::from("power");
  ctx.set_entry(&name_power, value_number!(2805, 1));
  scope.set_entry(&"engine".into(), Value::Context(ctx));
  accept(
    &scope,
    StartTextualExpression,
    "Power instance of engine.power",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Power`
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
fn _0009() {
  let scope = scope!();
  scope.set_entry(&"Power".into(), value_number!(255, 1));
  let mut ctx: FeelContext = Default::default();
  let name_power = Name::from("power");
  ctx.set_entry(&name_power, Value::Boolean(false));
  scope.set_entry(&"engine".into(), Value::Context(ctx));
  accept(
    &scope,
    StartTextualExpression,
    "Power instance of engine.power",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Power`
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
fn _0010() {
  let scope = scope!();
  scope.set_entry(&"Items".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Items instance of list<list<number>>",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Items`
       └─ ListType
          └─ ListType
             └─ FeelType
                └─ number
    "#,
    false,
  );
}

#[test]
fn _0011() {
  let scope = scope!();
  scope.set_entry(&"Numbers".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "Numbers instance of list <number>",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ ListType
          └─ FeelType
             └─ number
    "#,
    false,
  );
}

#[test]
fn _0012() {
  let scope = scope!();
  scope.set_entry(&"Numbers".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "  Numbers    instance  of   list    <   number   >   ",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ ListType
          └─ FeelType
             └─ number
    "#,
    false,
  );
}

#[test]
fn _0013() {
  let scope = scope!();
  scope.set_entry(&"Numbers".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    " Numbers \t     instance  \t of  \t range \r <   \n number \t  >  ",
    r#"
       InstanceOf
       ├─ Name
       │  └─ `Numbers`
       └─ RangeType
          └─ FeelType
             └─ number
    "#,
    false,
  );
}
