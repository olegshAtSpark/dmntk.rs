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

use crate::{INDENT, INDENT_2, INDENT_3, NL, WS};
use dmntk_model::model::{DecisionTable, DecisionTableOrientation};

const DECISION_TABLE_TEMPLATE: &str = include_str!("templates/decision_table_template.html");
const HTML_DECISION_TABLE: &str = "#DECISION_TABLE#";

/// Decision table attributes.
struct DecisionTableAttributes {
  /// Number of columns in decision table.
  column_count: usize,
  /// Flag indicating if input expressions are present.
  input_expressions_present: bool,
  /// Flag indicating if allowed values are present.
  allowed_values_present: bool,
  /// Flag indicating if there is more than one output clause.
  compound_output: bool,
  /// Total number of rules.
  rule_count: usize,
}

/// Hit policy attributes.
struct HitPolicyAttributes {
  /// Name of the class used for formatting the cell containing the hit policy.
  class_name: &'static str,
  /// Number of rows the hit policy cell spans over.
  row_span: usize,
}

/// Rule number attributes.
struct RuleNumberAttributes {
  /// Name of the class used for formatting the cell containing the rule name.
  class_name: &'static str,
}

/// Generates single decision table in HTML format.
pub fn decision_table_to_html(decision_table: &DecisionTable) -> String {
  let attributes = get_decision_table_attributes(decision_table);
  DECISION_TABLE_TEMPLATE.replace(HTML_DECISION_TABLE, &html_decision_table(decision_table, &attributes))
}

