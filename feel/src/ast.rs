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

//! Implementation of a node in Abstract Syntax Tree for `FEEL` grammar.

use crate::ast_tree::ast_tree;
use crate::types::FeelType;
use crate::{Name, Scope};
use std::borrow::Borrow;

/// Type for optional AST node.
pub type OptAstNode = Option<AstNode>;

/// Node of the Abstract Syntax Tree for `FEEL` grammar.
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
  /// Node representing an arithmetic operator `+` (addition).
  Add(Box<AstNode>, Box<AstNode>),

  /// Node representing a logical operator `and` (conjunction).
  And(Box<AstNode>, Box<AstNode>),

  /// Node representing `@` (at literal).
  At(String),

  /// Node representing a comparison operator `between`.
  Between(Box<AstNode>, Box<AstNode>, Box<AstNode>),

  /// Node representing a value of type `boolean`.
  Boolean(bool),

  /// Node representing a comma separated list of AST nodes, used internally in parser.
  CommaList(Vec<AstNode>),

  /// Node representing a context.
  /// Context entries are stored in the order of appearance in definition.
  Context(Vec<AstNode>),

  /// Node representing single context entry; key-value pair.
  ContextEntry(Box<AstNode>, Box<AstNode>),

  /// Node representing the key of the context entry; the key in context entry
  /// may be a name or string literal. String literals are converted to one segment names
  /// containing exactly the value of the string.
  ContextEntryKey(Name),

  /// Node representing the type of a context. Context type is defined by names
  /// and types of all entries. This node contains a collection of types
  /// for all context entries in the order of appearance in context type definition.
  ContextType(Vec<AstNode>),

  /// Node representing single context type entry.
  ContextTypeEntry(
    /// Node representing entry name of the context key.
    Box<AstNode>,
    /// Node representing `FEEL` type of the context entry.
    Box<AstNode>,
  ),

  /// Node representing the key of the entry in context type definition.
  /// In context type definition, only `FEEL` name is allowed as an entry key.
  ContextTypeEntryKey(Name),

  /// Node representing arithmetic operator `/` (division).
  Div(Box<AstNode>, Box<AstNode>),

  ///
  Eq(Box<AstNode>, Box<AstNode>),

  /// Node representing an expression evaluated as a body of `for` expression.
  EvaluatedExpression(Box<AstNode>),

  /// Quantified expression `every`.
  Every(
    /// Node representing quantified contexts.
    Box<AstNode>,
    /// Node representing an expression after `satisfies` clause.
    Box<AstNode>,
  ),

  ///
  Exp(Box<AstNode>, Box<AstNode>),

  ///
  ExpressionList(Vec<AstNode>),

  /// Node representing `FEEL` type.
  FeelType(FeelType),

  /// Node representing filter expression.
  Filter(Box<AstNode>, Box<AstNode>),

  /// Node representing `for` expression.
  For(
    /// Node representing [iteration contexts](AstNode::IterationContexts).
    Box<AstNode>,
    /// Node representing an expression to be evaluated.
    Box<AstNode>,
  ),

  /// Node representing function's formal parameter.
  FormalParameter(
    /// Node representing the name of the parameter.
    Box<AstNode>,
    /// Node representing the `FEEL` type of the parameter.
    Box<AstNode>,
  ),

  /// Node representing a list of formal parameters.
  FormalParameters(Vec<AstNode>),

  /// Node representing function's body. This node holds mandatory function body
  /// and a flag indicating if the function is external.
  FunctionBody(Box<AstNode>, bool),

  /// Node representing function definition.
  /// This node holds function's formal parameter list and  function's body.
  FunctionDefinition(Box<AstNode>, Box<AstNode>),

  /// Node representing function invocation.
  FunctionInvocation(Box<AstNode>, Box<AstNode>),

  /// Node representing function type.
  FunctionType(
    /// Node representing function's parameter types as [AstNode::ParameterTypes].
    Box<AstNode>,
    /// Node representing function's result type.
    Box<AstNode>,
  ),

  ///
  Ge(Box<AstNode>, Box<AstNode>),

  ///
  Gt(Box<AstNode>, Box<AstNode>),

  /// Node representing `if` expression.
  If(
    /// Node representing the condition.
    Box<AstNode>,
    /// Node representing the expression to be evaluated when condition is `true`.
    Box<AstNode>,
    /// Node representing the expression to be evaluated when condition is `false`.
    Box<AstNode>,
  ),

  ///
  In(Box<AstNode>, Box<AstNode>),

  /// Node representing type checking function.
  InstanceOf(
    /// Node representing the tested value.
    Box<AstNode>,
    /// Node representing `FELL` type to be checked.
    Box<AstNode>,
  ),

  /// Node representing the interval end used in ranges.
  IntervalEnd(Box<AstNode>, bool),

  /// Node representing the interval start used in ranges.
  IntervalStart(Box<AstNode>, bool),

  /// Node representing the comparison operator `irrelevant`.
  Irrelevant,

  /// List of iteration contexts.
  IterationContexts(Vec<AstNode>),

  /// Node representing iteration context containing the variable name and a single list of elements to iterate over.
  IterationContextSingle(
    /// Node representing variable name used in this iteration context.
    Box<AstNode>,
    /// Node representing a single list of elements to iterate over.
    Box<AstNode>,
  ),

  /// Node representing iteration context containing the variable name and a range of numbers to iterate over.
  IterationContextRange(
    /// Node representing variable name used in this iteration context.
    Box<AstNode>,
    /// Node representing the **start** of the range of numbers to iterate over.
    Box<AstNode>,
    /// Node representing the **end** of the range of numbers to iterate over.
    Box<AstNode>,
  ),

  ///
  Le(Box<AstNode>, Box<AstNode>),

  ///
  Lt(Box<AstNode>, Box<AstNode>),

  /// Node representing a list.
  List(Vec<AstNode>),

  /// Node representing a list type.
  ListType(Box<AstNode>),

  /// Node representing arithmetic operator `*` (multiplication).
  Mul(Box<AstNode>, Box<AstNode>),

  /// Node representing a `FEEL` name.
  Name(Name),

  /// Node representing single named parameter.
  NamedParameter(
    /// Node representing parameter name.
    Box<AstNode>,
    /// Node representing parameter type.
    Box<AstNode>,
  ),

  /// Node representing a collection of named parameters.
  NamedParameters(Vec<AstNode>),

  /// Node representing a negated list (used in negated unary tests).
  NegatedList(Vec<AstNode>),

  /// Node representing an unary arithmetic negation `-`.
  Neg(Box<AstNode>),

  ///
  Nq(Box<AstNode>, Box<AstNode>),

  /// Node representing a value of type `Null`.
  Null,

  /// Node representing a value of type `number`.
  Numeric(String, String),

  ///
  Or(Box<AstNode>, Box<AstNode>),

  ///
  Out(Box<AstNode>, Box<AstNode>),

  /// Node representing a name of the function's formal parameter.
  ParameterName(Name),

  /// Node representing a collection of function parameter types.
  ParameterTypes(Vec<AstNode>),

  /// Node representing a path expression.
  Path(Box<AstNode>, Box<AstNode>),

  /// Node representing a collection of positional parameters.
  PositionalParameters(Vec<AstNode>),

  /// Node representing a collection of names that constitute a qualified name.
  QualifiedName(Vec<AstNode>),

  /// Node representing a segment of a qualified name.
  QualifiedNameSegment(Name),

  /// List of quantified contexts.
  QuantifiedContexts(Vec<AstNode>),

  /// Quantified context containing variable name and evaluation expression.
  QuantifiedContext(
    /// Node representing variable name used in this quantified context.
    Box<AstNode>,
    /// Node representing evaluation expression.
    Box<AstNode>,
  ),

  ///
  Range(Box<AstNode>, Box<AstNode>),

  /// Node representing range type.
  RangeType(Box<AstNode>),

  /// Node representing `satisfies` clause in quantified expression.
  Satisfies(Box<AstNode>),

  /// Node representing quantified expression `some`.
  Some(
    /// Node representing quantified contexts.
    Box<AstNode>,
    /// Node representing an expression after `satisfies` clause.
    Box<AstNode>,
  ),

  /// Node representing a value of type `string`.
  String(String),

  ///
  Sub(Box<AstNode>, Box<AstNode>),

  ///
  UnaryGe(Box<AstNode>),

  ///
  UnaryGt(Box<AstNode>),

  ///
  UnaryLe(Box<AstNode>),

  ///
  UnaryLt(Box<AstNode>),
}

