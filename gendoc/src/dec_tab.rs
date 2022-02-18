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

use crate::dec_tab::OutputAttr::{AllowedValues, CompoundNames, EmptyCell, Ignored, OutputLabel};
use crate::{INDENT, INDENT_2, INDENT_3, NL, WS};
use dmntk_model::model::{DecisionTable, DecisionTableOrientation};

const DECISION_TABLE_HTML_TEMPLATE: &str = include_str!("templates/decision-table.html");
const DECISION_TABLE_CSS_TEMPLATE: &str = include_str!("templates/decision-table.css");
const DECISION_TABLE_STYLES_PLACEHOLDER: &str = "/*#STYLES#*/";
const DECISION_TABLE_HTML_PLACEHOLDER: &str = "<!--#DECISION_TABLE#-->";

/// Decision table attributes.
struct DecisionTableAttr {
  /// Number of columns in decision table.
  column_count: usize,
  /// Flag indicating if input expressions are present.
  input_expressions_present: bool,
  /// Flag indicating if allowed values are present.
  allowed_values_present: bool,
  /// Flag indicating if the output label is present.
  output_label_present: bool,
  /// Flag indicating if there is more than one output clause.
  compound_output: bool,
  /// Flag indicating if annotations are present.
  annotations_present: bool,
  /// Total number of input clauses.
  input_clause_count: usize,
  /// Total number of output clauses.
  output_clause_count: usize,
  /// Total number of annotation clauses.
  annotation_clause_count: usize,
  /// Total number of rules.
  rule_count: usize,
}

/// Hit policy attributes.
struct HitPolicyAttr {
  /// Name of the class used for formatting the cell containing the hit policy.
  class: &'static str,
  /// Number of rows the hit policy cell spans over.
  rowspan: usize,
}

/// Input expression attributes.
struct InputExpressionAttr {
  /// Name of the class used for formatting the cell containing the input expression.
  class: &'static str,
  /// Number of rows the input expression spans over.
  rowspan: usize,
}

/// Output label attributes.
struct OutputLabelAttr {
  /// Name of the class used for formatting the cell containing the output label.
  class: &'static str,
  /// Number of columns the output label spans over.
  colspan: usize,
}

/// Output component attributes.
struct OutputComponentAttr {
  /// Name of the class used for formatting the cell containing the output component.
  class: &'static str,
}

/// Rule number attributes.
struct RuleNumberAttr {
  /// Name of the class used for formatting the cell containing the rule name.
  class: &'static str,
}

/// Input entry attributes.
struct InputEntryAttr {
  /// Name of the class used for formatting the cell containing the input entry.
  class: &'static str,
}

/// Output entry attributes.
struct OutputEntryAttr {
  /// Name of the class used for formatting the cell containing the output entry.
  class: &'static str,
}

/// Annotation attributes.
struct AnnotationAttr {
  /// Name of the class used for formatting the cell containing the annotation names.
  class: &'static str,
  /// Number of rows the annotation name spans over.
  rowspan: usize,
}

/// Annotation entry attributes.
struct AnnotationEntryAttr {
  /// Name of the class used for formatting the cell containing the annotation entry.
  class: &'static str,
}

/// Attributes containing only CSS class name.
struct ClassAttr {
  /// Name of the class used for styling the cell.
  class: &'static str,
}

/// Attributes for styling allowed input values.
type InputValueAttr = ClassAttr;

/// Attributes for styling allowed output values.
type OutputValueAttr = ClassAttr;

/// Attributes for styling allowed annotation values (always empty cell).
type AnnotationValueAttr = ClassAttr;

#[derive(Default)]
struct Row {
  cells: Vec<Cell>,
}

