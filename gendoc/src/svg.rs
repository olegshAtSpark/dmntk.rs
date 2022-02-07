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

//! SVG utilities.

use dmntk_model::model::*;

const NL: char = '\n';
const WS: &str = "";

/// Prepares `<svg>` element with specified dimension.
pub fn svg_begin(indent: usize, dimension: &Option<DcDimension>) -> String {
  if let Some(size) = dimension {
    format!(r#"{}{:i$}<svg width="{}" height="{}">{}"#, NL, WS, size.width, size.height, NL, i = indent)
  } else {
    format!(r#"{:i$}<svg>{}"#, WS, NL, i = indent)
  }
}

/// Prepares `</svg>` element.
pub fn svg_end(indent: usize) -> String {
  format!(r#"{:i$}</svg>{}"#, WS, NL, i = indent)
}

/// Prepares decision shape.
pub fn svg_decision(mut indent: usize, shape: &DmnShape, decision: &Decision) -> String {
  indent += 4;
  let mut svg_content = String::new();
  let text = get_label_text(shape, decision.name());
  let _text_position = get_text_center_position(&shape.bounds);
  let shape_class = get_shape_shared_style_id(shape);
  let _label_class = get_shape_label_shared_style_id(shape);
  svg_content.push_str(&format!(
    r#"{:i$}<rect width="{}" height="{}" x="{}" y="{}" class="{}"/>{}"#,
    WS,
    shape.bounds.width,
    shape.bounds.height,
    shape.bounds.x,
    shape.bounds.y,
    shape_class,
    NL,
    i = indent
  ));

  // svg_content.push_str(&format!(
  //   r#"{:i$}<text x="{}" y="{}" dominant-baseline="middle" text-anchor="middle" class="{}" fill="black" stroke="none">{}</text>{}"#,
  //   WS,
  //   text_position.0,
  //   text_position.1,
  //   label_class,
  //   text,
  //   NL,
  //   i = indent
  // ));

  //<foreignObject x="20" y="20" width="160" height="160"><div>kuku</div></foreignObject>
  svg_content.push_str(&format!(
    r#"{:i$}<foreignObject width="{}" height="{}" x="{}" y="{}"><div style="display:table;height:100%;width:100%;text-align:center;"><span style="display:table-cell;vertical-align:middle;">{}</span></div></foreignObject>{}"#,
    WS,
    shape.bounds.width,
    shape.bounds.height,
    shape.bounds.x,
    shape.bounds.y,
    text,
    NL,
    i = indent
  ));

  svg_content
}

/// Prepares input data shape.
pub fn svg_input_data(mut indent: usize, shape: &DmnShape, input_data: &InputData) -> String {
  indent += 4;
  let mut svg_content = String::new();
  let rxy = shape.bounds.height / 2.0;
  let text = get_label_text(shape, input_data.name());
  let text_position = get_text_center_position(&shape.bounds);
  let shape_class = get_shape_shared_style_id(shape);
  let label_class = get_shape_label_shared_style_id(shape);
  svg_content.push_str(&format!(
    r#"{:i$}<rect width="{}" height="{}" x="{}" y="{}" rx="{}" ry="{}" class="{}"/>{}"#,
    WS,
    shape.bounds.width,
    shape.bounds.height,
    shape.bounds.x,
    shape.bounds.y,
    rxy,
    rxy,
    shape_class,
    NL,
    i = indent
  ));
  svg_content.push_str(&format!(
    r#"{:i$}<text x="{}" y="{}" dominant-baseline="middle" text-anchor="middle" class="{}" fill="black" stroke="none">{}</text>{}"#,
    WS,
    text_position.0,
    text_position.1,
    label_class,
    text,
    NL,
    i = indent
  ));
  svg_content
}

/// Calculates the position of the centered text inside a shape.
fn get_text_center_position(bounds: &DcBounds) -> (f64, f64) {
  let x = bounds.x + (bounds.width / 2.0);
  let y = bounds.y + (bounds.height / 2.0);
  (x, y)
}

/// Returns the text of the label associated with the shape,
/// when no label, then returns the specified name.
fn get_label_text(shape: &DmnShape, name: &str) -> String {
  if let Some(label) = &shape.label {
    if let Some(label_text) = &label.text {
      return label_text.to_string();
    }
  }
  name.to_string()
}

fn get_shape_shared_style_id(shape: &DmnShape) -> String {
  if let Some(style_id) = &shape.shared_style {
    style_id.to_string()
  } else {
    String::new()
  }
}

fn get_shape_label_shared_style_id(shape: &DmnShape) -> String {
  if let Some(label) = &shape.label {
    if let Some(style_id) = &label.shared_style {
      style_id.to_string()
    } else {
      String::new()
    }
  } else {
    String::new()
  }
}
