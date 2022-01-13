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
use crate::lalr::TokenType::StartTextualExpression;
use dmntk_feel::values::Value;
use dmntk_feel::{scope, value_null, Scope};

/// Compares the given `name` with `expected` name after parsing.
fn accept_name(scope: &Scope, name: &str, expected: &str) {
  accept(
    scope,
    StartTextualExpression,
    name,
    &format!("\n       Name\n       └─ `{}`\n    ", expected),
    false,
  );
}

#[test]
fn _0001() {
  let scope = scope!();
  scope.set_entry(&"hello world".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "hello world",
    r#"
       Name
       └─ `hello world`
    "#,
    false,
  );
}

#[test]
fn _0002() {
  let scope = scope!();
  scope.set_entry(&"  \n  \n  \t  thing \n \t \t ".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "thing",
    r#"
       Name
       └─ `thing`
    "#,
    false,
  );
}

#[test]
fn _0003() {
  let scope = scope!();
  scope.set_entry(&"income/loss".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "income/loss",
    r#"
       Name
       └─ `income/loss`
    "#,
    false,
  );
}

#[test]
fn _0004() {
  let scope = scope!();
  scope.set_entry(&"fr**n*s".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "fr**n*s",
    r#"
       Name
       └─ `fr**n*s`
    "#,
    false,
  );
}

#[test]
fn _0005() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "book",
    r#"
       Name
       └─ `book`
    "#,
    false,
  );
}

#[test]
fn _0006() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "profit/loss",
    r#"
       Name
       └─ `profit/loss`
    "#,
    false,
  );
}

#[test]
fn _0007() {
  let scope = scope!();
  scope.set_entry(&"before.after".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "before.after",
    r#"
       Name
       └─ `before.after`
    "#,
    false,
  );
}

#[test]
fn _0008() {
  let scope = scope!();
  assert!(crate::parse_name(&scope, "5Cards", false).is_err());
}

#[test]
fn _0009() {
  let scope = scope!();
  assert!(crate::parse_name(&scope, "123.45", false).is_err());
}

#[test]
fn _0010() {
  let scope = scope!();
  scope.set_entry(&"thing".into(), value_null!());
  scope.set_entry(&"one two three four".into(), value_null!());
  scope.set_entry(&"one and two".into(), value_null!());
  scope.set_entry(&"one or two".into(), value_null!());
  scope.set_entry(&"before.after".into(), value_null!());
  scope.set_entry(&"before.or.after".into(), value_null!());
  scope.set_entry(&"before.and.after".into(), value_null!());
  scope.set_entry(&"before.between.after".into(), value_null!());
  scope.set_entry(&"before.next to between.worm".into(), value_null!());
  scope.set_entry(&"income/loss".into(), value_null!());
  scope.set_entry(&"per/month/income/loss".into(), value_null!());
  scope.set_entry(&"a-b".into(), value_null!());
  scope.set_entry(&"to-be-or-not-to-be".into(), value_null!());
  scope.set_entry(&"that's".into(), value_null!());
  scope.set_entry(&"ok that's".into(), value_null!());
  scope.set_entry(&"bed+breakfast".into(), value_null!());
  scope.set_entry(&"night+and+day".into(), value_null!());
  scope.set_entry(&"fr**n*s".into(), value_null!());
  scope.set_entry(&"bo++e--m*".into(), value_null!());
  scope.set_entry(&"wh*t*v*r".into(), value_null!());
  accept_name(&scope, "thing", "thing");
  accept_name(&scope, "one two three four", "one two three four");
  accept_name(&scope, "one and two", "one and two");
  accept_name(&scope, "one or two", "one or two");
  accept_name(&scope, "before.after", "before.after");
  accept_name(&scope, "before.or.after", "before.or.after");
  accept_name(&scope, "before.and.after", "before.and.after");
  accept_name(&scope, "before.between.after", "before.between.after");
  accept_name(&scope, "before.next to between.worm", "before.next to between.worm");
  accept_name(&scope, "income/loss", "income/loss");
  accept_name(&scope, "per/month/income/loss", "per/month/income/loss");
  accept_name(&scope, "a-b", "a-b");
  accept_name(&scope, "to-be-or-not-to-be", "to-be-or-not-to-be");
  accept_name(&scope, "that's", "that's");
  accept_name(&scope, "ok that's", "ok that's");
  accept_name(&scope, "bed+breakfast", "bed+breakfast");
  accept_name(&scope, "night+and+day", "night+and+day");
  accept_name(&scope, "fr**n*s", "fr**n*s");
  accept_name(&scope, "bo++e--m*", "bo++e--m*");
  accept_name(&scope, "wh*t*v*r", "wh*t*v*r");
}

#[test]
fn _0011() {
  let scope = scope!();
  scope.set_entry(&"?".into(), value_null!());
  accept(
    &scope,
    StartTextualExpression,
    "?.Rating > 0",
    r#"
       Gt
       ├─ Path
       │  ├─ Name
       │  │  └─ `?`
       │  └─ Name
       │     └─ `Rating`
       └─ Numeric
          └─ `0.`
    "#,
    false,
  );
}

#[test]
fn _0012() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "?",
    r#"
       Name
       └─ `?`
    "#,
    false,
  );
}