fn html_decision_table(decision_table: &DecisionTable, attributes: &DecisionTableAttributes) -> String {
  let mut html = String::new();
  html.push_str(&format!(r#"<table class="decision-table">{}"#, NL));
  html.push_str(&format!(r#"  <tbody>{}"#, NL));
  match decision_table.preferred_orientation {
    DecisionTableOrientation::RuleAsRow => html_horizontal_decision_table(INDENT, &mut html, decision_table, attributes),
    DecisionTableOrientation::RuleAsColumn => {}
    DecisionTableOrientation::CrossTable => {}
  }
  html.push_str(&format!(r#"  </tbody>{}"#, NL));
  html.push_str(&format!(r#"</table>{}"#, NL));
  html
}

fn html_horizontal_decision_table(indent: usize, html: &mut String, decision_table: &DecisionTable, decision_table_attributes: &DecisionTableAttributes) {
  // write one row: information item name
  if let Some(information_item_name) = &decision_table.information_item_name {
    html.push_str(&html_information_item_name(
      indent,
      information_item_name,
      decision_table_attributes.column_count,
    ))
  }
  // write one row: hit policy, input expressions, output label, annotations
  html.push_str(&format!(r#"{:i$}<tr>{}"#, WS, NL, i = indent));
  let hit_policy_attributes = get_hit_policy_attributes(decision_table_attributes);
  html.push_str(&format!(
    r#"{:i$}<td class="{}" rowspan="{}">{}</td>{}"#,
    WS,
    hit_policy_attributes.class_name,
    hit_policy_attributes.row_span,
    decision_table.hit_policy,
    NL,
    i = indent + INDENT
  ));
  html.push_str(&format!(
    r#"{:i$}<td class="h-output-label-A">{}</td>{}"#,
    WS,
    decision_table.output_label.as_ref().unwrap().trim(),
    NL,
    i = indent + INDENT
  ));
  html.push_str(&format!(r#"{:i$}</tr>{}"#, WS, NL, i = indent));
  // write one row: compound outputs when present

  // write one row: allowed values when present

  // write multiple rows: rules
  for (rule_index, rule) in decision_table.rules.iter().enumerate() {
    let rule_number = rule_index + 1;
    let rule_number_attributes = get_rule_number_attributes(rule_number, decision_table_attributes);
    html.push_str(&format!(r#"{:i$}<tr>{}"#, WS, NL, i = indent));
    html.push_str(&format!(
      r#"{:i$}<td class="{}">{}</td>{}"#,
      WS,
      rule_number_attributes.class_name,
      rule_number,
      NL,
      i = indent + INDENT
    ));
    html.push_str(&format!(
      r#"{:i$}<td class="{}">{}</td>{}"#,
      WS,
      "output-entry-last",
      rule.output_entries[0].text.trim(),
      NL,
      i = indent + INDENT
    ));
    html.push_str(&format!(r#"{:i$}</tr>{}"#, WS, NL, i = indent));
  }
}

fn html_information_item_name(indent: usize, information_item_name: &str, col_span: usize) -> String {
  let mut html = String::new();
  html.push_str(&format!(r#"{:i$}<tr>{}"#, WS, NL, i = indent));
  html.push_str(&format!(
    r#"{:i$}<td colspan="{}" class="information-item">{}"#,
    WS,
    col_span,
    NL,
    i = indent + INDENT
  ));
  html.push_str(&format!(
    r#"{:i$}<div class="information-item-name-container">{}"#,
    WS,
    NL,
    i = indent + INDENT_2
  ));
  html.push_str(&format!(
    r#"{:i$}<div class="information-item-name">{}</div>{}"#,
    WS,
    information_item_name.trim(),
    NL,
    i = indent + INDENT_3
  ));
  html.push_str(&format!(r#"{:i$}</div>{}"#, WS, NL, i = indent + INDENT_2));
  html.push_str(&format!(r#"{:i$}</td>{}"#, WS, NL, i = indent + INDENT));
  html.push_str(&format!(r#"{:i$}</tr>{}"#, WS, NL, i = indent));
  html
}

fn get_decision_table_attributes(decision_table: &DecisionTable) -> DecisionTableAttributes {
  let input_expressions_present = !decision_table.input_clauses.is_empty();
  let mut allowed_values_present = false;
  for input_clause in &decision_table.input_clauses {
    if input_clause.input_values.is_some() {
      allowed_values_present = true;
      break;
    }
  }
  if !allowed_values_present {
    for output_clause in &decision_table.output_clauses {
      if output_clause.output_values.is_some() {
        allowed_values_present = true;
        break;
      }
    }
  }
  let column_count = if decision_table.preferred_orientation == DecisionTableOrientation::RuleAsRow {
    decision_table.input_clauses.len() + decision_table.output_clauses.len() + decision_table.annotations.len() + 1
  } else {
    0
  };
  let compound_output = decision_table.output_clauses.len() > 1;
  let rule_count = decision_table.rules.len();
  DecisionTableAttributes {
    column_count,
    input_expressions_present,
    allowed_values_present,
    compound_output,
    rule_count,
  }
}

fn get_hit_policy_attributes(decision_table_attributes: &DecisionTableAttributes) -> HitPolicyAttributes {
  let (class_name, row_span) = match (
    decision_table_attributes.input_expressions_present,
    decision_table_attributes.compound_output,
    decision_table_attributes.allowed_values_present,
  ) {
    (true, true, true) => ("h-hit-policy-A", 3),
    (false, true, true) => ("h-hit-policy-B", 3),
    (true, false, true) => ("h-hit-policy-A", 2),
    (false, false, true) => ("h-hit-policy-B", 2),
    (true, true, false) => ("h-hit-policy-A", 2),
    (false, true, false) => ("h-hit-policy-B", 2),
    (true, false, false) => ("h-hit-policy-A", 1),
    (false, false, false) => ("h-hit-policy-B", 1),
  };
  HitPolicyAttributes { class_name, row_span }
}

fn get_rule_number_attributes(rule_number: usize, decision_table_attributes: &DecisionTableAttributes) -> RuleNumberAttributes {
  let class_name = match (
    decision_table_attributes.input_expressions_present,
    rule_number == decision_table_attributes.rule_count,
  ) {
    (false, false) => "horz-rule-number-no-input",
    (false, true) => "horz-rule-number-no-input-last",
    _ => "",
  };
  RuleNumberAttributes { class_name }
}
