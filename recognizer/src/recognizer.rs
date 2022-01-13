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

//! Recognizer.

use crate::canvas;
use crate::canvas::Canvas;
use crate::errors::*;
use crate::plane::{HitPolicyPlacement, Plane, RuleNumbersPlacement};
use dmntk_common::Result;
use dmntk_model::model::{DecisionTableOrientation, HitPolicy};

/// Global flag indicating if the recognizing process should be verbose
/// and trace messages should be printed to standard output.
/// Used only during development.
const TRACE_ENABLED: bool = false;

pub struct Recognizer {
  /// Canvas used during recognition process.
  pub canvas: Canvas,
  /// Plane used during recognition process.
  pub plane: Plane,
  /// Optional information item name.
  pub information_item_name: Option<String>,
  /// Placement of the hit policy.
  pub hit_policy_placement: HitPolicyPlacement,
  /// Detected hit policy.
  pub hit_policy: HitPolicy,
  /// Placement of rule numbers.
  pub rule_numbers_placement: RuleNumbersPlacement,
  /// Detected table orientation.
  pub orientation: DecisionTableOrientation,
  /// Number of recognized input clauses.
  pub input_clause_count: usize,
  /// List of input expressions.
  pub input_expressions: Vec<String>,
  /// List of input values.
  pub input_values: Vec<String>,
  /// Matrix of input entries.
  pub input_entries: Vec<Vec<String>>,
  /// Number of recognized output clauses.
  pub output_clause_count: usize,
  /// Detected output label.
  pub output_label: Option<String>,
  /// List of output component names.
  pub output_components: Vec<String>,
  /// List of output values.
  pub output_values: Vec<String>,
  /// Matrix of output entries.
  pub output_entries: Vec<Vec<String>>,
  /// Number of recognized annotation clauses.
  pub annotation_clause_count: usize,
  /// List of annotations.
  pub annotations: Vec<String>,
  /// Matrix of annotation entries.
  pub annotation_entries: Vec<Vec<String>>,
  /// Number of recognized rules.
  pub rule_count: usize,
}

impl Recognizer {
  /// Recognizes the decision table defined as text.
  pub fn recognize(text: &str) -> Result<Recognizer> {
    let mut canvas = canvas::scan(text)?;
    let information_item_name = canvas.information_item_name.clone();
    let plane = canvas.plane()?;
    let mut recognizer = Recognizer {
      canvas,
      plane,
      information_item_name,
      hit_policy_placement: HitPolicyPlacement::NotPresent,
      hit_policy: HitPolicy::Unique,
      rule_numbers_placement: RuleNumbersPlacement::NotPresent,
      orientation: DecisionTableOrientation::CrossTable,
      input_clause_count: 0,
      input_expressions: vec![],
      input_values: vec![],
      input_entries: vec![],
      output_clause_count: 0,
      output_label: None,
      output_components: vec![],
      output_values: vec![],
      output_entries: vec![],
      annotation_clause_count: 0,
      annotations: vec![],
      annotation_entries: vec![],
      rule_count: 0,
    };
    recognizer.recognize_table_components()?;
    recognizer.trace();
    Ok(recognizer)
  }

  /// Recognizes the decision table components based on table orientation.
  pub fn recognize_table_components(&mut self) -> Result<()> {
    self.recognize_orientation()?;
    match self.orientation {
      DecisionTableOrientation::RuleAsRow => {
        self.plane.remove_first_column();
        self.recognize_horizontal_table()?;
      }
      DecisionTableOrientation::RuleAsColumn => {
        self.plane.remove_last_row();
        self.plane.pivot();
        self.recognize_horizontal_table()?;
      }
      DecisionTableOrientation::CrossTable => {
        self.recognize_crosstab_table()?;
      }
    }
    Ok(())
  }

