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

//! Generator of DMN documentation.

use crate::svg::*;
use dmntk_model::model::*;

const HTML_TEMPLATE: &str = include_str!("template.html");
const SVG_CONTENT: &str = "#SVG_CONTENT#";
const PI_2: f64 = std::f64::consts::PI * 2.0;

/// Generates HTML documentation for DMN model.
pub fn generate(definitions: &Definitions) -> String {
  add_svg_content(HTML_TEMPLATE, definitions)
}

fn add_svg_content(html: &str, definitions: &Definitions) -> String {
  let mut svg_content = String::new();
  let indent = 0_usize;

  if let Some(dmndi) = definitions.dmndi() {
    let styles = svg_styles(&dmndi.styles);
    for diagram in &dmndi.diagrams {
      svg_content.push_str(&svg_begin(indent, &diagram.size));
      svg_content = format!("{}{}", svg_content, styles);

      for diagram_element in &diagram.diagram_elements {
        match diagram_element {
          DmnDiagramElement::DmnShape(shape) => {
            if let Some(dmn_element_ref) = &shape.dmn_element_ref {
              if let Some(decision) = definitions.decision_by_id(dmn_element_ref.as_str()) {
                svg_content.push_str(&svg_decision(indent, shape, decision));
              } else if let Some(input_data) = definitions.input_data_by_id(dmn_element_ref.as_str()) {
                svg_content.push_str(&svg_input_data(indent, shape, input_data));
              } else if let Some(business_knowledge) = definitions.business_knowledge_model_by_id(dmn_element_ref.as_str()) {
                svg_content.push_str(&svg_business_knowledge_model(indent, shape, business_knowledge));
              } else if let Some(knowledge_source) = definitions.knowledge_source_by_id(dmn_element_ref.as_str()) {
                svg_content.push_str(&svg_knowledge_source(indent, shape, knowledge_source));
              }
            }
          }
          DmnDiagramElement::DmnEdge(edge) => {
            svg_content = format!("{}\n{}", svg_content, svg_edge(edge));
          }
        }
      }

      svg_content.push_str(&svg_end(indent));
    }
  }
  html.replace(SVG_CONTENT, &svg_content)
}

/// Generate svg element for edge.
fn svg_edge(edge: &DmnEdge) -> String {
  let points: String = edge.way_points.iter().map(|w| format!("{},{} ", w.x, w.y)).collect();
  let start_point = &edge.way_points[edge.way_points.len() - 2];
  let end_point = &edge.way_points[edge.way_points.len() - 1];

  let points_for_arrowhead = format!(
    "{},{} {},{} {},{}",
    end_point.x + 1.0,
    end_point.y,
    end_point.x + 10.0,
    end_point.y - 3.0,
    end_point.x + 10.0,
    end_point.y + 3.0
  );
  let angle = get_angle(start_point, end_point);
  let mut edge = format!("<polyline points=\"{}\"/>", points);
  edge = format!(
    "{}<polygon points=\"{}\" transform=\"rotate({},{},{})\" style=\"fill:rgb(0,0,0);\" />",
    edge, points_for_arrowhead, angle, end_point.x, end_point.y
  );
  edge
}

fn svg_styles(styles: &[DmnStyle]) -> String {
  let mut svg_styles = String::from("<style>");
  for style in styles {
    if let Some(style_id) = &style.id {
      let fill_color = if let Some(fill_color) = &style.fill_color {
        format!("fill: {};", get_rgb_color(fill_color))
      } else {
        "".to_string()
      };
      let stroke_color = if let Some(stroke_color) = &style.stroke_color {
        format!("stroke: {};", get_rgb_color(stroke_color))
      } else {
        "".to_string()
      };
      let font_color = if let Some(font_color) = &style.font_color {
        format!("color: {};", get_rgb_color(font_color))
      } else {
        "".to_string()
      };
      let font_family = format!("font-family: {};", &style.font_family);
      let font_size = format!("font-size: {}px;", &style.font_size);
      let font_italic = if style.font_italic { "font-style: italic;" } else { "" }.to_string();
      let font_bold = if style.font_bold { "font-weight: bold;" } else { "" }.to_string();
      let font_underline = if style.font_underline { "text-decoration: underline;" } else { "" }.to_string();
      let font_strike_through = if style.font_strike_through { "overflow: visible;" } else { "" }.to_string();
      let label_horizontal_alignment = if let Some(_alignment) = &style.label_horizontal_alignment {
        "horizontal_alignment".to_string()
      } else {
        "".to_string()
      };
      let label_vertical_alignment = if let Some(_alignment) = &style.label_vertical_alignment {
        "vertical_alignment".to_string()
      } else {
        "".to_string()
      };
      let svg_style = format!(
        ".{} {{ {} {} {} {} {} {} {} {} {} {} {} }}",
        style_id,
        fill_color,
        stroke_color,
        font_color,
        font_family,
        font_size,
        font_italic,
        font_bold,
        font_underline,
        font_strike_through,
        label_horizontal_alignment,
        label_vertical_alignment
      );
      svg_styles = format!("{}{}", svg_styles, svg_style);
    }
  }
  svg_styles = format!("{}</style>\n", svg_styles);
  svg_styles
}

fn get_rgb_color(color: &DcColor) -> String {
  format!("rgb({},{},{})", color.red, color.green, color.blue)
}

/// Returns the rotation angle of an arrow.
fn get_angle(start: &DcPoint, end: &DcPoint) -> f64 {
  let x = end.x - start.x;
  let y = end.y - start.y;
  if x == 0.0 {
    return if y >= 0.0 { -90.0 } else { 90.0 };
  }
  let angle = ((y / x).atan() * 360.0) / PI_2;
  if x > 0.0 {
    if y >= 0.0 {
      angle - 180.0
    } else {
      angle + 180.0
    }
  } else {
    angle
  }
}
