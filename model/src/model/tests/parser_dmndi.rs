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

use crate::model::*;

///
fn get_shape(elements: &[DmnDiagramElement], index: usize) -> Option<&DmnShape> {
  let element = elements.get(index).unwrap();
  match element {
    DmnDiagramElement::DmnShape(dmn_shape) => Some(dmn_shape),
    _ => None,
  }
}

///
fn get_edge(elements: &[DmnDiagramElement], index: usize) -> Option<&DmnEdge> {
  let element = elements.get(index).unwrap();
  match element {
    DmnDiagramElement::DmnEdge(dmn_edge) => Some(dmn_edge),
    _ => None,
  }
}

#[test]
fn _2_0001() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0001).unwrap();
  assert_eq!("_c910c9ba-c584-4ac9-a773-1e6de185cd85", definitions.id().as_ref().unwrap().as_str());
  let dmndi = definitions.dmndi.unwrap();
  // there are no shared styles defined
  assert_eq!(0, dmndi.styles.len());
  // there is a single diagram defined
  assert_eq!(1, dmndi.diagrams.len());
  // there is a diagram id
  assert_eq!("_d3a3312e-5924-4f7b-ac0e-232ef9203ff6", dmndi.diagrams.get(0).unwrap().id.as_ref().unwrap());
  // there is a height of a diagram
  assert!(650.0_f64.eq(&dmndi.diagrams.get(0).unwrap().size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(650.0_f64.eq(&dmndi.diagrams.get(0).unwrap().size.as_ref().unwrap().width));
  // there are a 3 diagram elements
  assert_eq!(3, dmndi.diagrams.get(0).unwrap().diagram_elements.len());
  // there is a first shape's id
  let shape_0 = get_shape(&dmndi.diagrams.get(0).unwrap().diagram_elements, 0).unwrap();
  assert_eq!("_ebf33cfc-0ee3-4708-af8b-91c52237b7d6", shape_0.id.as_ref().unwrap());
  // there is a first shape's dmn_element_ref
  assert_eq!("_75b3add2-4d36-4a19-a76c-268b49b2f436", shape_0.dmn_element_ref.as_ref().unwrap());
  // there is a first shape's bound - height
  assert_eq!(60.0, shape_0.bounds.height);
  // there is a first shape's bound - width
  assert_eq!(150.0, shape_0.bounds.width);
  // there is a first shape's bound - x
  assert_eq!(150.0, shape_0.bounds.x);
  // there is a first shape's bound - y
  assert!(150.0_f64.eq(&shape_0.bounds.y));
  // there is a second shape's id
  let shape_1 = get_shape(&dmndi.diagrams.get(0).unwrap().diagram_elements, 1).unwrap();
  assert_eq!("_48ea7a1d-2575-4cb7-8b63-8baa4cb3b371", shape_1.id.as_ref().unwrap());
  // there is a second shape's dmn_element_ref
  assert_eq!("_cba86e4d-e91c-46a2-9176-e9adf88e15db", shape_1.dmn_element_ref.as_ref().unwrap());
  // there is a second shape's bound - height
  assert!(60.0_f64.eq(&shape_1.bounds.height));
  // there is a second shape's bound - width
  assert!(150.0_f64.eq(&shape_1.bounds.width));
  // there is a second shape's bound - x
  assert!(150.0_f64.eq(&shape_1.bounds.x));
  // there is a second shape's bound - y
  assert!(330.0_f64.eq(&shape_1.bounds.y));
  // there is a first edge's id
  let edge_0 = get_edge(&dmndi.diagrams.get(0).unwrap().diagram_elements, 2).unwrap();
  assert_eq!("_e9a73517-0ba2-4b31-b308-82279ae21591", edge_0.id.as_ref().unwrap());
  // there is a first edge's dmn_element_ref
  assert_eq!("_8c935b50-10b7-426b-80a9-dddb4264b4a9", edge_0.dmn_element_ref.as_ref().unwrap());
  // there is a first waypoint's x
  assert_eq!(225.0, edge_0.way_points.get(0).unwrap().x);
  // there is a first waypoint's y
  assert_eq!(330.0, edge_0.way_points.get(0).unwrap().y);
  // there is a second waypoint's x
  assert_eq!(225.0, edge_0.way_points.get(1).unwrap().x);
  // there is a second waypoint's y
  assert_eq!(210.0, edge_0.way_points.get(1).unwrap().y);
}

#[test]
fn _2_0002() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0002).unwrap();
  assert_eq!("_072005e3-2635-47c9-8dec-5aca4b869376", definitions.id().as_ref().unwrap().as_str());
  let dmndi = definitions.dmndi.unwrap();
  // there are no shared styles defined
  assert_eq!(0, dmndi.styles.len());
  // there is a single diagram defined
  assert_eq!(1, dmndi.diagrams.len());
  // there is a diagram id
  assert_eq!("_0002-input-data-number_D1", dmndi.diagrams.get(0).unwrap().id.as_ref().unwrap());
  // there is a height of a diagram
  assert_eq!(650.0, dmndi.diagrams.get(0).unwrap().size.as_ref().unwrap().height);
  // there is a width of a diagram
  assert_eq!(650.0, dmndi.diagrams.get(0).unwrap().size.as_ref().unwrap().width);
  // there are a 3 diagram elements
  assert_eq!(3, dmndi.diagrams.get(0).unwrap().diagram_elements.len());
  // there is a first shape's id
  let shape_0 = get_shape(&dmndi.diagrams.get(0).unwrap().diagram_elements, 0).unwrap();
  assert_eq!("_0002-input-data-number_s1", shape_0.id.as_ref().unwrap());
  // there is a first shape's dmn_element_ref
  assert_eq!("d_YearlySalary", shape_0.dmn_element_ref.as_ref().unwrap());
  // there is a first shape's bound - height
  assert_eq!(61.0, shape_0.bounds.height);
  // there is a first shape's bound - width
  assert_eq!(154.0, shape_0.bounds.width);
  // there is a first shape's bound - x
  assert_eq!(150.0, shape_0.bounds.x);
  // there is a first shape's bound - y
  assert_eq!(150.0, shape_0.bounds.y);
  // there is a second shape's id
  let shape_1 = get_shape(&dmndi.diagrams.get(0).unwrap().diagram_elements, 1).unwrap();
  assert_eq!("_0002-input-data-number_s2", shape_1.id.as_ref().unwrap());
  // there is a second shape's dmn_element_ref
  assert_eq!("i_MonthlySalary", shape_1.dmn_element_ref.as_ref().unwrap());
  // there is a second shape's bound - height
  assert_eq!(60.0, shape_1.bounds.height);
  // there is a second shape's bound - width
  assert_eq!(153.0, shape_1.bounds.width);
  // there is a second shape's bound - x
  assert_eq!(151.0, shape_1.bounds.x);
  // there is a second shape's bound - y
  assert_eq!(331.0, shape_1.bounds.y);
  // there is a first edge's id
  let edge_0 = get_edge(&dmndi.diagrams.get(0).unwrap().diagram_elements, 2).unwrap();
  assert_eq!("_0002-input-data-number_e1", edge_0.id.as_ref().unwrap());
  // there is a first edge's dmn_element_ref
  assert_eq!("_94534179-9eda-4522-b970-aaffcb4e0c97", edge_0.dmn_element_ref.as_ref().unwrap());
  // there is a first waypoint's x
  assert_eq!(227.0, edge_0.way_points.get(0).unwrap().x);
  // there is a first waypoint's y
  assert_eq!(331.0, edge_0.way_points.get(0).unwrap().y);
  // there is a second waypoint's x
  assert_eq!(227.0, edge_0.way_points.get(1).unwrap().x);
  // there is a second waypoint's y
  assert_eq!(211.0, edge_0.way_points.get(1).unwrap().y);
}

