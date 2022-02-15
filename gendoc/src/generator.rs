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

use dmntk_model::model::*;

use crate::decision_table::generate_decision_table;
use crate::svg::*;

const DMN_MODEL_TEMPLATE: &str = include_str!("templates/dmn_model_template.html");
const SVG_CONTENT: &str = "#SVG_CONTENT#";
const HTML_CONTENT: &str = "#HTML_CONTENT#";
const PI_2: f64 = std::f64::consts::PI * 2.0;

/// Generates HTML documentation for DMN model.
pub fn definitions_to_html(definitions: &Definitions) -> String {
  let html = add_svg_content(DMN_MODEL_TEMPLATE, definitions);
  add_html_content(&html, definitions)
}

fn add_svg_content(html: &str, definitions: &Definitions) -> String {
  let mut diagrams_content = String::new();
  let indent = 0_usize;
  if let Some(dmndi) = definitions.dmndi() {
    let styles = svg_styles(&dmndi.styles);
    for diagram in &dmndi.diagrams {
      let mut svg_content = String::new();
      // prepare the name of the diagram
      svg_content.push_str(r#"<section>"#);
      svg_content.push_str(&format!(r#"<h2>{}</h2>"#, diagram.name));
      // prepare diagram graphics
      svg_content.push_str(&svg_begin(indent, &diagram.size));
      svg_content.push_str(&styles);
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
            if let Some(id) = &edge.dmn_element_ref {
              if let Some(requirement) = definitions.requirements_by_id().get(id) {
                match requirement {
                  Requirement::Information(_) => {
                    // information requirement is depicted as solid line with dark-filled arrow
                    svg_content = format!("{}\n{}", svg_content, svg_edge_solid_with_black_arrow(&edge.way_points))
                  }
                  Requirement::Knowledge(_) => {
                    // knowledge requirement is depicted as dashed line with thin arrow, or
                    // with dashed line with black-filled arrow when required knowledge is decision service
                    svg_content = format!("{}\n{}", svg_content, svg_edge_dashed_with_thin_arrow(&edge.way_points))
                  }
                  Requirement::Authority(_) => {
                    // authority requirement is depicted as dashed line with dark-filled end-point
                    svg_content = format!("{}\n{}", svg_content, svg_edge_dashed_with_end_point(&edge.way_points))
                  }
                }
              }
            }
          }
        }
      }
      svg_content.push_str(&svg_end(indent));
      svg_content.push_str(r#"</section>"#);
      svg_content.push_str("\n<br/>");
      diagrams_content.push_str(&svg_content);
    }
  }
  html.replace(SVG_CONTENT, &diagrams_content)
}

/// Prepare solid edge line with black filled arrow.  
fn svg_edge_solid_with_black_arrow(way_points: &[DcPoint]) -> String {
  let mut svg_content = String::new();
  // prepare line
  let points = way_points.iter().map(|w| format!("{},{} ", w.x, w.y)).collect::<String>();
  svg_content.push_str(&format!(r#"<polyline points="{}" stroke="black"/>"#, points));
  // prepare arrow
  let start_point = &way_points[way_points.len() - 2];
  let end_point = &way_points[way_points.len() - 1];
  let points_ending = format!(
    "{},{} {},{} {},{}",
    end_point.x,
    end_point.y,
    end_point.x + 12.0,
    end_point.y - 4.0,
    end_point.x + 12.0,
    end_point.y + 4.0
  );
  let angle = get_angle(start_point, end_point);
  svg_content.push_str(&format!(
    r#"<polygon points="{}" transform="rotate({},{},{})" fill="black" stroke="none"/>"#,
    points_ending, angle, end_point.x, end_point.y
  ));
  svg_content
}

/// Prepare dashed edge line with thin arrow.  
fn svg_edge_dashed_with_thin_arrow(way_points: &[DcPoint]) -> String {
  let mut svg_content = String::new();
  // prepare line
  let points = way_points.iter().map(|w| format!("{},{} ", w.x, w.y)).collect::<String>();
  svg_content.push_str(&format!(r#"<polyline points="{}" stroke-dasharray="5 3"/>"#, points));
  // prepare arrow
  let start_point = &way_points[way_points.len() - 2];
  let end_point = &way_points[way_points.len() - 1];
  let path = format!(
    "M {},{} l {},{} M {},{} l {}, {}",
    end_point.x, end_point.y, 12.0, -4.0, end_point.x, end_point.y, 12.0, 4.0
  );
  let angle = get_angle(start_point, end_point);
  svg_content.push_str(&format!(
    r#"<path d="{}" transform="rotate({},{},{})" fill="none" stroke="black"/>"#,
    path, angle, end_point.x, end_point.y
  ));
  svg_content
}

/// Prepare dashed edge line with black end-point.  
fn svg_edge_dashed_with_end_point(way_points: &[DcPoint]) -> String {
  let mut svg_content = String::new();
  // prepare line
  let points = way_points.iter().map(|w| format!("{},{} ", w.x, w.y)).collect::<String>();
  svg_content.push_str(&format!(r#"<polyline points="{}" stroke="black" stroke-dasharray="5 3"/>"#, points));
  let end_point = &way_points[way_points.len() - 1];
  svg_content.push_str(&format!(
    r#"<circle cx="{}" cy="{}" r="4" fill="black" stroke="none"/>"#,
    end_point.x, end_point.y
  ));
  svg_content
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

fn add_html_content(html: &str, definitions: &Definitions) -> String {
  let mut html_content = String::new();
  if let Some(dmndi) = definitions.dmndi() {
    for diagram in &dmndi.diagrams {
      for diagram_element in &diagram.diagram_elements {
        if let DmnDiagramElement::DmnShape(shape) = diagram_element {
          if let Some(dmn_element_ref) = &shape.dmn_element_ref {
            if let Some(decision) = definitions.decision_by_id(dmn_element_ref.as_str()) {
              if let Some(decision_logic) = decision.decision_logic() {
                match decision_logic {
                  ExpressionInstance::Context(_) => {}
                  ExpressionInstance::DecisionTable(decision_table) => {
                    html_content.push_str(&generate_decision_table(decision_table));
                  }
                  ExpressionInstance::FunctionDefinition(_) => {}
                  ExpressionInstance::Invocation(_) => {}
                  ExpressionInstance::LiteralExpression(_) => {}
                  ExpressionInstance::Relation(_) => {}
                }
              }
            }
          }
        }
      }
    }
  }

  html.replace(HTML_CONTENT, &html_content)
}
