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

//! ???

use crate::AstNode;
use ascii_tree::{write_tree, Tree};

pub fn ast_tree(root: &AstNode) -> String {
  let mut ascii_tree = String::new();
  let tree = ast_node_to_tree(root);
  let _ = write_tree(&mut ascii_tree, &tree);
  ascii_tree.lines().map(|line| format!("\n      {}", line)).collect()
}

fn ast_node_to_tree(node: &AstNode) -> Tree {
  match node {
    AstNode::Add(lhs, rhs) => node_2("Add", lhs, rhs),
    AstNode::And(lhs, rhs) => node_2("And", lhs, rhs),
    AstNode::At(mid) => node_and_leaf("At", &format!("`{}`", mid)),
    AstNode::Between(lhs, mid, rhs) => node_3("Between", lhs, mid, rhs),
    AstNode::Boolean(mid) => node_and_leaf("Boolean", &format!("`{}`", mid)),
    AstNode::CommaList(mid) => node_items("CommaList", mid),
    AstNode::Context(items) => node_items("Context", items),
    AstNode::ContextEntry(lhs, rhs) => node_2("ContextEntry", lhs, rhs),
    AstNode::ContextEntryKey(mid) => node_and_leaf("ContextEntryKey", &format!("`{}`", mid)),
    AstNode::ContextType(items) => node_items("ContextType", items),
    AstNode::ContextTypeEntry(lhs, rhs) => node_2("ContextTypeEntry", lhs, rhs),
    AstNode::ContextTypeEntryKey(mid) => node_and_leaf("Name", &format!("`{}`", mid)),
    AstNode::Div(lhs, rhs) => node_2("Div", lhs, rhs),
    AstNode::Eq(lhs, rhs) => node_2("Eq", lhs, rhs),
    AstNode::EvaluatedExpression(mid) => node_1("EvaluatedExpression", mid),
    AstNode::Every(lhs, rhs) => node_2("Every", lhs, rhs),
    AstNode::Exp(lhs, rhs) => node_2("Exp", lhs, rhs),
    AstNode::ExpressionList(items) => node_items("ExpressionList", items),
    AstNode::FeelType(lhs) => node_and_leaf("FeelType", &lhs.to_string()),
    AstNode::Filter(lhs, rhs) => node_2("Filter", lhs, rhs),
    AstNode::For(lhs, rhs) => node_2("For", lhs, rhs),
    AstNode::FormalParameter(lhs, rhs) => node_2("FormalParameter", lhs, rhs),
    AstNode::FormalParameters(items) => node_items("FormalParameters", items),
    AstNode::FunctionBody(lhs, external) => node_and_label("FunctionBody", lhs, " (external)", "", *external),
    AstNode::FunctionDefinition(lhs, rhs) => node_2("FunctionDefinition", lhs, rhs),
    AstNode::FunctionInvocation(lhs, rhs) => node_2("FunctionInvocation", lhs, rhs),
    AstNode::FunctionType(lhs, rhs) => node_2("FunctionType", lhs, rhs),
    AstNode::Ge(lhs, rhs) => node_2("Ge", lhs, rhs),
    AstNode::Gt(lhs, rhs) => node_2("Gt", lhs, rhs),
    AstNode::If(lhs, mid, rhs) => node_3("If", lhs, mid, rhs),
    AstNode::In(lhs, rhs) => node_2("In", lhs, rhs),
    AstNode::InstanceOf(lhs, rhs) => node_2("InstanceOf", lhs, rhs),
    AstNode::IntervalEnd(lhs, closed) => node_and_label("IntervalEnd", lhs, " (closed)", " (opened)", *closed),
    AstNode::IntervalStart(lhs, closed) => node_and_label("IntervalStart", lhs, " (closed)", " (opened)", *closed),
    AstNode::Irrelevant => leaf("Irrelevant"),
    AstNode::IterationContexts(items) => node_items("IterationContexts", items),
    AstNode::IterationContextSingle(lhs, rhs) => node_2("IterationContextSingle", lhs, rhs),
    AstNode::IterationContextRange(lhs, mid, rhs) => node_3("IterationContextRange", lhs, mid, rhs),
    AstNode::Le(lhs, rhs) => node_2("Le", lhs, rhs),
    AstNode::List(mid) => node_items("List", mid),
    AstNode::ListType(lhs) => node_1("ListType", lhs),
    AstNode::Lt(lhs, rhs) => node_2("Lt", lhs, rhs),
    AstNode::Mul(lhs, rhs) => node_2("Mul", lhs, rhs),
    AstNode::Name(mid) => node_and_leaf("Name", &format!("`{}`", mid)),
    AstNode::NamedParameter(lhs, rhs) => node_2("NamedParameter", lhs, rhs),
    AstNode::NamedParameters(items) => node_items("NamedParameters", items),
    AstNode::Neg(mid) => node_1("Neg", mid),
    AstNode::NegatedList(mid) => node_items("NegatedList", mid),
    AstNode::Nq(lhs, rhs) => node_2("Nq", lhs, rhs),
    AstNode::Null => leaf("Null"),
    AstNode::Numeric(lhs, rhs) => node_and_leaf("Numeric", &format!("`{}.{}`", lhs, rhs)),
    AstNode::Or(lhs, rhs) => node_2("Or", lhs, rhs),
    AstNode::Out(lhs, rhs) => node_2("Out", lhs, rhs),
    AstNode::ParameterName(lhs) => node_and_leaf("ParameterName", &format!("`{}`", lhs)),
    AstNode::ParameterTypes(items) => node_items("ParameterTypes", items),
    AstNode::Path(lhs, rhs) => node_2("Path", lhs, rhs),
    AstNode::PositionalParameters(items) => node_items("PositionalParameters", items),
    AstNode::QualifiedName(items) => node_items("QualifiedName", items),
    AstNode::QualifiedNameSegment(lhs) => node_and_leaf("Name", &format!("`{}`", lhs)),
    AstNode::QuantifiedContext(lhs, rhs) => node_2("QuantifiedContext", lhs, rhs),
    AstNode::QuantifiedContexts(items) => node_items("QuantifiedContexts", items),
    AstNode::Range(lhs, rhs) => node_2("Range", lhs, rhs),
    AstNode::RangeType(lhs) => node_1("RangeType", lhs),
    AstNode::Satisfies(mid) => node_1("Satisfies", mid),
    AstNode::Some(lhs, rhs) => node_2("Some", lhs, rhs),
    AstNode::String(mid) => node_and_leaf("String", &format!("`{}`", mid)),
    AstNode::Sub(lhs, rhs) => node_2("Sub", lhs, rhs),
    AstNode::UnaryGe(mid) => node_1("UnaryGe", mid),
    AstNode::UnaryGt(mid) => node_1("UnaryGt", mid),
    AstNode::UnaryLe(mid) => node_1("UnaryLe", mid),
    AstNode::UnaryLt(mid) => node_1("UnaryLt", mid),
  }
}