#[test]
fn _3_0086() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0086).unwrap();
  assert_eq!("_8bb2d2bb-b981-415c-a5c3-cdb255f2d967", definitions.id().as_ref().unwrap().as_str());
  let dmndi = definitions.dmndi.unwrap();
  // there are no shared styles defined
  assert_eq!(1, dmndi.styles.len());
  // there is a single diagram defined
  assert_eq!(1, dmndi.diagrams.len());
  // there is a diagram id
  assert_eq!("_c3e08836-7973-4e4d-af2b-d46b23725c13_D1", dmndi.diagrams.get(0).unwrap().id.as_ref().unwrap());
  // there is a diagram name
  assert_eq!("Page 1", dmndi.diagrams.get(0).unwrap().name);
  // there is a height of a diagram
  assert!(650.0_f64.eq(&dmndi.diagrams.get(0).unwrap().size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(650.0_f64.eq(&dmndi.diagrams.get(0).unwrap().size.as_ref().unwrap().width));
  // there are a 3 diagram elements
  assert_eq!(5, dmndi.diagrams.get(0).unwrap().diagram_elements.len());

  // there is a first shape's id
  let shape_0 = get_shape(&dmndi.diagrams.get(0).unwrap().diagram_elements, 0).unwrap();
  assert_eq!("_c3e08836-7973-4e4d-af2b-d46b23725c13_s1", shape_0.id.as_ref().unwrap());
  // there is a first shape's dmn_element_ref
  assert_eq!("_9df2ca89-d100-4ba3-9a44-6a71cae5c001", shape_0.dmn_element_ref.as_ref().unwrap());
  // there is a first shape's bound - height
  assert!(60.0_f64.eq(&shape_0.bounds.height));
  // there is a first shape's bound - width
  assert!(153.0_f64.eq(&shape_0.bounds.width));
  // there is a first shape's bound - x
  assert!(151.0_f64.eq(&shape_0.bounds.x));
  // there is a first shape's bound - y
  assert!(331.0_f64.eq(&shape_0.bounds.y));
  // there is a sharedStyle of label
  assert_eq!(
    "LS_c3e08836-7973-4e4d-af2b-d46b23725c13_0",
    shape_0.label.as_ref().unwrap().shared_style.as_ref().unwrap()
  );

  // there is a second shape's id
  let shape_1 = get_shape(&dmndi.diagrams.get(0).unwrap().diagram_elements, 1).unwrap();
  assert_eq!("_c3e08836-7973-4e4d-af2b-d46b23725c13_s2", shape_1.id.as_ref().unwrap());
  // there is a second shape's dmn_element_ref
  assert_eq!("_2d131943-c513-416b-acc6-6efe8fe01ba4", shape_1.dmn_element_ref.as_ref().unwrap());
  // there is a second shape's bound - height
  assert!(61.0_f64.eq(&shape_1.bounds.height));
  // there is a second shape's bound - width
  assert!(154.0_f64.eq(&shape_1.bounds.width));
  // there is a second shape's bound - x
  assert!(150.0_f64.eq(&shape_1.bounds.x));
  // there is a second shape's bound - y
  assert!(150.0_f64.eq(&shape_1.bounds.y));
  // there is a sharedStyle of label
  assert_eq!(
    "LS_c3e08836-7973-4e4d-af2b-d46b23725c13_0",
    shape_1.label.as_ref().unwrap().shared_style.as_ref().unwrap()
  );

  // there is a third shape's id
  let shape_2 = get_shape(&dmndi.diagrams.get(0).unwrap().diagram_elements, 2).unwrap();
  assert_eq!("_42d5102d-9f7a-4ba7-9f11-e4371b8527e6", shape_2.id.as_ref().unwrap());
  // there is a third shape's dmn_element_ref
  assert_eq!("include1:_32543811-b499-4608-b784-6c6f294b1c58", shape_2.dmn_element_ref.as_ref().unwrap());
  // there is a third shape's bound - height
  assert!(59.0_f64.eq(&shape_2.bounds.height));
  // there is a third shape's bound - width
  assert!(152.0_f64.eq(&shape_2.bounds.width));
  // there is a third shape's bound - x
  assert!(394.0_f64.eq(&shape_2.bounds.x));
  // there is a third shape's bound - y
  assert!(151.0_f64.eq(&shape_2.bounds.y));
  // there is a sharedStyle of label
  assert_eq!(
    "LS_c3e08836-7973-4e4d-af2b-d46b23725c13_0",
    shape_2.label.as_ref().unwrap().shared_style.as_ref().unwrap()
  );
  // there is a label's bound - height
  let shape_2_label_bounds = shape_2.label.as_ref().unwrap().bounds.as_ref().unwrap();
  assert!(12.0_f64.eq(&shape_2_label_bounds.height));
  // there is a  label's bound - width
  assert!(94.0_f64.eq(&shape_2_label_bounds.width));
  // there is a label's bound - x
  assert!(422.0_f64.eq(&shape_2_label_bounds.x));
  // there is a label's bound - y
  assert!(174.0_f64.eq(&shape_2_label_bounds.y));

  // there is a first edge's id
  let edge_0 = get_edge(&dmndi.diagrams.get(0).unwrap().diagram_elements, 3).unwrap();
  assert_eq!("_c3e08836-7973-4e4d-af2b-d46b23725c13_e1", edge_0.id.as_ref().unwrap());
  // there is a first edge's dmn_element_ref
  assert_eq!("_01a9f8c0-6333-45cf-a693-e2e67b23fa13", edge_0.dmn_element_ref.as_ref().unwrap());
  // there is a first waypoint's x
  assert!(227.0_f64.eq(&edge_0.way_points.get(0).unwrap().x));
  // there is a first waypoint's y
  assert!(331.0_f64.eq(&edge_0.way_points.get(0).unwrap().y));
  // there is a second waypoint's x
  assert!(227.0_f64.eq(&edge_0.way_points.get(1).unwrap().x));
  // there is a second waypoint's y
  assert!(211.0_f64.eq(&edge_0.way_points.get(1).unwrap().y));
  // there is a first edge label id
  assert_eq!(
    "LS_c3e08836-7973-4e4d-af2b-d46b23725c13_0",
    edge_0.label.as_ref().unwrap().shared_style.as_ref().unwrap()
  );

  // there is a second edge's id
  let edge_1 = get_edge(&dmndi.diagrams.get(0).unwrap().diagram_elements, 4).unwrap();
  assert_eq!("_1fa3820f-3254-4000-88f4-c7dc79996907", edge_1.id.as_ref().unwrap());
  // there is a second edge's dmn_element_ref
  assert_eq!("_73d5099c-5a54-4a04-a9b5-80e1957ad8e9", edge_1.dmn_element_ref.as_ref().unwrap());
  // there is a first waypoint's x
  assert!(394.0_f64.eq(&edge_1.way_points.get(0).unwrap().x));
  // there is a first waypoint's y
  assert!(180.0_f64.eq(&edge_1.way_points.get(0).unwrap().y));
  // there is a second waypoint's x
  assert!(304.0_f64.eq(&edge_1.way_points.get(1).unwrap().x));
  // there is a second waypoint's y
  assert!(180.5_f64.eq(&edge_1.way_points.get(1).unwrap().y));
  // there is a second edge label id
  assert_eq!(
    "LS_c3e08836-7973-4e4d-af2b-d46b23725c13_0",
    edge_1.label.as_ref().unwrap().shared_style.as_ref().unwrap()
  );

  // there is a style id
  let style = dmndi.styles.get(0).unwrap();
  assert_eq!("LS_c3e08836-7973-4e4d-af2b-d46b23725c13_0", style.id.as_ref().unwrap());
  // there is a style fontFamily
  assert_eq!("arial,helvetica,sans-serif", style.font_family);
  // there is a style fontSize
  assert!(11.0_f64.eq(&style.font_size));
  // there is a style fontBold
  assert!(!style.font_bold);
  // there is a style fontItalic
  assert!(!style.font_italic);
  // there is a style fontUnderline
  assert!(!style.font_underline);
  // there is a style fontStrikeThrough
  assert!(!style.font_strike_through);
}

#[test]
fn _3_0087() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0087).unwrap();
  assert_eq!("_9d01a0c4-f529-4ad8-ad8e-ec5fb5d96ad4", definitions.id().as_ref().unwrap().as_str());
  let dmndi = definitions.dmndi.unwrap();
  // there are no shared styles defined
  assert_eq!(1, dmndi.styles.len());
  // there is a single diagram defined
  assert_eq!(6, dmndi.diagrams.len());

  // there is a first diagram
  let diagram_0 = dmndi.diagrams.get(0).unwrap();
  assert_eq!("_ce4a4c00-c3a3-46a6-8938-055239f6b326", diagram_0.id.as_ref().unwrap());
  // there is a diagram name
  assert_eq!("DRD of all automated decision-making", diagram_0.name);
  // there is a height of a diagram
  assert!(1050.9786834716797_f64.eq(&diagram_0.size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(1411.2411708831787_f64.eq(&diagram_0.size.as_ref().unwrap().width));
  // there are diagram elements
  assert_eq!(76, diagram_0.diagram_elements.len());

  // there is a second diagram
  let diagram_1 = dmndi.diagrams.get(1).unwrap();
  assert_eq!("_0e22b6cf-0a6e-40e1-a81e-44b31ad86262", diagram_1.id.as_ref().unwrap());
  // there is a diagram name
  assert_eq!("DRD for Decide bureau strategy decision point", diagram_1.name);
  // there is a height of a diagram
  assert!(967.0000038146973_f64.eq(&diagram_1.size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(979.8080854415894_f64.eq(&diagram_1.size.as_ref().unwrap().width));
  // there are diagram elements
  assert_eq!(48, diagram_1.diagram_elements.len());

  // there is a third diagram
  let diagram_2 = dmndi.diagrams.get(2).unwrap();
  assert_eq!("_3275163a-921d-48f8-967a-21c4373b1197", diagram_2.id.as_ref().unwrap());
  // there is a diagram name
  assert_eq!("DRD for Decide routing decision point", diagram_2.name);
  // there is a height of a diagram
  assert!(768.4786834716797_f64.eq(&diagram_2.size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(1140.5_f64.eq(&diagram_2.size.as_ref().unwrap().width));
  // there are diagram elements
  assert_eq!(40, diagram_2.diagram_elements.len());

  // there is a 4th diagram
  let diagram_3 = dmndi.diagrams.get(3).unwrap();
  assert_eq!("_a35ef6e9-0408-4288-b8f2-d28ac4baca3b", diagram_3.id.as_ref().unwrap());
  // there is a diagram name
  assert_eq!("DRD for Review application decision point", diagram_3.name);
  // there is a height of a diagram
  assert!(430.9786834716797_f64.eq(&diagram_3.size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(665.7411708831787_f64.eq(&diagram_3.size.as_ref().unwrap().width));
  // there are diagram elements
  assert_eq!(11, diagram_3.diagram_elements.len());

  // there is a 5th diagram
  let diagram_4 = dmndi.diagrams.get(4).unwrap();
  assert_eq!("_5c111794-4c6b-4747-8dfc-99d2ad0b6313", diagram_4.id.as_ref().unwrap());
  // there is a diagram name
  assert_eq!("Bureau Strategy Decision Service", diagram_4.name);
  // there is a height of a diagram
  assert!(893.0000038146973_f64.eq(&diagram_4.size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(743.8705854415894_f64.eq(&diagram_4.size.as_ref().unwrap().width));
  // there are diagram elements
  assert_eq!(23, diagram_4.diagram_elements.len());
  // there is a third shape
  let shape_2 = get_shape(&diagram_4.diagram_elements, 2).unwrap();
  assert_eq!("_392bc431-eb67-4350-9671-2a0677db09f4", shape_2.id.as_ref().unwrap());
  // there is a third shape's dmn_element_ref
  assert_eq!("_7befd964-eefa-4d8f-908d-8f6ad8d22c67", shape_2.dmn_element_ref.as_ref().unwrap());
  // there is a third shape's isCollapsed
  assert!(!shape_2.is_collapsed);
  // there is a third shape's bound - height
  assert!(670.0_f64.eq(&shape_2.bounds.height));
  // there is a third shape's bound - width
  assert!(643.8705854415894_f64.eq(&shape_2.bounds.width));
  // there is a third shape's bound - x
  assert!(50.0_f64.eq(&shape_2.bounds.x));
  // there is a third shape's bound - y
  assert!(50.0_f64.eq(&shape_2.bounds.y));
  // there is a sharedStyle of label
  assert_eq!(
    "LS_9d01a0c4-f529-4ad8-ad8e-ec5fb5d96ad4_0",
    shape_2.label.as_ref().unwrap().shared_style.as_ref().unwrap()
  );
  // there is a DecisionServiceDividerLine
  let shape_2_divider_line = shape_2.decision_service_divider_line.as_ref().unwrap();
  // there is a first waypoint's x
  assert!(50.0_f64.eq(&shape_2_divider_line.way_points.get(0).unwrap().x));
  // there is a first waypoint's y
  assert!(275.0_f64.eq(&shape_2_divider_line.way_points.get(0).unwrap().y));
  // there is a second waypoint's x
  assert!(693.8705854415894_f64.eq(&shape_2_divider_line.way_points.get(1).unwrap().x));
  // there is a second waypoint's y
  assert!(275.0_f64.eq(&shape_2_divider_line.way_points.get(1).unwrap().y));

  // there is a 6th diagram
  let diagram_5 = dmndi.diagrams.get(5).unwrap();
  assert_eq!("_69750f88-f46f-4b47-bb3c-fb77f574f2b3", diagram_5.id.as_ref().unwrap());
  // there is a diagram name
  assert_eq!("Routing Decision Service", diagram_5.name);
  // there is a height of a diagram
  assert!(789.4573631286621_f64.eq(&diagram_5.size.as_ref().unwrap().height));
  // there is a width of a diagram
  assert!(793.0_f64.eq(&diagram_5.size.as_ref().unwrap().width));
  // there are diagram elements
  assert_eq!(20, diagram_5.diagram_elements.len());
  // there is a third shape
  let shape_3 = get_shape(&diagram_5.diagram_elements, 3).unwrap();
  assert_eq!("_448a6614-664f-4aa1-b68f-b8e5286db1b3", shape_3.id.as_ref().unwrap());
  // there is a third shape's dmn_element_ref
  assert_eq!("_4d91e3a5-acec-4254-81e4-8535a1d336ee", shape_3.dmn_element_ref.as_ref().unwrap());
  // there is a third shape's isCollapsed
  assert!(!shape_3.is_collapsed);
  // there is a third shape's bound - height
  assert!(562.4786796569824_f64.eq(&shape_3.bounds.height));
  // there is a third shape's bound - width
  assert!(693.0_f64.eq(&shape_3.bounds.width));
  // there is a third shape's bound - x
  assert!(50.0_f64.eq(&shape_3.bounds.x));
  // there is a third shape's bound - y
  assert!(50.0_f64.eq(&shape_3.bounds.y));
  // there is a sharedStyle of label
  assert_eq!(
    "LS_9d01a0c4-f529-4ad8-ad8e-ec5fb5d96ad4_0",
    shape_3.label.as_ref().unwrap().shared_style.as_ref().unwrap()
  );
  // there is a DecisionServiceDividerLine
  let shape_2_divider_line = shape_3.decision_service_divider_line.as_ref().unwrap();
  // there is a first waypoint's x
  assert!(50.0_f64.eq(&shape_2_divider_line.way_points.get(0).unwrap().x));
  // there is a first waypoint's y
  assert!(190.0_f64.eq(&shape_2_divider_line.way_points.get(0).unwrap().y));
  // there is a second waypoint's x
  assert!(743.0_f64.eq(&shape_2_divider_line.way_points.get(1).unwrap().x));
  // there is a second waypoint's y
  assert!(190.0_f64.eq(&shape_2_divider_line.way_points.get(1).unwrap().y));

  // there is a style id
  let style = dmndi.styles.get(0).unwrap();
  assert_eq!("LS_9d01a0c4-f529-4ad8-ad8e-ec5fb5d96ad4_0", style.id.as_ref().unwrap());
  // there is a style fontFamily
  assert_eq!("arial,helvetica,sans-serif", style.font_family);
  // there is a style fontSize
  assert!(14.0_f64.eq(&style.font_size));
  // there is a style fontBold
  assert!(!style.font_bold);
  // there is a style fontItalic
  assert!(!style.font_italic);
  // there is a style fontUnderline
  assert!(!style.font_underline);
  // there is a style fontStrikeThrough
  assert!(!style.font_strike_through);
}
