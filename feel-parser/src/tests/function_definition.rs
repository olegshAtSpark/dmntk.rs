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
use crate::lalr::TokenType::*;
use dmntk_feel::{scope, Scope};

#[test]
fn _0001() {
  let scope = scope!();
  accept(
    &scope,
    StartContext,
    r#"{msg: function () "hello!" }"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `msg`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ (empty)
             └─ FunctionBody
                └─ String
                   └─ `hello!`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  accept(
    &scope,
    StartContext,
    r#"{pow: function (x: number) x * x }"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `pow`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `x`
             │     └─ FeelType
             │        └─ number
             └─ FunctionBody
                └─ Mul
                   ├─ Name
                   │  └─ `x`
                   └─ Name
                      └─ `x`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  accept(
    &scope,
    StartContext,
    r#"{add: function (x: number, y: number) x + y }"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `add`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  ├─ FormalParameter
             │  │  ├─ ParameterName
             │  │  │  └─ `x`
             │  │  └─ FeelType
             │  │     └─ number
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `y`
             │     └─ FeelType
             │        └─ number
             └─ FunctionBody
                └─ Add
                   ├─ Name
                   │  └─ `x`
                   └─ Name
                      └─ `y`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  accept(
    &scope,
    StartContext,
    r#"{add3:function(x:number,y:number,z:number)x+y+z}"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `add3`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  ├─ FormalParameter
             │  │  ├─ ParameterName
             │  │  │  └─ `x`
             │  │  └─ FeelType
             │  │     └─ number
             │  ├─ FormalParameter
             │  │  ├─ ParameterName
             │  │  │  └─ `y`
             │  │  └─ FeelType
             │  │     └─ number
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `z`
             │     └─ FeelType
             │        └─ number
             └─ FunctionBody
                └─ Add
                   ├─ Add
                   │  ├─ Name
                   │  │  └─ `x`
                   │  └─ Name
                   │     └─ `y`
                   └─ Name
                      └─ `z`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  accept(
    &scope,
    StartContext,
    r#"{inc: function (state) state + 1 }"#,
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `inc`
          └─ FunctionDefinition
             ├─ FormalParameters
             │  └─ FormalParameter
             │     ├─ ParameterName
             │     │  └─ `state`
             │     └─ FeelType
             │        └─ Any
             └─ FunctionBody
                └─ Add
                   ├─ Name
                   │  └─ `state`
                   └─ Numeric
                      └─ `1.`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartExpression,
    r#"function(a,b) a+b"#,
    r#"
       FunctionDefinition
       ├─ FormalParameters
       │  ├─ FormalParameter
       │  │  ├─ ParameterName
       │  │  │  └─ `a`
       │  │  └─ FeelType
       │  │     └─ Any
       │  └─ FormalParameter
       │     ├─ ParameterName
       │     │  └─ `b`
       │     └─ FeelType
       │        └─ Any
       └─ FunctionBody
          └─ Add
             ├─ Name
             │  └─ `a`
             └─ Name
                └─ `b`
    "#,
    false,
  );
}
