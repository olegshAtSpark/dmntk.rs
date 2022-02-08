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
const AMPLITUDE: f64 = 20.0;

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
  let shape_class = get_shape_shared_style_id(shape);
  let label_class = get_shape_label_shared_style_id(shape);
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
  svg_content.push_str(&svg_multiline_text(indent, &shape.bounds, &label_class, &text));
  svg_content
}

/// Prepares business knowledge model shape.
pub fn svg_business_knowledge_model(mut indent: usize, shape: &DmnShape, business_knowledge: &BusinessKnowledgeModel) -> String {
  indent += 4;
  let mut svg_content = String::new();
  let text = get_label_text(shape, business_knowledge.name());
  let shape_class = get_shape_shared_style_id(shape);
  let label_class = get_shape_label_shared_style_id(shape);
  let (x, y, w, h) = (shape.bounds.x, shape.bounds.y, shape.bounds.width, shape.bounds.height);
  let points = format!(
    "{},{} {},{} {},{} {},{} {},{} {},{}",
    x,
    y + 15.0,
    x + 15.0,
    y,
    x + w,
    y,
    x + w,
    y + h - 15.0,
    x + w - 15.0,
    y + h,
    x,
    y + h
  );
  svg_content.push_str(&format!(r#"{:i$}<polygon points="{}" class="{}"/>{}"#, WS, points, shape_class, NL, i = indent));
  svg_content.push_str(&svg_multiline_text(indent, &shape.bounds, &label_class, &text));
  svg_content
}

/// Prepares knowledge source shape.
pub fn svg_knowledge_source(mut indent: usize, shape: &DmnShape, knowledge_source: &KnowledgeSource) -> String {
  indent += 4;
  let mut svg_content = String::new();
  let text = get_label_text(shape, knowledge_source.name());
  let path = get_path_to_knowledge_source(&shape.bounds);
  let shape_class = get_shape_shared_style_id(shape);
  let label_class = get_shape_label_shared_style_id(shape);
  svg_content.push_str(&format!("<path d=\"{}\" class=\"{}\"/>", path, shape_class));
  let bounds = DcBounds {
    height: shape.bounds.height - AMPLITUDE / 2.0,
    ..shape.bounds
  };
  svg_content.push_str(&svg_multiline_text(indent, &bounds, &label_class, &text));
  svg_content
}

fn get_path_to_knowledge_source(bounds: &DcBounds) -> String {
  let period_div_2 = AMPLITUDE / 2.0;
  let curve_base_height = bounds.y + bounds.height - period_div_2;
  let width_div_4: f64 = bounds.width / 4.0;
  let width_div_2: f64 = bounds.width / 2.0;

  let mut path = format!("M {} {}", bounds.x, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, bounds.y);
  path = format!("{} L {} {}", path, bounds.x + bounds.width, curve_base_height);
  path = format!(
    "{} C {},{} {},{} {},{}",
    path,
    bounds.x + bounds.width,
    curve_base_height,
    bounds.x + bounds.width - width_div_4,
    curve_base_height - AMPLITUDE,
    bounds.x + bounds.width - width_div_2,
    curve_base_height
  );
  path = format!(
    "{} C {},{} {},{} {},{}",
    path,
    bounds.x + bounds.width - width_div_2,
    curve_base_height,
    bounds.x + width_div_4,
    curve_base_height + AMPLITUDE,
    bounds.x,
    curve_base_height
  );
  path = format!("{} L {} {} Z", path, bounds.x, bounds.y);
  path
}

/// Prepares input data shape.
pub fn svg_input_data(mut indent: usize, shape: &DmnShape, input_data: &InputData) -> String {
  indent += 4;
  let mut svg_content = String::new();
  let radius = shape.bounds.height / 2.0;
  let text = get_label_text(shape, input_data.name());
  let shape_class = get_shape_shared_style_id(shape);
  let label_class = get_shape_label_shared_style_id(shape);
  svg_content.push_str(&format!(
    r#"{:i$}<rect width="{}" height="{}" x="{}" y="{}" rx="{}" ry="{}" class="{}"/>{}"#,
    WS,
    shape.bounds.width,
    shape.bounds.height,
    shape.bounds.x,
    shape.bounds.y,
    radius,
    radius,
    shape_class,
    NL,
    i = indent
  ));
  svg_content.push_str(&svg_multiline_text(indent, &shape.bounds, &label_class, &text));
  svg_content
}

/// Prepares SVG object containing multiline text.
///
/// Text given in argument `text` is placed in the following construct:
///
/// ```text
/// <foreignObject>
///   <div>
///     <span>...text...</span>
///   </div>
/// </foreignObject>
/// ```
/// The `div` and `span` elements have such styles set, that the content is displayed as a table cell.
/// Text in table cell is centered horizontally and middle aligned vertically.
///
pub fn svg_multiline_text(indent: usize, bounds: &DcBounds, label_class: &str, text: &str) -> String {
  format!(
    r#"{:i$}<foreignObject x="{}" y="{}" width="{}" height="{}"><div style="display:table;height:100%;width:100%;text-align:center;"><span style="display:table-cell;vertical-align:middle;" class="{}">{}</span></div></foreignObject>{}"#,
    WS,
    bounds.x,
    bounds.y,
    bounds.width,
    bounds.height,
    label_class,
    text,
    NL,
    i = indent
  )
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