fn node_1(name: &str, mid: &AstNode) -> Tree {
  Tree::Node(name.to_string(), vec![ast_node_to_tree(mid)])
}

fn node_2(name: &str, lhs: &AstNode, rhs: &AstNode) -> Tree {
  Tree::Node(name.to_string(), vec![ast_node_to_tree(lhs), ast_node_to_tree(rhs)])
}

fn node_3(name: &str, lhs: &AstNode, mid: &AstNode, rhs: &AstNode) -> Tree {
  Tree::Node(name.to_string(), vec![ast_node_to_tree(lhs), ast_node_to_tree(mid), ast_node_to_tree(rhs)])
}

fn node_items(name: &str, items: &[AstNode]) -> Tree {
  Tree::Node(
    name.to_string(),
    if !items.is_empty() {
      items.iter().map(ast_node_to_tree).collect()
    } else {
      vec![Tree::Leaf(vec!["(empty)".to_string()])]
    },
  )
}

fn node_and_leaf(name: &str, leaf: &str) -> Tree {
  Tree::Node(name.to_string(), vec![Tree::Leaf(vec![leaf.to_string()])])
}

fn node_and_label(name: &str, lhs: &AstNode, label_true: &str, label_false: &str, label_flag: bool) -> Tree {
  Tree::Node(
    format!("{}{}", name, if label_flag { label_true } else { label_false }),
    vec![ast_node_to_tree(lhs)],
  )
}

fn leaf(leaf: &str) -> Tree {
  Tree::Leaf(vec![leaf.to_string()])
}

#[cfg(test)]
mod tests {
  use crate::ast_tree::ast_tree;
  use crate::AstNode;

  #[test]
  fn test_add() {
    let node = AstNode::Add(
      Box::new(AstNode::Numeric("1".to_string(), "".to_string())),
      Box::new(AstNode::Numeric("2".to_string(), "".to_string())),
    );
    println!("{}", ast_tree(&node));
  }
}