impl Row {
  /// Adds a cell to the row.
  fn add(&mut self, class: String, colspan: usize, rowspan: usize, content: String) {
    self.cells.push(Cell {
      class,
      colspan,
      rowspan,
      content,
    })
  }
  /// Writes a row to HTML output when contains any cells
  fn write(&self, html: &mut String, indent: usize) {
    if !self.cells.is_empty() {
      html.push_str(&format!(r#"{:i$}<tr>{}"#, WS, NL, i = indent));
      for cell in &self.cells {
        cell.write(html, indent + INDENT);
      }
      html.push_str(&format!(r#"{:i$}</tr>{}"#, WS, NL, i = indent));
    }
  }
}

#[derive(Default)]
struct Cell {
  class: String,
  colspan: usize,
  rowspan: usize,
  content: String,
}

impl Cell {
  /// Writes the cell to HTML output.
  fn write(&self, html: &mut String, indent: usize) {
    html.push_str(&format!(
      r#"{:i$}<td{}{}{}>{}</td>{}"#,
      WS,
      if !self.class.is_empty() {
        format!(r#" class="{}""#, self.class)
      } else {
        "".to_string()
      },
      if self.colspan > 0 {
        format!(r#" colspan="{}""#, self.colspan)
      } else {
        "".to_string()
      },
      if self.rowspan > 0 {
        format!(r#" rowspan="{}""#, self.rowspan)
      } else {
        "".to_string()
      },
      self.content,
      NL,
      i = indent
    ));
  }
}

enum OutputAttr {
  EmptyCell,
  OutputLabel,
  CompoundNames,
  AllowedValues,
  Ignored,
}

/// Generates single decision table in HTML format.
pub fn decision_table_to_html(decision_table: &DecisionTable) -> String {
  let decision_table_attributes = get_decision_table_attr(decision_table);
  DECISION_TABLE_HTML_TEMPLATE
    .replace(DECISION_TABLE_STYLES_PLACEHOLDER, &indent_content(DECISION_TABLE_CSS_TEMPLATE))
    .replace(
      DECISION_TABLE_HTML_PLACEHOLDER,
      &get_decision_table_html(decision_table, &decision_table_attributes),
    )
}

/// Returns HTML code containing the definition of decision table.
fn get_decision_table_html(decision_table: &DecisionTable, decision_table_attr: &DecisionTableAttr) -> String {
  let mut html = String::new();
  html.push_str(&format!(r#"<table class="decision-table horizontal">{}"#, NL));
  html.push_str(&format!(r#"  <tbody>{}"#, NL));
  match decision_table.preferred_orientation {
    DecisionTableOrientation::RuleAsRow => write_horizontal_decision_table(INDENT, &mut html, decision_table, decision_table_attr),
    DecisionTableOrientation::RuleAsColumn => {}
    DecisionTableOrientation::CrossTable => {}
  }
  html.push_str(&format!(r#"  </tbody>{}"#, NL));
  html.push_str(&format!(r#"</table>{}"#, NL));
  html
}

/// Writes the HTML code for horizontal decision table.
fn write_horizontal_decision_table(indent: usize, html: &mut String, decision_table: &DecisionTable, decision_table_attr: &DecisionTableAttr) {
  // write a row with information item name
  if let Some(information_item_name) = &decision_table.information_item_name {
    html.push_str(&get_information_item_name_html(indent, information_item_name, decision_table_attr.column_count))
  }
  // prepare three starting rows
  let mut row1 = Row::default();
  let mut row2 = Row::default();
  let mut row3 = Row::default();

  // add hit policy, always to first row
  let hit_policy_attributes = get_hit_policy_attr(decision_table_attr);
  row1.add(
    hit_policy_attributes.class.to_string(),
    0,
    hit_policy_attributes.rowspan,
    decision_table.hit_policy.to_string(),
  );
  // add input expressions, always to first row
  for (index, input_clause) in decision_table.input_clauses.iter().enumerate() {
    let input_expression_attributes = get_input_expression_attr(index, decision_table_attr);
    let content = input_clause.input_expression.trim().to_string();
    let class = input_expression_attributes.class.to_string();
    let rowspan = input_expression_attributes.rowspan;
    row1.add(class, 0, rowspan, content);
  }
  // prepare three first rows depending on the decision table structure
  let (r1, r2, r3) = get_output_attr(decision_table_attr);
  match r1 {
    EmptyCell => {
      let output_label_attr = get_output_label_attr(decision_table_attr);
      row1.add(output_label_attr.class.to_string(), output_label_attr.colspan, 0, "".to_string());
    }
    OutputLabel => {
      let output_label_attr = get_output_label_attr(decision_table_attr);
      row1.add(
        output_label_attr.class.to_string(),
        output_label_attr.colspan,
        0,
        decision_table.output_label.as_ref().unwrap_or(&"".to_string()).trim().to_string(),
      );
    }
    CompoundNames => {
      for (index, output_clause) in decision_table.output_clauses.iter().enumerate() {
        let output_component_attr = get_output_component_attr(index, decision_table_attr);
        row1.add(
          output_component_attr.class.to_string(),
          0,
          0,
          output_clause.name.as_ref().unwrap_or(&"".to_string()).trim().to_string(),
        );
      }
    }
    _ => {}
  }
  match r2 {
    CompoundNames => {
      for (index, output_clause) in decision_table.output_clauses.iter().enumerate() {
        let output_component_attr = get_output_component_attr(index, decision_table_attr);
        row2.add(
          output_component_attr.class.to_string(),
          0,
          0,
          output_clause.name.as_ref().unwrap_or(&"".to_string()).trim().to_string(),
        );
      }
    }
    AllowedValues => {
      for (index, input_clause) in decision_table.input_clauses.iter().enumerate() {
        let input_value_attr = get_input_value_attr(index, decision_table_attr);
        row2.add(
          input_value_attr.class.to_string(),
          0,
          0,
          input_clause.input_values.as_ref().unwrap_or(&"".to_string()).trim().to_string(),
        );
      }
      for (index, output_clause) in decision_table.output_clauses.iter().enumerate() {
        let output_value_attr = get_output_value_attr(index, decision_table_attr);
        row2.add(
          output_value_attr.class.to_string(),
          0,
          0,
          output_clause.output_values.as_ref().unwrap_or(&"".to_string()).trim().to_string(),
        );
      }
      for (index, _) in decision_table.annotations.iter().enumerate() {
        let annotation_value_attr = get_annotation_value_attr(index, decision_table_attr);
        row2.add(annotation_value_attr.class.to_string(), 0, 0, "".to_string());
      }
    }
    _ => {}
  }
  if let AllowedValues = r3 {
    for (index, input_clause) in decision_table.input_clauses.iter().enumerate() {
      let input_value_attr = get_input_value_attr(index, decision_table_attr);
      row3.add(
        input_value_attr.class.to_string(),
        0,
        0,
        input_clause.input_values.as_ref().unwrap_or(&"".to_string()).trim().to_string(),
      );
    }
    for (index, output_clause) in decision_table.output_clauses.iter().enumerate() {
      let output_value_attr = get_output_value_attr(index, decision_table_attr);
      row3.add(
        output_value_attr.class.to_string(),
        0,
        0,
        output_clause.output_values.as_ref().unwrap_or(&"".to_string()).trim().to_string(),
      );
    }
    for (index, _) in decision_table.annotations.iter().enumerate() {
      let annotation_value_attr = get_annotation_value_attr(index, decision_table_attr);
      row3.add(annotation_value_attr.class.to_string(), 0, 0, "".to_string());
    }
  }
  // write annotation names
  for (index, annotation) in decision_table.annotations.iter().enumerate() {
    let annotation_attr = get_annotation_attr(index, decision_table_attr);
    row1.add(
      annotation_attr.class.to_string(),
      0,
      annotation_attr.rowspan,
      annotation.name.trim().to_string(),
    );
  }
  // write three starting rows (empty row is simple omitted)
  row1.write(html, indent);
  row2.write(html, indent);
  row3.write(html, indent);
  // write multiple rows with rules
  for (rule_index, rule) in decision_table.rules.iter().enumerate() {
    let mut row = Row::default();
    let rule_number_attributes = get_rule_number_attr(rule_index, decision_table_attr);
    row.add(rule_number_attributes.class.to_string(), 0, 0, format!("{}", rule_index + 1));
    for (index, input_entry) in rule.input_entries.iter().enumerate() {
      let input_entry_attr = get_input_entry_attr(index, rule_index, decision_table_attr);
      row.add(input_entry_attr.class.to_string(), 0, 0, input_entry.text.trim().to_string());
    }
    for (index, output_entry) in rule.output_entries.iter().enumerate() {
      let output_entry_attr = get_output_entry_attr(index, rule_index, decision_table_attr);
      row.add(output_entry_attr.class.to_string(), 0, 0, output_entry.text.trim().to_string());
    }
    for (index, annotation_entry) in rule.annotation_entries.iter().enumerate() {
      let annotation_entry_attr = get_annotation_entry_attr(index, decision_table_attr);
      row.add(annotation_entry_attr.class.to_string(), 0, 0, annotation_entry.text.trim().to_string());
    }
    row.write(html, indent);
  }
}

/// Returns HTML code containing a row with information item name.
fn get_information_item_name_html(indent: usize, content: &str, colspan: usize) -> String {
  let mut html = String::new();
  html.push_str(&format!(r#"{:i$}<tr>{}"#, WS, NL, i = indent));
  html.push_str(&format!(
    r#"{:i$}<td colspan="{}" class="information-item">{}"#,
    WS,
    colspan,
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
    content.trim(),
    NL,
    i = indent + INDENT_3
  ));
  html.push_str(&format!(r#"{:i$}</div>{}"#, WS, NL, i = indent + INDENT_2));
  html.push_str(&format!(r#"{:i$}</td>{}"#, WS, NL, i = indent + INDENT));
  html.push_str(&format!(r#"{:i$}</tr>{}"#, WS, NL, i = indent));
  html
}

fn get_decision_table_attr(decision_table: &DecisionTable) -> DecisionTableAttr {
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
  let output_label_present = !decision_table.output_label.as_ref().unwrap_or(&"".to_string()).trim().is_empty();
  let annotations_present = !decision_table.annotations.is_empty();
  let compound_output = decision_table.output_clauses.len() > 1;
  let input_clause_count = decision_table.input_clauses.len();
  let output_clause_count = decision_table.output_clauses.len();
  let annotation_clause_count = decision_table.annotations.len();
  let rule_count = decision_table.rules.len();
  DecisionTableAttr {
    column_count,
    input_expressions_present,
    allowed_values_present,
    output_label_present,
    compound_output,
    annotations_present,
    input_clause_count,
    output_clause_count,
    annotation_clause_count,
    rule_count,
  }
}

fn get_hit_policy_attr(decision_table_attr: &DecisionTableAttr) -> HitPolicyAttr {
  let (class, rowspan) = match (
    decision_table_attr.input_expressions_present,
    decision_table_attr.compound_output,
    decision_table_attr.allowed_values_present,
  ) {
    (true, true, true) => ("hit-policy-a", 3),
    (false, true, true) => ("hit-policy-b", 3),
    (true, false, true) => ("hit-policy-a", 2),
    (false, false, true) => ("hit-policy-b", 2),
    (true, true, false) => ("hit-policy-a", 2),
    (false, true, false) => ("hit-policy-b", 2),
    (true, false, false) => ("hit-policy-a", 1),
    (false, false, false) => ("hit-policy-b", 1),
  };
  HitPolicyAttr { class, rowspan }
}

fn get_input_expression_attr(index: usize, decision_table_attr: &DecisionTableAttr) -> InputExpressionAttr {
  let (class, rowspan) = match (
    decision_table_attr.output_label_present,
    decision_table_attr.compound_output,
    decision_table_attr.allowed_values_present,
    index == decision_table_attr.input_clause_count - 1,
  ) {
    (true, true, true, false) => ("input-expression-a", 2),
    (false, true, true, false) => ("input-expression-a", 1),
    (_, false, true, false) => ("input-expression-a", 1),
    (true, true, false, false) => ("input-expression-c", 2),
    (false, true, false, false) => ("input-expression-c", 1),
    (_, false, false, false) => ("input-expression-c", 1),
    (true, true, true, true) => ("input-expression-b", 2),
    (false, true, true, true) => ("input-expression-b", 1),
    (_, false, true, true) => ("input-expression-b", 1),
    (true, true, false, true) => ("input-expression-d", 2),
    (false, true, false, true) => ("input-expression-d", 1),
    (_, false, false, true) => ("input-expression-d", 1),
  };
  InputExpressionAttr { class, rowspan }
}

fn get_output_label_attr(decision_table_attr: &DecisionTableAttr) -> OutputLabelAttr {
  let class = match (
    decision_table_attr.allowed_values_present,
    decision_table_attr.compound_output,
    decision_table_attr.annotations_present,
  ) {
    (true, true, true) => "output-label-c",
    (false, true, true) => "output-label-c",
    (true, false, true) => "output-label-c",
    (false, false, true) => "output-label-d",
    (true, true, false) => "output-label-b",
    (false, true, false) => "output-label-b",
    (true, false, false) => "output-label-b",
    (false, false, false) => "output-label-a",
  };
  let colspan = if decision_table_attr.output_clause_count > 1 {
    decision_table_attr.output_clause_count
  } else {
    0
  };
  OutputLabelAttr { class, colspan }
}

fn get_output_component_attr(index: usize, decision_table_attr: &DecisionTableAttr) -> OutputComponentAttr {
  let class = match (
    decision_table_attr.output_label_present,
    decision_table_attr.allowed_values_present,
    decision_table_attr.annotations_present,
    index == decision_table_attr.output_clause_count - 1,
  ) {
    (true, true, true, true) => "output-component-d",
    (false, true, true, true) => "output-component-g",
    (true, false, true, true) => "output-component-f",
    (false, false, true, true) => "output-component-e",
    (true, true, false, true) => "output-component-d",
    (false, true, false, true) => "output-component-d",
    (true, false, false, true) => "output-component-b",
    (false, false, false, true) => "output-component-b",
    (true, true, true, false) => "output-component-c",
    (false, true, true, false) => "output-component-c",
    (true, false, true, false) => "output-component-a",
    (false, false, true, false) => "output-component-a",
    (true, true, false, false) => "output-component-c",
    (false, true, false, false) => "output-component-c",
    (true, false, false, false) => "output-component-a",
    (false, false, false, false) => "output-component-a",
  };
  OutputComponentAttr { class }
}

fn get_rule_number_attr(rule_index: usize, decision_table_attr: &DecisionTableAttr) -> RuleNumberAttr {
  let class = match (decision_table_attr.input_expressions_present, rule_index == decision_table_attr.rule_count - 1) {
    (true, true) => "rule-number-b",
    (false, true) => "rule-number-d",
    (true, false) => "rule-number-a",
    (false, false) => "rule-number-c",
  };
  RuleNumberAttr { class }
}

fn get_input_entry_attr(index: usize, rule_index: usize, decision_table_attr: &DecisionTableAttr) -> InputEntryAttr {
  let class = match (
    index == decision_table_attr.input_clause_count - 1,
    rule_index == decision_table_attr.rule_count - 1,
  ) {
    (false, false) => "input-entry-a",
    (false, true) => "input-entry-a",
    (true, false) => "input-entry-b",
    (true, true) => "input-entry-c",
  };
  InputEntryAttr { class }
}

fn get_output_entry_attr(index: usize, rule_index: usize, decision_table_attr: &DecisionTableAttr) -> OutputEntryAttr {
  let class = match (
    index == decision_table_attr.output_clause_count - 1,
    rule_index == decision_table_attr.rule_count - 1,
    decision_table_attr.annotations_present,
  ) {
    (true, true, true) => "output-entry-c",
    (false, true, true) => "output-entry-a",
    (true, false, true) => "output-entry-b",
    (false, false, true) => "output-entry-a",
    (true, true, false) => "output-entry-b",
    (false, true, false) => "output-entry-a",
    (true, false, false) => "output-entry-b",
    (false, false, false) => "output-entry-a",
  };
  OutputEntryAttr { class }
}

fn get_annotation_attr(index: usize, decision_table_attr: &DecisionTableAttr) -> AnnotationAttr {
  let (class, rowspan) = match (
    index == decision_table_attr.annotation_clause_count - 1,
    decision_table_attr.allowed_values_present,
    decision_table_attr.compound_output && decision_table_attr.output_label_present,
  ) {
    (true, true, true) => ("annotation-d", 2),
    (false, true, true) => ("annotation-c", 2),
    (true, false, true) => ("annotation-b", 2),
    (false, false, true) => ("annotation-a", 2),
    (true, true, false) => ("annotation-d", 1),
    (false, true, false) => ("annotation-c", 1),
    (true, false, false) => ("annotation-b", 1),
    (false, false, false) => ("annotation-a", 1),
  };
  AnnotationAttr { class, rowspan }
}

fn get_annotation_entry_attr(index: usize, decision_table_attr: &DecisionTableAttr) -> AnnotationEntryAttr {
  let class = match index == decision_table_attr.annotation_clause_count - 1 {
    true => "annotation-entry-b",
    false => "annotation-entry-a",
  };
  AnnotationEntryAttr { class }
}

fn get_input_value_attr(index: usize, decision_table_attr: &DecisionTableAttr) -> InputValueAttr {
  let class = match index == decision_table_attr.input_clause_count - 1 {
    true => "input-value-b",
    false => "input-value-a",
  };
  InputValueAttr { class }
}

fn get_output_value_attr(index: usize, decision_table_attr: &DecisionTableAttr) -> OutputValueAttr {
  let class = match (index == decision_table_attr.output_clause_count - 1, decision_table_attr.annotations_present) {
    (false, _) => "output-value-a",
    (true, false) => "output-value-b",
    (true, true) => "output-value-c",
  };
  OutputValueAttr { class }
}

fn get_annotation_value_attr(index: usize, decision_table_attr: &DecisionTableAttr) -> AnnotationValueAttr {
  let class = match index == decision_table_attr.annotation_clause_count - 1 {
    true => "annotation-value-b",
    false => "annotation-value-a",
  };
  AnnotationValueAttr { class }
}

fn get_output_attr(decision_table_attr: &DecisionTableAttr) -> (OutputAttr, OutputAttr, OutputAttr) {
  match (
    decision_table_attr.output_label_present,
    decision_table_attr.compound_output,
    decision_table_attr.allowed_values_present,
  ) {
    (false, false, false) => (EmptyCell, Ignored, Ignored),
    (true, false, false) => (OutputLabel, Ignored, Ignored),
    (false, true, false) => (CompoundNames, Ignored, Ignored),
    (true, true, false) => (OutputLabel, CompoundNames, Ignored),
    (false, false, true) => (EmptyCell, AllowedValues, Ignored),
    (true, false, true) => (OutputLabel, AllowedValues, Ignored),
    (false, true, true) => (CompoundNames, AllowedValues, Ignored),
    (true, true, true) => (OutputLabel, CompoundNames, AllowedValues),
  }
}

/// Returns the content indented four spaces.
fn indent_content(content: &str) -> String {
  let mut indented_content = String::new();
  let mut first = true;
  for line in content.lines() {
    if !first {
      indented_content.push('\n');
    }
    first = false;
    indented_content.push_str(&format!("    {}", line));
  }
  indented_content
}