  /// Recognizes decision table components from horizontally oriented plane.
  /// Vertical decision tables are pivot of horizontal decision tables.
  fn recognize_horizontal_table(&mut self) -> Result<()> {
    let r = self.plane.horz_input_clause_rect()?;
    // assign the number of recognized input clauses
    self.input_clause_count = r.width();
    // detect if the input values are present in decision table
    let input_values_present: bool;
    match r.height() {
      1 => {
        // by single row there are no input values, only input expressions
        input_values_present = false;
      }
      2 => {
        // by two rows when regions in each column are the same, then there are no input/output values
        // otherwise the are input/output values provided
        input_values_present = !self.plane.equal_regions_in_columns(&r)?;
      }
      3 => {
        // by three rows there must be always input/output values provided, just checking if the
        // two upper rows contains the same regions - input expressions
        if !self.plane.unique_regions_in_columns(&r.inc_top(1))? {
          return Err(invalid_input_expressions());
        }
        input_values_present = true;
      }
      _ => return Err(too_many_rows_in_input_clause()),
    }
    // retrieve input expressions from plane
    for col in r.left..r.right {
      self.input_expressions.push(self.plane.region_text(0, col)?);
    }
    // retrieve input values from plane
    if input_values_present {
      for col in r.left..r.right {
        self.input_values.push(self.plane.region_text(r.bottom - 1, col)?);
      }
    }
    // retrieve input entries from plane
    let r = self.plane.horz_input_entries_rect()?;
    for row in r.top..r.bottom {
      self.input_entries.push(vec![]);
      for col in r.left..r.right {
        self.input_entries.last_mut().unwrap().push(self.plane.region_text(row, col)?);
      }
    }
    // retrieve output clause
    let r = self.plane.horz_output_clause_rect()?;
    // assign the number of recognized output clauses
    self.output_clause_count = r.width();
    match r.width() {
      0 => {
        // no output columns => report an error
        return Err(no_output_clause());
      }
      1 => {
        // single output
        match r.height() {
          1 => {
            // only output label
            self.output_label = Some(self.plane.region_text(r.top, r.left)?);
          }
          2 => {
            if input_values_present && !self.plane.equal_regions(&r)? {
              // output label and output values
              self.output_label = Some(self.plane.region_text(r.top, r.left)?);
              self.output_values.push(self.plane.region_text(r.top + 1, r.left)?)
            } else {
              // invalid output clause
              return Err(plane_invalid_output_clause());
            }
          }
          _ => return Err(too_many_rows_in_input_clause()),
        }
      }
      _ => {
        // multiple outputs
        match r.height() {
          1 => {
            // only component names
            for col in r.left..r.right {
              self.output_components.push(self.plane.region_text(r.top, col)?);
            }
          }
          2 => {
            if input_values_present {
              // component names and output values
              for col in r.left..r.right {
                self.output_components.push(self.plane.region_text(r.top, col)?);
              }
              for col in r.left..r.right {
                self.output_values.push(self.plane.region_text(r.top + 1, col)?);
              }
            } else {
              // output label and component names
              self.output_label = Some(self.plane.region_text(r.top, r.left)?);
              for col in r.left..r.right {
                self.output_components.push(self.plane.region_text(r.top + 1, col)?);
              }
            }
          }
          3 => {
            // output label, component names and output values
            self.output_label = Some(self.plane.region_text(r.top, r.left)?);
            for col in r.left..r.right {
              self.output_components.push(self.plane.region_text(r.top + 1, col)?);
            }
            for col in r.left..r.right {
              self.output_values.push(self.plane.region_text(r.top + 2, col)?);
            }
          }
          _ => return Err(too_many_rows_in_input_clause()),
        }
      }
    }
    // retrieve output entries
    let r = self.plane.horz_output_entries_rect()?;
    for row in r.top..r.bottom {
      self.output_entries.push(vec![]);
      for col in r.left..r.right {
        self.output_entries.last_mut().unwrap().push(self.plane.region_text(row, col)?);
      }
    }
    // retrieve annotation clauses
    let r = self.plane.horz_annotation_clauses_rect()?;
    // assign the number of recognized annotation clauses
    self.annotation_clause_count = r.width();
    for col in r.left..r.right {
      self.annotations.push(self.plane.region_text(r.top, col)?);
    }
    // retrieve annotation entries
    let r = self.plane.horz_annotation_entries_rect()?;
    for row in r.top..r.bottom {
      self.annotation_entries.push(vec![]);
      for col in r.left..r.right {
        self.annotation_entries.last_mut().unwrap().push(self.plane.region_text(row, col)?);
      }
    }
    Ok(())
  }

  /// Recognizes decision table components from crosstab oriented plane.
  fn recognize_crosstab_table(&mut self) -> Result<()> {
    // TODO implement crosstab recognition
    self.rule_count = 0; // TODO properly recognize the total number of rules!
    Err(recognizing_cross_tab_not_supported_yet())
  }

