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

use crate::lalr::TokenType;
use crate::lalr::TokenType::StartTextualExpression;
use crate::parser::Parser;
use difference::Changeset;
use dmntk_feel::{scope, Name, Scope};

mod addition;
mod arithmetic_negation;
mod between;
mod bifs;
mod boxed_expression;
mod comment;
mod comparison;
mod conjunction;
mod context;
mod date;
mod disjunction;
mod division;
mod every_expression;
mod exponentiation;
mod expression;
mod filter;
mod for_expression;
mod function_definition;
mod function_invocation;
mod if_expression;
mod instance_of;
mod interval;
mod list;
mod literal;
mod miscellaneous;
mod multiplication;
mod name;
mod numeric_literal;
mod path;
mod range;
mod simple_positive_unary_test;
mod some_expression;
mod subtraction;
mod temporal_date;
mod temporal_date_time;
mod textual_expressions;
mod unary_tests;

/// Parses the input text and compared the result with expected value.
fn accept(scope: &Scope, start_token_type: TokenType, input: &str, expected: &str, trace: bool) {
  let node = Parser::new(scope, start_token_type, input, trace).parse().unwrap();
  let actual = node.to_string();
  if actual != expected {
    println!("EXPECTED:\n------------------------------------------------------------{}\n", expected);
    println!("ACTUAL:\n------------------------------------------------------------{}\n", actual);
    println!(
      "DIFF:\n------------------------------------------------------------{}\n",
      Changeset::new(expected, &actual, "")
    );
  }
  assert_eq!(expected, actual);
}

#[test]
fn test_parse_textual_expression() {
  let scope = scope!();
  assert_eq!(
    r#"
       Add
       ├─ Numeric
       │  └─ `1.`
       └─ Numeric
          └─ `2.`
    "#,
    crate::parse_textual_expression(&scope, "1+2", false).unwrap().to_string()
  );
}

#[test]
fn test_parse_textual_expressions() {
  let scope = scope!();
  assert_eq!(
    r#"
       ExpressionList
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `1.`
       │  └─ Numeric
       │     └─ `2.`
       ├─ Add
       │  ├─ Numeric
       │  │  └─ `2.`
       │  └─ Numeric
       │     └─ `3.`
       └─ Mul
          ├─ Numeric
          │  └─ `3.`
          └─ Numeric
             └─ `4.`
    "#,
    crate::parse_textual_expressions(&scope, "1+2,2+3,3*4", false).unwrap().to_string()
  );
}

#[test]
fn test_parse_unary_tests() {
  let scope = scope!();
  assert_eq!(
    r#"
       ExpressionList
       ├─ Numeric
       │  └─ `1.`
       ├─ Numeric
       │  └─ `2.`
       ├─ Numeric
       │  └─ `3.`
       └─ Numeric
          └─ `4.`
    "#,
    crate::parse_unary_tests(&scope, "1,2,3,4", false).unwrap().to_string()
  );
}

#[test]
fn test_parse_boxed_expression() {
  let scope = scope!();
  assert_eq!(
    r#"
       List
       ├─ Numeric
       │  └─ `1.`
       ├─ Numeric
       │  └─ `2.`
       ├─ Numeric
       │  └─ `3.`
       └─ Numeric
          └─ `4.`
    "#,
    crate::parse_boxed_expression(&scope, "[1,2,3,4]", false).unwrap().to_string()
  );
}

#[test]
fn test_parse_context() {
  let scope = scope!();
  assert_eq!(
    r#"
       Context
       └─ ContextEntry
          ├─ ContextEntryKey
          │  └─ `age`
          └─ Numeric
             └─ `50.`
    "#,
    crate::parse_context(&scope, "{age: 50}", false).unwrap().to_string()
  );
}

#[test]
fn test_parse_name() {
  let name_a: Name = Name::new(&["Full", "House"]);
  let scope = scope!();
  assert_eq!(name_a, crate::parse_name(&scope, "Full House", false).unwrap());
}

#[test]
fn test_parse_longest_name() {
  let name_a: Name = Name::new(&["Full", "House", "With", "A", "Cat"]);
  assert_eq!(name_a, crate::parse_longest_name(" Full House  With  \t A \n\n Cat    ").unwrap());
}

/// Covers the case when function `accept` reports an error,
/// which means that the test result differs from expected value.
#[test]
#[should_panic]
fn test_not_accept() {
  let scope = scope!();
  accept(
    &scope,
    StartTextualExpression,
    "1+2",
    r#"
        Add
          Numeric 1.
          Numeric 3.
      "#,
    false,
  );
}
