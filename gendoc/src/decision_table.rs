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

//! decision_tree utilities.

use dmntk_model::model::{BuiltinAggregator, DecisionTable, DecisionTableOrientation, HitPolicy};

pub fn generate_decision_table(decision_table: &DecisionTable) -> String {
  let mut decision_table_html = String::new();

  match &decision_table.preferred_orientation {
    DecisionTableOrientation::RuleAsRow => {
      let tr_information_item_name = get_information_item_name(decision_table);
      let table_header = get_table_header(decision_table);
      let mut rules = get_rules(decision_table);
      let mut table_content = vec![tr_information_item_name,
                                   table_header.0,
                                   table_header.1];
      table_content.append(&mut rules);
      let html_role_as_row = table("decision-table", table_content.as_mut_slice());

      decision_table_html.push_str(&html_role_as_row);
    }
    DecisionTableOrientation::RuleAsColumn => {
      decision_table_html.push_str(&div("", &["CONTENT RuleAsColumn".to_string()]));
    }
    DecisionTableOrientation::CrossTable => {
      decision_table_html.push_str(&div("", &["CONTENT CrossTable".to_string()]));
    }
  }
  decision_table_html
}

fn div(class: &str, content: &[String]) -> String {
  write_element("div", class, "", content)
}

fn table(class: &str, content: &[String]) -> String {
  write_element("table", class, "", content)
}

fn tr(class: &str, content: &[String]) -> String {
  write_element("tr", class, "", content)
}

fn td(class: &str, other_attributes: &str, content: &[String]) -> String {
  write_element("td", class, other_attributes, content)
}

fn write_element(element_name: &str, class: &str, other_attributes: &str, content: &[String]) -> String {
  let mut str = format!("<{} class=\"{}\" {}>", element_name, class, other_attributes);
  for c in content {
    str.push_str(c);
  }
  str.push_str(&format!("</{}>", element_name));
  str
}

fn get_hit_policy(hit_policy: &HitPolicy) -> String {
  match hit_policy {
    HitPolicy::Unique => {
      String::from("U")
    }
    HitPolicy::Any => {
      String::from("A")
    }
    HitPolicy::Priority => {
      String::from("P")
    }
    HitPolicy::First => {
      String::from("F")
    }
    HitPolicy::Collect(builtin_aggregator) => {
      match builtin_aggregator {
        BuiltinAggregator::List => {
          String::from("C")
        }
        BuiltinAggregator::Count => {
          String::from("C#")
        }
        BuiltinAggregator::Sum => {
          String::from("C+")
        }
        BuiltinAggregator::Min => {
          String::from("C<")
        }
        BuiltinAggregator::Max => {
          String::from("C>")
        }
      }
    }
    HitPolicy::OutputOrder => {
      String::from("O")
    }
    HitPolicy::RuleOrder => {
      String::from("R")
    }
  }
}

fn get_information_item_name(decision_table: &DecisionTable) -> String {
  if let Some(item_information_name) = &decision_table.information_item_name {
    tr("", &[
      td("information-item", format!("colspan=\"{}\"", decision_table.input_clauses.len() + decision_table.output_clauses.len()).as_str(), &[
        div("information-item-name-container", &[
          div("information-item-name", &[
            // "information-item-name".to_string()
            item_information_name.clone()
          ])
        ])
      ])
    ])
  } else {
    String::new()
  }
}

fn get_table_header(decision_table: &DecisionTable) -> (String,String) {
  let mut tds1 = vec![td("hit-policy", "rowspan=\"2\"", &[get_hit_policy(&decision_table.hit_policy)])];
  let mut tds2 = Vec::new();
  for input_clause in &decision_table.input_clauses {
    if decision_table.input_clauses.len() > tds1.len() {
      tds1.push(td("input-expression", "", &[input_clause.input_expression.clone()]));
      if let Some(input_values) = &input_clause.input_values {
        tds2.push(td("input-value", "", &[input_values.clone()]))
      } else {
        tds2.push(td("input-value", "", &[String::new()]))
      }
    } else {
      tds1.push(td("input-expression-last", "", &[input_clause.input_expression.clone()]));
      if let Some(input_values) = &input_clause.input_values {
        tds2.push(td("input-value-last", "", &[input_values.clone()]))
      } else {
        tds2.push(td("input-value-last", "", &[String::new()]))
      }
    }
  }

  if let Some(label) = &decision_table.output_label {
    tds1.push(td("output-label", "", &[label.clone()]));
  }
  else {
    tds1.push(td("output-label", "", &[String::new()]));
  }

  for output_clause in &decision_table.output_clauses {
    // if decision_table.output_clauses.len() - 1 < tds1.len() {
      if let Some(output_values) = &output_clause.output_values {
        tds2.push(td("output-value-last", "", &[output_values.clone()]))
      } else {
        tds2.push(td("output-value-last", "", &[String::new()]))
      }
    // }
  }

  let tr1 = tr("", tds1.as_mut_slice());
  let tr2 = tr("", tds2.as_mut_slice());

  (tr1, tr2)
}

fn get_rules(decision_table: &DecisionTable) -> Vec<String> {
  let mut trs_rules = vec![];
  let mut i = 1;
  for rule in &decision_table.rules {
    if decision_table.rules.len() -1 > trs_rules.len() {
      let mut tds_rules = vec![td("rule-number", "", &[i.to_string()])];

      for input_entry in &rule.input_entries {
        if rule.input_entries.len() > tds_rules.len() {
          tds_rules.push(td("input-entry", "", &[input_entry.text.clone()]));
        } else {
          tds_rules.push(td("input-entry-last", "", &[input_entry.text.clone()]));
        }
      }
      for output_entry in &rule.output_entries {
        if rule.input_entries.len() + rule.output_entries.len() > tds_rules.len() {
          tds_rules.push(td("output-entry", "", &[output_entry.text.clone()]));
        } else {
          tds_rules.push(td("output-entry-last", "", &[output_entry.text.clone()]));
        }
      }
      let tr_rule = tr("", tds_rules.as_mut_slice());
      trs_rules.push(tr_rule);
      i += 1;
    }
   else {
     let mut tds_rules = vec![td("rule-number-last", "", &[i.to_string()])];

     for input_entry in &rule.input_entries {
       if rule.input_entries.len() > tds_rules.len() {
         tds_rules.push(td("input-entry-bottom", "", &[input_entry.text.clone()]));
       } else {
         tds_rules.push(td("input-entry-bottom-last", "", &[input_entry.text.clone()]));
       }
     }
     for output_entry in &rule.output_entries {
       if rule.input_entries.len() + rule.output_entries.len() > tds_rules.len() {
         tds_rules.push(td("output-entry-bottom", "", &[output_entry.text.clone()]));
       } else {
         tds_rules.push(td("output-entry-bottom-last", "", &[output_entry.text.clone()]));
       }
     }
     let tr_rule = tr("", tds_rules.as_mut_slice());
     trs_rules.push(tr_rule);
     i += 1;
   }
  }

  trs_rules
}
