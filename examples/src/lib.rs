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

mod compatibility;
pub mod decision_logic;
pub mod decision_tables;
mod examples;
pub mod input_data;
pub mod item_definition;

pub use compatibility::*;
pub use examples::valid::*;
pub use examples::*;

#[cfg(test)]
mod tests {
  #[test]
  fn test_calculate_decision_table_variants() {
    let decision_table_orientation = ["horizontal", "vertical", "crosstab"];
    let information_item_name = ["present", "absent"];
    let output_label = ["present", "absent"];
    let allowed_values = ["absent", "present"];
    let inputs = ["absent", "single", "double", "multiple"];
    let outputs = ["single", "double", "multiple"];
    let annotations = ["absent", "single", "double", "multiple"];
    let total_variants = decision_table_orientation.len()
      * information_item_name.len()
      * output_label.len()
      * allowed_values.len()
      * inputs.len()
      * outputs.len()
      * annotations.len();
    assert_eq!(1152, total_variants);
    println!("┌─────────────┬─────────────┬─────────┬─────────┬──────────┬──────────┬─────────────┬─────────┬────────┐",);
    println!("│  Preferred  │ Information │ Output  │ Allowed │  Inputs  │ Outputs  │ Annotations │ Example │ Status │",);
    println!("│ orientation │  item name  │  label  │ values  │          │          │             │         │        │",);
    println!("├─────────────┼─────────────┼─────────┼─────────┼──────────┼──────────┼─────────────┼─────────┼────────┤",);
    let mut counter = 1;
    for v_decision_table_orientation in decision_table_orientation {
      for v_information_item_name in information_item_name {
        for v_output_label in output_label {
          for v_allowed_values in allowed_values {
            for v_inputs in inputs {
              for v_outputs in outputs {
                for v_annotations in annotations {
                  println!(
                    "│{:^13}│{:^13}│{:^9}│{:^9}│{:^10}│{:^10}│{:^13}│ DT_{:04} │        │",
                    v_decision_table_orientation, v_information_item_name, v_output_label, v_allowed_values, v_inputs, v_outputs, v_annotations, counter
                  );
                  counter += 1;
                }
              }
            }
          }
        }
      }
    }
    println!("└─────────────┴─────────────┴─────────┴─────────┴──────────┴──────────┴─────────────┴─────────┴────────┘",);
  }
}