  /// Recognizes the orientation of decision table.
  fn recognize_orientation(&mut self) -> Result<()> {
    self.hit_policy_placement = self.plane.recognize_hit_policy_placement()?;
    self.rule_numbers_placement = self.plane.recognize_rule_numbers_placement()?;
    if self.plane.horizontal_double_crossing().is_some() {
      // horizontal orientation
      if self.hit_policy_placement.is_top_left() {
        if self.rule_numbers_placement.is_left_below() {
          self.hit_policy = self.hit_policy_placement.hit_policy();
          self.orientation = DecisionTableOrientation::RuleAsRow;
          self.rule_count = self.rule_numbers_placement.rule_count();
          Ok(())
        } else {
          Err(expected_left_below_rule_numbers_placement())
        }
      } else {
        Err(expected_top_left_hit_policy_placement())
      }
    } else if self.plane.vertical_double_crossing().is_some() {
      // vertical orientation
      if self.hit_policy_placement.is_bottom_left() {
        if self.rule_numbers_placement.is_right_after() {
          self.hit_policy = self.hit_policy_placement.hit_policy();
          self.orientation = DecisionTableOrientation::RuleAsColumn;
          self.rule_count = self.rule_numbers_placement.rule_count();
          Ok(())
        } else {
          Err(expected_right_after_rule_numbers_placement())
        }
      } else {
        Err(expected_bottom_left_hit_policy_placement())
      }
    } else {
      // detect the orientation
      match self.hit_policy_placement {
        HitPolicyPlacement::TopLeft(_) => {
          if self.rule_numbers_placement.is_left_below() {
            self.hit_policy = self.hit_policy_placement.hit_policy();
            self.orientation = DecisionTableOrientation::RuleAsRow;
            self.rule_count = self.rule_numbers_placement.rule_count();
            Ok(())
          } else {
            Err(expected_left_below_rule_numbers_placement())
          }
        }
        HitPolicyPlacement::BottomLeft(_) => {
          if self.rule_numbers_placement.is_right_after() {
            self.hit_policy = self.hit_policy_placement.hit_policy();
            self.orientation = DecisionTableOrientation::RuleAsColumn;
            self.rule_count = self.rule_numbers_placement.rule_count();
            Ok(())
          } else {
            Err(expected_right_after_rule_numbers_placement())
          }
        }
        HitPolicyPlacement::NotPresent => {
          if self.rule_numbers_placement.is_not_present() {
            self.hit_policy = self.hit_policy_placement.hit_policy();
            self.orientation = DecisionTableOrientation::CrossTable;
            self.rule_count = 0; // will be recognized later
            Ok(())
          } else {
            Err(expected_no_rule_numbers_present())
          }
        }
      }
    }
  }

  /// Traces the result of decision table recognition.
  pub fn trace(&self) {
    if TRACE_ENABLED {
      print!("\n>> input expressions:\n|");
      for text in &self.input_expressions {
        self.trace_line(text);
      }
      print!("\n\n>> input values:\n|");
      for text in &self.input_values {
        self.trace_line(text);
      }
      print!("\n\n>> input entries:\n");
      for row in &self.input_entries {
        print!("|");
        for text in row {
          self.trace_line(text);
        }
        println!()
      }
      print!("\n>> output label:\n|");
      if let Some(text) = &self.output_label {
        self.trace_line(text);
      }
      print!("\n\n>> output components:\n|");
      for text in &self.output_components {
        self.trace_line(text);
      }
      print!("\n\n>> output values:\n|");
      for text in &self.output_values {
        self.trace_line(text);
      }
      print!("\n\n>> output entries:\n");
      for row in &self.output_entries {
        print!("|");
        for text in row {
          self.trace_line(text);
        }
        println!()
      }
      print!("\n\n>> annotations:\n|");
      for text in &self.annotations {
        self.trace_line(text);
      }
      print!("\n\n>> annotation entries:\n");
      for row in &self.annotation_entries {
        print!("|");
        for text in row {
          self.trace_line(text);
        }
        println!()
      }
    }
  }

  /// Displays a single tracing line.
  fn trace_line(&self, text: &str) {
    print!("{}|", text.replace("\n", " ").trim());
  }
}