impl ToString for AstNode {
  /// Converts [AstNode] to textual representation, including child nodes.
  fn to_string(&self) -> String {
    format!("{}\n    ", ast_tree(self))
  }
}

impl AstNode {
  /// Evaluates the type of the expression represented by this node.
  pub fn type_of(&self, scope: &Scope) -> FeelType {
    match self {
      AstNode::Add(lhs, rhs) => lhs.type_of(scope).zip(&rhs.type_of(scope)),
      AstNode::And { .. } => FeelType::Any,
      AstNode::At { .. } => FeelType::Any,
      AstNode::Between { .. } => FeelType::Any,
      AstNode::Boolean(_) => FeelType::Any,
      AstNode::CommaList { .. } => FeelType::Any,
      AstNode::Context { .. } => FeelType::Any,
      AstNode::ContextEntry { .. } => FeelType::Any,
      AstNode::ContextEntryKey(_) => FeelType::Any,
      AstNode::ContextType(items) => {
        if items.is_empty() {
          FeelType::Any
        } else {
          let mut type_entries = vec![];
          for item in items {
            if let AstNode::ContextTypeEntry(entry_name, entry_type) = item {
              if let AstNode::Name(name) = entry_name.borrow() {
                if let AstNode::FeelType(feel_type) = entry_type.borrow() {
                  type_entries.push((name, feel_type));
                }
              }
            }
          }
          FeelType::context(&type_entries)
        }
      }
      AstNode::ContextTypeEntry(_, node) => node.type_of(scope),
      AstNode::ContextTypeEntryKey(name) => name.into(),
      AstNode::Div(lhs, rhs) => lhs.type_of(scope).zip(&rhs.type_of(scope)),
      AstNode::Eq { .. } => FeelType::Boolean,
      AstNode::EvaluatedExpression { .. } => FeelType::Any,
      AstNode::Exp { .. } => FeelType::Any,
      AstNode::ExpressionList { .. } => FeelType::Any,
      AstNode::FeelType(feel_type) => feel_type.clone(),
      AstNode::Filter { .. } => FeelType::Any,
      AstNode::For { .. } => FeelType::Any,
      AstNode::FormalParameter(_, rhs) => rhs.type_of(scope),
      AstNode::FormalParameters(_) => FeelType::Any,
      AstNode::FunctionBody { .. } => FeelType::Any,
      AstNode::FunctionDefinition { .. } => FeelType::Any,
      AstNode::FunctionInvocation { .. } => FeelType::Any,
      AstNode::FunctionType(a, b) => {
        if let AstNode::ParameterTypes(items) = a.borrow() {
          let x = items.iter().map(|c| c.type_of(scope)).collect::<Vec<FeelType>>();
          FeelType::function(&x, &b.type_of(scope))
        } else {
          FeelType::Any
        }
      }
      AstNode::Gt { .. } => FeelType::Any,
      AstNode::Ge { .. } => FeelType::Any,
      AstNode::If { .. } => FeelType::Any,
      AstNode::In { .. } => FeelType::Any,
      AstNode::InstanceOf { .. } => FeelType::Any,
      AstNode::Irrelevant => FeelType::Any,
      AstNode::IterationContexts { .. } => FeelType::Any,
      AstNode::IterationContextSingle { .. } => FeelType::Any,
      AstNode::IterationContextRange { .. } => FeelType::Any,
      AstNode::Lt { .. } => FeelType::Any,
      AstNode::Le { .. } => FeelType::Any,
      AstNode::Neg { .. } => FeelType::Any,
      AstNode::List(nodes) => {
        if nodes.is_empty() {
          FeelType::Any
        } else {
          let mut base_type = nodes[0].type_of(scope);
          for node in nodes.iter().skip(1) {
            base_type = base_type.zip(&node.type_of(scope));
          }
          base_type
        }
      }
      AstNode::ListType(lhs) => FeelType::List(Box::new(lhs.type_of(scope))),
      AstNode::Mul { .. } => FeelType::Any,
      AstNode::Name(name) => name.into(),
      AstNode::NamedParameter { .. } => FeelType::Any,
      AstNode::NamedParameters { .. } => FeelType::Any,
      AstNode::NegatedList { .. } => FeelType::Any,
      AstNode::Nq { .. } => FeelType::Boolean,
      AstNode::Null => FeelType::Null,
      AstNode::Numeric(_, _) => FeelType::Number,
      AstNode::Or { .. } => FeelType::Any,
      AstNode::Out { .. } => FeelType::Any,
      AstNode::ParameterName(_) => FeelType::Any,
      AstNode::ParameterTypes(_) => FeelType::Any,
      AstNode::Path { .. } => FeelType::Any,
      AstNode::PositionalParameters { .. } => FeelType::Any,
      AstNode::QualifiedName(nodes) => {
        let names = nodes
          .iter()
          .filter_map(|node| if let AstNode::Name(name) = node { Some(name.clone()) } else { None })
          .collect::<Vec<Name>>();
        if let Some(value) = scope.search_deep(&names) {
          value.type_of()
        } else {
          FeelType::Any
        }
      }
      AstNode::QualifiedNameSegment(_) => FeelType::Any,
      AstNode::QuantifiedContexts { .. } => FeelType::Any,
      AstNode::QuantifiedContext { .. } => FeelType::Any,
      AstNode::Every { .. } => FeelType::Any,
      AstNode::Some { .. } => FeelType::Any,
      AstNode::Range { .. } => FeelType::Any,
      AstNode::RangeType(lhs) => FeelType::Range(Box::new(lhs.type_of(scope))),
      AstNode::Satisfies(mid) => mid.type_of(scope),
      AstNode::String(_) => FeelType::String,
      AstNode::Sub { .. } => FeelType::Any,
      AstNode::UnaryGe { .. } => FeelType::Any,
      AstNode::UnaryGt { .. } => FeelType::Any,
      AstNode::UnaryLt { .. } => FeelType::Any,
      AstNode::UnaryLe { .. } => FeelType::Any,
      AstNode::IntervalEnd(_, _) => FeelType::Any,
      AstNode::IntervalStart(_, _) => FeelType::Any,
    }
  }

  /// Writes a trace of the AST starting from this node.
  pub fn trace(&self) {
    println!("      AST:{}", self.to_string());
  }
}
