/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * MIT license
 *
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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
 * Copyright (c) 2018-2021 Dariusz Depta Engos Software
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

//! Parser for loading a model from the XML file containing DMN interchange format.

use self::errors::*;
use self::xml_utils::*;
use super::*;
use dmntk_common::Result;
use roxmltree::Node;

const NODE_ALLOWED_ANSWERS: &str = "allowedAnswers";
const NODE_ALLOWED_VALUES: &str = "allowedValues";
const NODE_BINDING: &str = "binding";
const NODE_BUSINESS_KNOWLEDGE_MODEL: &str = "businessKnowledgeModel";
const NODE_COLUMN: &str = "column";
const NODE_CONTEXT: &str = "context";
const NODE_CONTEXT_ENTRY: &str = "contextEntry";
const NODE_DEFAULT_OUTPUT_ENTRY: &str = "defaultOutputEntry";
const NODE_DEFINITIONS: &str = "definitions";
const NODE_DECISION: &str = "decision";
const NODE_DECISION_TABLE: &str = "decisionTable";
const NODE_DECISION_SERVICE: &str = "decisionService";
const NODE_DMNDI: &str = "DMNDI";
const NODE_DMNDI_DMN_DIAGRAM: &str = "DMNDiagram";
const NODE_DMNDI_SIZE: &str = "Size";
const NODE_DMNDI_STYLE: &str = "DMNStyle";
// const NODE_DMNDI_SHARED_STYLE: &str = "sharedStyle";
const NODE_DMNDI_LOCAL_STYLE: &str = "localStyle";
const NODE_DMNDI_DMN_SHAPE: &str = "DMNShape";
const NODE_DMNDI_BOUNDS: &str = "Bounds";
const NODE_DMNDI_DMN_EDGE: &str = "DMNEdge";
const NODE_DMNDI_WAYPOINT: &str = "waypoint";
const NODE_DMNDI_FILL_COLOR: &str = "fillColor";
const NODE_DMNDI_STROKE_COLOR: &str = "strokeColor";
const NODE_DMNDI_FONT_COLOR: &str = "fontColor";
const NODE_DMNDI_LABEL_HORIZONTAL_ALIGNMENT: &str = "labelHorizontalAlignment";
const NODE_DMNDI_LABEL_VERTICAL_ALIGNMENT: &str = "labelVerticalAlignment";
const NODE_DMNDI_LABEL: &str = "DMNLabel";
const NODE_DMNDI_DECISION_SERVICE_DIVIDER_LINE: &str = "DMNDecisionServiceDividerLine";
const NODE_DESCRIPTION: &str = "description";
const NODE_ENCAPSULATED_DECISION: &str = "encapsulatedDecision";
const NODE_ENCAPSULATED_LOGIC: &str = "encapsulatedLogic";
const NODE_FUNCTION_DEFINITION: &str = "functionDefinition";
const NODE_FORMAL_PARAMETER: &str = "formalParameter";
const NODE_FUNCTION_ITEM: &str = "functionItem";
const NODE_INFORMATION_REQUIREMENT: &str = "informationRequirement";
const NODE_INPUT_DATA: &str = "inputData";
const NODE_INPUT: &str = "input";
const NODE_INPUT_DECISION: &str = "inputDecision";
const NODE_INPUT_ENTRY: &str = "inputEntry";
const NODE_INPUT_EXPRESSION: &str = "inputExpression";
const NODE_INPUT_VALUES: &str = "inputValues";
const NODE_INVOCATION: &str = "invocation";
const NODE_ITEM_DEFINITION: &str = "itemDefinition";
const NODE_ITEM_COMPONENT: &str = "itemComponent";
const NODE_KNOWLEDGE_REQUIREMENT: &str = "knowledgeRequirement";
const NODE_KNOWLEDGE_SOURCE: &str = "knowledgeSource";
const NODE_LITERAL_EXPRESSION: &str = "literalExpression";
const NODE_OUTPUT: &str = "output";
const NODE_OUTPUT_DECISION: &str = "outputDecision";
const NODE_OUTPUT_ENTRY: &str = "outputEntry";
const NODE_OUTPUT_VALUES: &str = "outputValues";
const NODE_PARAMETER: &str = "parameter";
const NODE_QUESTION: &str = "question";
const NODE_RELATION: &str = "relation";
const NODE_REQUIRED_DECISION: &str = "requiredDecision";
const NODE_REQUIRED_KNOWLEDGE: &str = "requiredKnowledge";
const NODE_REQUIRED_INPUT: &str = "requiredInput";
const NODE_ROW: &str = "row";
const NODE_RULE: &str = "rule";
const NODE_TEXT: &str = "text";
const NODE_TYPE_REF: &str = "typeRef";
const NODE_VARIABLE: &str = "variable";

const ATTR_BLUE: &str = "blue";
const ATTR_DMN_ELEMENT_REF: &str = "dmnElementRef";
const ATTR_EXPORTER: &str = "exporter";
const ATTR_EXPORTER_VERSION: &str = "exporter_version";
const ATTR_EXPRESSION_LANGUAGE: &str = "expressionLanguage";
const ATTR_FONT_BOLD: &str = "fontBold";
const ATTR_FONT_FAMILY: &str = "fontFamily";
const ATTR_FONT_ITALIC: &str = "fontItalic";
const ATTR_FONT_SIZE: &str = "fontSize";
const ATTR_FONT_STRIKE_THROUGH: &str = "fontStrikeThrough";
const ATTR_FONT_UNDERLINE: &str = "fontUnderline";
const ATTR_GREEN: &str = "green";
const ATTR_HIT_POLICY: &str = "hitPolicy";
const ATTR_AGGREGATION: &str = "aggregation";
const ATTR_PREFERRED_ORIENTATION: &str = "preferredOrientation";
const ATTR_HEIGHT: &str = "height";
const ATTR_HREF: &str = "href";
const ATTR_ID: &str = "id";
const ATTR_IS_COLLAPSED: &str = "isCollapsed";
const ATTR_IS_COLLECTION: &str = "isCollection";
const ATTR_KIND: &str = "kind";
const ATTR_LABEL: &str = "label";
const ATTR_NAME: &str = "name";
const ATTR_NAMESPACE: &str = "namespace";
const ATTR_OUTPUT_LABEL: &str = "outputLabel";
const ATTR_OUTPUT_TYPE_REF: &str = "outputTypeRef";
const ATTR_RED: &str = "red";
const ATTR_RESOLUTION: &str = "resolution";
const ATTR_SHARED_STYLE: &str = "sharedStyle";
const ATTR_TEXT: &str = "text";
const ATTR_TYPE_LANGUAGE: &str = "typeLanguage";
const ATTR_TYPE_REF: &str = "typeRef";
const ATTR_WIDTH: &str = "width";
const ATTR_X: &str = "x";
const ATTR_Y: &str = "y";

#[derive(Default)]
pub struct ModelParser {}

impl ModelParser {
  /// Parses the XML document containing definitions.
  pub fn parse(&mut self, xml: &str, source: &str) -> Result<Definitions> {
    match roxmltree::Document::parse(xml) {
      Ok(document) => {
        let definitions_node = document.root_element();
        if definitions_node.tag_name().name() != NODE_DEFINITIONS {
          return Err(xml_unexpected_node(NODE_DEFINITIONS, definitions_node.tag_name().name()));
        }
        self.parse_definitions(&definitions_node, source)
      }
      Err(reason) => Err(xml_parsing_model_failed(&reason.to_string())),
    }
  }

  /// Parses model definitions.
  ///
  /// # Arguments
  ///
  /// - mode   - definitions node.
  /// - source - name of the model source file.
  ///
  fn parse_definitions(&mut self, node: &Node, source: &str) -> Result<Definitions> {
    let mut definitions = Definitions {
      name: required_name(node)?,
      feel_name: optional_feel_name(node)?,
      id: optional_attribute(node, ATTR_ID),
      description: optional_child_optional_content(node, NODE_DESCRIPTION),
      label: optional_attribute(node, ATTR_LABEL),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      namespace: required_attribute(node, ATTR_NAMESPACE)?,
      expression_language: optional_attribute(node, ATTR_EXPRESSION_LANGUAGE),
      type_language: optional_attribute(node, ATTR_TYPE_LANGUAGE),
      exporter: optional_attribute(node, ATTR_EXPORTER),
      exporter_version: optional_attribute(node, ATTR_EXPORTER_VERSION),
      item_definitions: self.parse_item_definitions(node, NODE_ITEM_DEFINITION)?,
      drg_elements: self.parse_drg_elements(node)?,
      business_context_elements: self.parse_business_context_elements(node)?,
      source: source.to_owned(),
      dmndi: None, // DMNDI (if present) is parsed in next step below //FIXME maybe this could be done here?
    };
    self.parse_dmndi(node, &mut definitions)?;
    Ok(definitions)
  }

  fn parse_item_definitions(&mut self, node: &Node, child_name: &str) -> Result<Vec<ItemDefinition>> {
    let mut items = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == child_name) {
      let type_ref = optional_child_required_content(child_node, NODE_TYPE_REF)?;
      let type_language = optional_attribute(child_node, ATTR_TYPE_LANGUAGE);
      let allowed_values = self.parse_unary_tests(child_node, NODE_ALLOWED_VALUES)?;
      let item_components_definitions = self.parse_item_definitions(child_node, NODE_ITEM_COMPONENT)?;
      let item_definition = ItemDefinition {
        name: required_name(child_node)?,
        feel_name: optional_feel_name(child_node)?,
        id: optional_attribute(child_node, ATTR_ID),
        description: optional_child_optional_content(child_node, NODE_DESCRIPTION),
        label: optional_attribute(child_node, ATTR_LABEL),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        type_ref,
        type_language,
        feel_type: None,
        allowed_values,
        item_components: item_components_definitions,
        is_collection: self.parse_boolean_attribute(child_node, ATTR_IS_COLLECTION, false),
        function_item: self.parse_function_item(child_node),
        item_definition_type: None,
      };
      items.push(item_definition);
    }
    Ok(items)
  }

  ///
  fn parse_function_item(&self, node: &Node) -> Option<FunctionItem> {
    node
      .children()
      .find(|n| n.tag_name().name() == NODE_FUNCTION_ITEM)
      .as_ref()
      .map(|n| FunctionItem {
        output_type_ref: optional_attribute(n, ATTR_OUTPUT_TYPE_REF),
        parameters: vec![],
      })
  }

  fn parse_unary_tests(&self, node: &Node, child_name: &str) -> Result<Option<UnaryTests>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      Ok(Some(UnaryTests {
        text: optional_child_required_content(&child_node, NODE_TEXT)?,
        expression_language: optional_attribute(&child_node, ATTR_EXPRESSION_LANGUAGE),
      }))
    } else {
      Ok(None)
    }
  }

  fn parse_drg_elements(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    drg_elements.append(&mut self.parse_input_data(node)?);
    drg_elements.append(&mut self.parse_decisions(node)?);
    drg_elements.append(&mut self.parse_business_knowledge_models(node)?);
    drg_elements.append(&mut self.parse_decision_services(node)?);
    drg_elements.append(&mut self.parse_knowledge_sources(node)?);
    Ok(drg_elements)
  }

  fn parse_input_data(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut input_data_items = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_INPUT_DATA) {
      let input_data = InputData {
        id: optional_attribute(child_node, ATTR_ID),
        description: optional_child_optional_content(child_node, NODE_DESCRIPTION),
        label: optional_attribute(child_node, ATTR_LABEL),
        extension_elements: self.parse_extension_elements(child_node),
        extension_attributes: self.parse_extension_attributes(child_node),
        name: required_name(child_node)?,
        feel_name: optional_feel_name(node)?,
        variable: self.parse_information_item_child(child_node, NODE_VARIABLE)?,
      };
      input_data_items.push(DrgElement::InputData(input_data));
    }
    Ok(input_data_items)
  }

  fn parse_decisions(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut decision_items = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_DECISION) {
      let decision = Decision {
        name: required_name(child_node)?,
        feel_name: optional_feel_name(child_node)?,
        id: optional_attribute(child_node, ATTR_ID),
        description: optional_child_optional_content(child_node, NODE_DESCRIPTION),
        label: optional_attribute(child_node, ATTR_LABEL),
        extension_elements: None,
        extension_attributes: vec![],
        question: optional_child_optional_content(child_node, NODE_QUESTION),
        allowed_answers: optional_child_optional_content(child_node, NODE_ALLOWED_ANSWERS),
        variable: self.parse_information_item_child(child_node, NODE_VARIABLE)?,
        decision_logic: self.parse_optional_expression_instance(child_node)?,
        information_requirements: self.parse_information_requirements(child_node, NODE_INFORMATION_REQUIREMENT)?,
        knowledge_requirements: self.parse_knowledge_requirements(child_node, NODE_KNOWLEDGE_REQUIREMENT)?,
      };
      decision_items.push(DrgElement::Decision(decision));
    }
    Ok(decision_items)
  }

  fn parse_business_knowledge_models(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut parsed_items = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_BUSINESS_KNOWLEDGE_MODEL) {
      let business_knowledge_model = BusinessKnowledgeModel {
        name: required_name(child_node)?,
        feel_name: optional_feel_name(child_node)?,
        id: optional_attribute(child_node, ATTR_ID),
        description: optional_child_optional_content(child_node, NODE_DESCRIPTION),
        label: optional_attribute(child_node, ATTR_LABEL),
        extension_elements: None,
        extension_attributes: vec![],
        variable: self.parse_information_item_child(child_node, NODE_VARIABLE)?,
        encapsulated_logic: self.parse_function_definition_child(child_node, NODE_ENCAPSULATED_LOGIC)?,
        knowledge_requirements: self.parse_knowledge_requirements(child_node, NODE_KNOWLEDGE_REQUIREMENT)?,
        authority_requirements: vec![],
      };
      parsed_items.push(DrgElement::BusinessKnowledgeModel(business_knowledge_model));
    }
    Ok(parsed_items)
  }

  fn parse_decision_services(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_DECISION_SERVICE) {
      let decision_service = DecisionService {
        name: required_name(child_node)?,
        feel_name: optional_feel_name(child_node)?,
        id: optional_attribute(child_node, ATTR_ID),
        description: optional_child_optional_content(child_node, NODE_DESCRIPTION),
        label: optional_attribute(child_node, ATTR_LABEL),
        extension_elements: None,
        extension_attributes: vec![],
        variable: self.parse_information_item_child(child_node, NODE_VARIABLE)?,
        output_decisions: self.required_hrefs_in_child_nodes(child_node, NODE_OUTPUT_DECISION)?,
        encapsulated_decisions: self.required_hrefs_in_child_nodes(child_node, NODE_ENCAPSULATED_DECISION)?,
        input_decisions: self.required_hrefs_in_child_nodes(child_node, NODE_INPUT_DECISION)?,
        input_data: self.required_hrefs_in_child_nodes(child_node, NODE_INPUT_DATA)?,
      };
      drg_elements.push(DrgElement::DecisionService(decision_service));
    }
    Ok(drg_elements)
  }

  fn parse_knowledge_sources(&self, node: &Node) -> Result<Vec<DrgElement>> {
    let mut drg_elements = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_KNOWLEDGE_SOURCE) {
      let knowledge_source = KnowledgeSource {
        id: optional_attribute(child_node, ATTR_ID),
        description: optional_child_optional_content(child_node, NODE_DESCRIPTION),
        label: optional_attribute(child_node, ATTR_LABEL),
        extension_elements: None,
        extension_attributes: vec![],
        name: required_name(child_node)?,
        feel_name: optional_feel_name(child_node)?,
      };
      drg_elements.push(DrgElement::KnowledgeSource(knowledge_source));
    }
    Ok(drg_elements)
  }

  fn required_hrefs_in_child_nodes(&self, node: &Node, child_name: &str) -> Result<Vec<HRef>> {
    let mut hrefs = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == child_name) {
      let text = required_attribute(child_node, ATTR_HREF)?;
      let href = HRef::try_from(text.as_str())?;
      hrefs.push(href);
    }
    Ok(hrefs)
  }

  fn parse_function_definition_child(&self, node: &Node, child_name: &str) -> Result<Option<FunctionDefinition>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      Ok(Some(self.parse_function_definition(&child_node)?))
    } else {
      Ok(None)
    }
  }

  fn parse_optional_function_definition(&self, node: &Node) -> Result<Option<FunctionDefinition>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == NODE_FUNCTION_DEFINITION) {
      Ok(Some(self.parse_function_definition(&child_node)?))
    } else {
      Ok(None)
    }
  }

  fn parse_function_definition(&self, node: &Node) -> Result<FunctionDefinition> {
    Ok(FunctionDefinition {
      id: optional_attribute(node, ATTR_ID),
      description: optional_child_optional_content(node, NODE_DESCRIPTION),
      label: optional_attribute(node, ATTR_LABEL),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      formal_parameters: self.parse_information_items_child(node, NODE_FORMAL_PARAMETER)?,
      body: self.parse_optional_expression_instance(node)?,
      kind: self.parse_function_kind(node)?,
    })
  }

  fn parse_function_kind(&self, node: &Node) -> Result<FunctionKind> {
    if let Some(function_kind_text) = optional_attribute(node, ATTR_KIND) {
      match function_kind_text.trim() {
        "FEEL" => Ok(FunctionKind::Feel),
        "Java" => Ok(FunctionKind::Java),
        "PMML" => Ok(FunctionKind::Pmml),
        other => Err(invalid_function_kind(other)),
      }
    } else {
      Ok(FunctionKind::Feel)
    }
  }

  #[allow(clippy::unnecessary_wraps)]
  fn parse_business_context_elements(&self, _node: &Node) -> Result<Vec<BusinessContextElementInstance>> {
    Ok(vec![])
  }

  fn parse_information_item_child(&self, node: &Node, child_name: &str) -> Result<InformationItem> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      self.parse_information_item(&child_node)
    } else {
      Err(xml_expected_mandatory_child_node(&node_name_pos(node), child_name))
    }
  }

  fn parse_optional_information_item_child(&self, node: &Node, child_name: &str) -> Result<Option<InformationItem>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      return Ok(Some(self.parse_information_item(&child_node)?));
    }
    Ok(None)
  }

  fn parse_information_items_child(&self, node: &Node, child_name: &str) -> Result<Vec<InformationItem>> {
    let mut information_items = vec![];
    for child_node in node.children().filter(|n| n.tag_name().name() == child_name) {
      information_items.push(self.parse_information_item(&child_node)?);
    }
    Ok(information_items)
  }

  fn parse_information_item(&self, node: &Node) -> Result<InformationItem> {
    Ok(InformationItem {
      id: optional_attribute(node, ATTR_ID),
      description: optional_child_optional_content(node, NODE_DESCRIPTION),
      label: optional_attribute(node, ATTR_LABEL),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      name: required_name(node)?,
      feel_name: optional_feel_name(node)?,
      value_expression: self.parse_optional_expression_instance(node)?,
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      feel_type: None,
    })
  }

  fn parse_information_requirements(&self, node: &Node, child_name: &str) -> Result<Vec<InformationRequirement>> {
    let mut information_requirement_items = vec![];
    for child_node in node.children().filter(|n| n.tag_name().name() == child_name) {
      information_requirement_items.push(self.parse_information_requirement(&child_node)?);
    }
    Ok(information_requirement_items)
  }

  fn parse_information_requirement(&self, node: &Node) -> Result<InformationRequirement> {
    Ok(InformationRequirement {
      id: optional_attribute(node, ATTR_ID),
      description: optional_child_optional_content(node, NODE_DESCRIPTION),
      label: optional_attribute(node, ATTR_LABEL),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      required_decision: optional_child_required_href(node, NODE_REQUIRED_DECISION)?,
      required_input: optional_child_required_href(node, NODE_REQUIRED_INPUT)?,
    })
  }

  fn parse_knowledge_requirements(&self, node: &Node, child_name: &str) -> Result<Vec<KnowledgeRequirement>> {
    let mut knowledge_requirement_items = vec![];
    for child_node in node.children().filter(|n| n.tag_name().name() == child_name) {
      knowledge_requirement_items.push(self.parse_knowledge_requirement(&child_node)?);
    }
    Ok(knowledge_requirement_items)
  }

  fn parse_knowledge_requirement(&self, node: &Node) -> Result<KnowledgeRequirement> {
    Ok(KnowledgeRequirement {
      id: optional_attribute(node, ATTR_ID),
      description: optional_child_optional_content(node, NODE_DESCRIPTION),
      label: optional_attribute(node, ATTR_LABEL),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      required_knowledge: optional_child_required_href(node, NODE_REQUIRED_KNOWLEDGE)?,
    })
  }

  fn parse_required_expression_instance(&self, node: &Node) -> Result<ExpressionInstance> {
    self
      .parse_optional_expression_instance(node)?
      .ok_or_else(required_expression_instance_is_missing)
  }

  fn parse_optional_expression_instance(&self, node: &Node) -> Result<Option<ExpressionInstance>> {
    if let Some(context) = self.parse_optional_context(node)? {
      return Ok(Some(ExpressionInstance::Context(context)));
    }
    if let Some(decision_table) = self.parse_decision_table(node)? {
      return Ok(Some(ExpressionInstance::DecisionTable(decision_table)));
    }
    if let Some(function_definition) = self.parse_optional_function_definition(node)? {
      return Ok(Some(ExpressionInstance::FunctionDefinition(Box::new(function_definition))));
    }
    if let Some(invocation) = self.parse_optional_invocation(node)? {
      return Ok(Some(ExpressionInstance::Invocation(Box::new(invocation))));
    }
    if let Some(literal_expression) = self.parse_optional_literal_expression(node) {
      return Ok(Some(ExpressionInstance::LiteralExpression(literal_expression)));
    }
    if let Some(relation) = self.parse_optional_relation(node)? {
      return Ok(Some(ExpressionInstance::Relation(relation)));
    }
    Ok(None)
  }

  fn parse_decision_table(&self, node: &Node) -> Result<Option<DecisionTable>> {
    if let Some(ref child_node) = node.children().find(|n| n.tag_name().name() == NODE_DECISION_TABLE) {
      return Ok(Some(DecisionTable {
        information_item_name: None,
        input_clauses: self.parse_decision_table_inputs(child_node)?,
        output_clauses: self.parse_decision_table_outputs(child_node)?,
        annotations: vec![],
        rules: self.parse_decision_table_rules(child_node)?,
        hit_policy: self.parse_hit_policy_attribute(child_node)?,
        aggregation: None,
        preferred_orientation: self.parse_preferred_orientation_attribute(child_node)?,
        output_label: optional_attribute(child_node, ATTR_OUTPUT_LABEL),
      }));
    }
    Ok(None)
  }

  fn parse_decision_table_inputs(&self, node: &Node) -> Result<Vec<InputClause>> {
    let mut input_clauses = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_INPUT) {
      input_clauses.push(self.parse_decision_table_input(child_node)?);
    }
    Ok(input_clauses)
  }

  fn parse_decision_table_input(&self, node: &Node) -> Result<InputClause> {
    let input_expression = if let Ok(ref child_node) = required_child(node, NODE_INPUT_EXPRESSION) {
      required_child_required_content(child_node, NODE_TEXT)?
    } else {
      return Err(required_input_expression_is_missing());
    };
    let input_values = if let Some(ref child_node) = optional_child(node, NODE_INPUT_VALUES) {
      optional_child_required_content(child_node, NODE_TEXT)?
    } else {
      None
    };
    Ok(InputClause {
      input_expression,
      input_values,
    })
  }

  fn parse_decision_table_outputs(&self, node: &Node) -> Result<Vec<OutputClause>> {
    let mut output_clauses = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_OUTPUT) {
      output_clauses.push(self.parse_decision_table_output(child_node)?);
    }
    Ok(output_clauses)
  }

  fn parse_decision_table_output(&self, node: &Node) -> Result<OutputClause> {
    let output_values = if let Some(ref child_node) = optional_child(node, NODE_OUTPUT_VALUES) {
      optional_child_required_content(child_node, NODE_TEXT)?
    } else {
      None
    };
    let default_output_entry = if let Some(ref child_node) = optional_child(node, NODE_DEFAULT_OUTPUT_ENTRY) {
      optional_child_required_content(child_node, NODE_TEXT)?
    } else {
      None
    };
    Ok(OutputClause {
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      name: optional_attribute(node, ATTR_NAME),
      output_values,
      default_output_entry,
    })
  }

  fn parse_decision_table_rules(&self, node: &Node) -> Result<Vec<DecisionRule>> {
    let mut rules = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_RULE) {
      rules.push(self.parse_decision_table_rule(child_node)?);
    }
    Ok(rules)
  }

  fn parse_decision_table_rule(&self, node: &Node) -> Result<DecisionRule> {
    Ok(DecisionRule {
      input_entries: self.parse_decision_table_input_entries(node)?,
      output_entries: self.parse_decision_table_output_entries(node)?,
      annotation_entries: vec![],
    })
  }

  fn parse_decision_table_input_entries(&self, node: &Node) -> Result<Vec<InputEntry>> {
    let mut input_entries = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_INPUT_ENTRY) {
      input_entries.push(self.parse_decision_table_input_entry(child_node)?);
    }
    Ok(input_entries)
  }

  fn parse_decision_table_input_entry(&self, node: &Node) -> Result<InputEntry> {
    Ok(InputEntry {
      text: required_child_required_content(node, NODE_TEXT)?,
    })
  }

  fn parse_decision_table_output_entries(&self, node: &Node) -> Result<Vec<OutputEntry>> {
    let mut output_entries = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_OUTPUT_ENTRY) {
      output_entries.push(self.parse_decision_table_output_entry(child_node)?);
    }
    Ok(output_entries)
  }

  fn parse_decision_table_output_entry(&self, node: &Node) -> Result<OutputEntry> {
    Ok(OutputEntry {
      text: required_child_required_content(node, NODE_TEXT)?,
    })
  }

  fn parse_optional_context(&self, node: &Node) -> Result<Option<Context>> {
    if let Some(ref child_node) = node.children().find(|n| n.tag_name().name() == NODE_CONTEXT) {
      return Ok(Some(Context {
        context_entries: self.parse_context_entries(child_node)?,
      }));
    }
    Ok(None)
  }

  fn parse_context_entries(&self, node: &Node) -> Result<Vec<ContextEntry>> {
    let mut context_entries = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_CONTEXT_ENTRY) {
      context_entries.push(ContextEntry {
        variable: self.parse_optional_information_item_child(child_node, NODE_VARIABLE)?,
        value: self.parse_required_expression_instance(child_node)?,
      });
    }
    Ok(context_entries)
  }

  fn parse_optional_invocation(&self, node: &Node) -> Result<Option<Invocation>> {
    if let Some(ref child_node) = node.children().find(|n| n.tag_name().name() == NODE_INVOCATION) {
      return Ok(Some(Invocation {
        called_function: self.parse_required_expression_instance(child_node)?,
        bindings: self.parse_bindings(child_node)?,
      }));
    }
    Ok(None)
  }

  fn parse_bindings(&self, node: &Node) -> Result<Vec<Binding>> {
    let mut bindings = vec![];
    for ref child_node in node.children().filter(|n| n.tag_name().name() == NODE_BINDING) {
      bindings.push(Binding {
        parameter: self.parse_information_item_child(child_node, NODE_PARAMETER)?,
        binding_formula: self.parse_optional_expression_instance(child_node)?,
      });
    }
    Ok(bindings)
  }

  /// Searches for the first node named 'literalExpression' among children of the specified `node`.
  /// When such node is found, then parses literal expression and returns it; otherwise returns [None].
  fn parse_optional_literal_expression(&self, node: &Node) -> Option<LiteralExpression> {
    if let Some(ref child_node) = node.children().find(|n| n.tag_name().name() == NODE_LITERAL_EXPRESSION) {
      return Some(self.parse_literal_expression(child_node));
    }
    None
  }

  /// Parses [LiteralExpression] directly from the specified node.
  /// The `literal_expression_node` must be a node named `literalExpression`.
  fn parse_literal_expression(&self, node: &Node) -> LiteralExpression {
    LiteralExpression {
      id: optional_attribute(node, ATTR_ID),
      description: optional_child_optional_content(node, NODE_DESCRIPTION),
      label: optional_attribute(node, ATTR_LABEL),
      extension_elements: self.parse_extension_elements(node),
      extension_attributes: self.parse_extension_attributes(node),
      type_ref: optional_attribute(node, ATTR_TYPE_REF),
      text: optional_child_optional_content(node, NODE_TEXT),
      expression_language: optional_attribute(node, ATTR_EXPRESSION_LANGUAGE),
      imported_values: None,
    }
  }

  fn parse_optional_relation(&self, node: &Node) -> Result<Option<Relation>> {
    if let Some(ref relation_node) = node.children().find(|n| n.tag_name().name() == NODE_RELATION) {
      let mut columns = vec![];
      for ref column_node in relation_node.children().filter(|n| n.tag_name().name() == NODE_COLUMN) {
        columns.push(self.parse_information_item(column_node)?);
      }
      let mut rows = vec![];
      for ref row_node in relation_node.children().filter(|n| n.tag_name().name() == NODE_ROW) {
        let mut elements = vec![];
        for ref expression_instance_node in row_node.children() {
          if expression_instance_node.tag_name().name() == NODE_LITERAL_EXPRESSION {
            let literal_expression = self.parse_literal_expression(expression_instance_node);
            elements.push(ExpressionInstance::LiteralExpression(literal_expression));
          }
        }
        if elements.len() != columns.len() {
          return Err(number_of_elements_in_row_differs_from_number_of_columns());
        }
        rows.push(List {
          id: optional_attribute(row_node, ATTR_ID),
          description: optional_child_optional_content(row_node, NODE_DESCRIPTION),
          label: optional_attribute(row_node, ATTR_LABEL),
          extension_elements: self.parse_extension_elements(row_node),
          extension_attributes: self.parse_extension_attributes(row_node),
          type_ref: optional_attribute(row_node, ATTR_TYPE_REF),
          elements,
        });
      }
      return Ok(Some(Relation {
        id: optional_attribute(relation_node, ATTR_ID),
        description: optional_child_optional_content(relation_node, NODE_DESCRIPTION),
        label: optional_attribute(relation_node, ATTR_LABEL),
        extension_elements: self.parse_extension_elements(relation_node),
        extension_attributes: self.parse_extension_attributes(relation_node),
        type_ref: optional_attribute(relation_node, ATTR_TYPE_REF),
        rows,
        columns,
      }));
    }
    Ok(None)
  }

  /// Parses extension elements.
  /// Currently extension elements are ignored and [None] is always returned.
  /// This function is a placeholder for further development.   
  fn parse_extension_elements(&self, _: &Node) -> Option<ExtensionElement> {
    None
  }

  /// Parses extension attributes.
  /// Currently extension elements are omitted and [None] is always returned.
  /// This function is a placeholder for further development.   
  fn parse_extension_attributes(&self, _: &Node) -> Vec<ExtensionAttribute> {
    vec![]
  }

  /// Returns boolean value of the specified attribute.
  fn parse_boolean_attribute(&self, node: &Node, attr_name: &str, default_value: bool) -> bool {
    if let Some(attr_value) = node.attribute(attr_name) {
      attr_value == "true"
    } else {
      default_value
    }
  }

  /// Returns the value of the hit policy attribute.
  fn parse_hit_policy_attribute(&self, node: &Node) -> Result<HitPolicy> {
    if let Some(hit_policy_text) = node.attribute(ATTR_HIT_POLICY) {
      match hit_policy_text.trim() {
        "UNIQUE" => Ok(HitPolicy::Unique),
        "ANY" => Ok(HitPolicy::Any),
        "PRIORITY" => Ok(HitPolicy::Priority),
        "FIRST" => Ok(HitPolicy::First),
        "RULE ORDER" => Ok(HitPolicy::RuleOrder),
        "OUTPUT ORDER" => Ok(HitPolicy::OutputOrder),
        "COLLECT" => Ok(HitPolicy::Collect(self.parse_aggregation_attribute(node)?)),
        other => Err(invalid_hit_policy(other)),
      }
    } else {
      Ok(HitPolicy::Unique)
    }
  }

  /// Returns the value of the aggregation attribute.
  fn parse_aggregation_attribute(&self, node: &Node) -> Result<BuiltinAggregator> {
    if let Some(aggregation_text) = node.attribute(ATTR_AGGREGATION) {
      match aggregation_text.trim() {
        "COUNT" => Ok(BuiltinAggregator::Count),
        "SUM" => Ok(BuiltinAggregator::Sum),
        "MIN" => Ok(BuiltinAggregator::Min),
        "MAX" => Ok(BuiltinAggregator::Max),
        other => Err(invalid_aggregation(other)),
      }
    } else {
      Ok(BuiltinAggregator::List)
    }
  }

  /// Returns the value of the preferred decision table orientation attribute.
  fn parse_preferred_orientation_attribute(&self, node: &Node) -> Result<DecisionTableOrientation> {
    if let Some(attr_value) = node.attribute(ATTR_PREFERRED_ORIENTATION) {
      DecisionTableOrientation::try_from(attr_value)
    } else {
      Ok(DecisionTableOrientation::RuleAsRow)
    }
  }

  /// Parse DMNDI part of the diagram definitions.
  fn parse_dmndi(&self, node: &Node, definitions: &mut Definitions) -> Result<()> {
    for child_node in node.children().filter(|n| n.tag_name().name() == NODE_DMNDI) {
      let dmndi = Dmndi {
        styles: self.parse_styles(&child_node)?,
        diagrams: self.parse_diagrams(&child_node)?,
      };
      definitions.dmndi = Some(dmndi);
    }
    Ok(())
  }

  /// Parser shared styles defined in [Dmndi].
  fn parse_styles(&self, node: &Node) -> Result<Vec<DmnStyle>> {
    let mut styles = vec![];
    for child_node in node.children().filter(|n| n.tag_name().name() == NODE_DMNDI_STYLE) {
      styles.push(self.parse_style(&child_node)?);
    }
    Ok(styles)
  }

  /// Parses optional style.
  fn parse_optional_style(&self, node: &Node, child_name: &str) -> Result<Option<DmnStyle>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      Ok(Some(self.parse_style(&child_node)?))
    } else {
      Ok(None)
    }
  }

  /// Parses single style.
  fn parse_style(&self, node: &Node) -> Result<DmnStyle> {
    Ok(DmnStyle {
      id: optional_attribute(node, ATTR_ID),
      fill_color: self.parse_color(node, NODE_DMNDI_FILL_COLOR)?,
      stroke_color: self.parse_color(node, NODE_DMNDI_STROKE_COLOR)?,
      font_color: self.parse_color(node, NODE_DMNDI_FONT_COLOR)?,
      font_family: optional_string(node, ATTR_FONT_FAMILY, "Arial"),
      font_size: optional_double(node, ATTR_FONT_SIZE, 8.0),
      font_italic: optional_bool(node, ATTR_FONT_ITALIC, false),
      font_bold: optional_bool(node, ATTR_FONT_BOLD, false),
      font_underline: optional_bool(node, ATTR_FONT_UNDERLINE, false),
      font_strike_through: optional_bool(node, ATTR_FONT_STRIKE_THROUGH, false),
      label_horizontal_alignment: self.parse_alignment_kind(node, NODE_DMNDI_LABEL_HORIZONTAL_ALIGNMENT),
      label_vertical_alignment: self.parse_alignment_kind(node, NODE_DMNDI_LABEL_VERTICAL_ALIGNMENT),
    })
  }

  /// Parses a color definition.
  fn parse_color(&self, node: &Node, child_name: &str) -> Result<Option<DcColor>> {
    if let Some(color_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      Ok(Some(DcColor {
        red: required_color_part(&color_node, ATTR_RED)?,
        green: required_color_part(&color_node, ATTR_GREEN)?,
        blue: required_color_part(&color_node, ATTR_BLUE)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parser an alignment.
  fn parse_alignment_kind(&self, node: &Node, attr_name: &str) -> Option<DcAlignmentKind> {
    match node.attribute(attr_name) {
      Some("start") => Some(DcAlignmentKind::Start),
      Some("end") => Some(DcAlignmentKind::End),
      Some("center") => Some(DcAlignmentKind::Center),
      _ => None,
    }
  }

  /// Parses diagrams defined in [Dmndi].
  fn parse_diagrams(&self, node: &Node) -> Result<Vec<DmnDiagram>> {
    let mut diagrams = vec![];
    for child_node in node.children().filter(|n| n.tag_name().name() == NODE_DMNDI_DMN_DIAGRAM) {
      diagrams.push(self.parse_diagram(&child_node)?);
    }
    Ok(diagrams)
  }

  /// Parses a single diagram.
  fn parse_diagram(&self, node: &Node) -> Result<DmnDiagram> {
    Ok(DmnDiagram {
      id: optional_attribute(node, ATTR_ID),
      name: optional_string(node, ATTR_NAME, ""),
      documentation: "".to_string(),
      resolution: optional_double(node, ATTR_RESOLUTION, 300.0),
      diagram_elements: self.parse_diagram_elements(node)?,
      shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
      local_style: self.parse_optional_style(node, NODE_DMNDI_LOCAL_STYLE)?,
      size: self.parse_dimension(node)?,
    })
  }

  /// Parses dimension.
  fn parse_dimension(&self, size_node: &Node) -> Result<Option<DcDimension>> {
    if let Some(node) = size_node.children().find(|n| n.tag_name().name() == NODE_DMNDI_SIZE) {
      Ok(Some(DcDimension {
        width: required_double(&node, ATTR_WIDTH)?,
        height: required_double(&node, ATTR_HEIGHT)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses diagram elements
  fn parse_diagram_elements(&self, size_node: &Node) -> Result<Vec<DmnDiagramElement>> {
    let mut diagram_element = vec![];
    for child_node in size_node.children().filter(|n| n.tag_name().name() == NODE_DMNDI_DMN_SHAPE) {
      diagram_element.push(self.parse_shape(&child_node)?);
    }
    for child_node in size_node.children().filter(|n| n.tag_name().name() == NODE_DMNDI_DMN_EDGE) {
      diagram_element.push(self.parse_edge(&child_node)?);
    }
    Ok(diagram_element)
  }

  /// Parses shape.
  fn parse_shape(&self, node: &Node) -> Result<DmnDiagramElement> {
    Ok(DmnDiagramElement::DmnShape(DmnShape {
      id: optional_attribute(node, ATTR_ID),
      bounds: self.parse_bounds(node)?,
      dmn_element_ref: optional_attribute(node, ATTR_DMN_ELEMENT_REF),
      is_listed_input_data: false,
      decision_service_divider_line: self.parse_divider_line(node)?,
      is_collapsed: optional_bool(node, ATTR_IS_COLLAPSED, false),
      shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
      local_style: self.parse_optional_style(node, NODE_DMNDI_LOCAL_STYLE)?,
      label: self.parse_label(node)?,
    }))
  }

  /// Parses bounds.
  fn parse_bounds(&self, node: &Node) -> Result<DcBounds> {
    match self.parse_optional_bounds(node) {
      Ok(Some(n)) => Ok(n),
      _ => Err(required_child_node_is_missing(node.tag_name().name(), NODE_DMNDI_BOUNDS)),
    }
  }

  /// Parses bounds.
  fn parse_optional_bounds(&self, node: &Node) -> Result<Option<DcBounds>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == NODE_DMNDI_BOUNDS) {
      Ok(Some(DcBounds {
        x: required_double(&child_node, ATTR_X)?,
        y: required_double(&child_node, ATTR_Y)?,
        width: required_double(&child_node, ATTR_WIDTH)?,
        height: required_double(&child_node, ATTR_HEIGHT)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses decisionServiceDividerLine
  fn parse_divider_line(&self, node: &Node) -> Result<Option<DmnDecisionServiceDividerLine>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == NODE_DMNDI_DECISION_SERVICE_DIVIDER_LINE) {
      Ok(Some(DmnDecisionServiceDividerLine {
        id: optional_attribute(&child_node, ATTR_ID),
        way_points: self.parse_way_points(&child_node)?,
        shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
        local_style: self.parse_optional_style(&child_node, NODE_DMNDI_LOCAL_STYLE)?,
      }))
    } else {
      Ok(None)
    }
  }

  /// Parses edge.
  fn parse_edge(&self, node: &Node) -> Result<DmnDiagramElement> {
    Ok(DmnDiagramElement::DmnEdge(DmnEdge {
      id: optional_attribute(node, ATTR_ID),
      way_points: self.parse_way_points(node)?,
      dmn_element_ref: optional_attribute(node, ATTR_DMN_ELEMENT_REF),
      source_element: None,
      target_element: None,
      shared_style: optional_attribute(node, ATTR_SHARED_STYLE),
      local_style: self.parse_optional_style(node, NODE_DMNDI_LOCAL_STYLE)?,
      label: self.parse_label(node)?,
    }))
  }

  /// Parses wayPoints.
  fn parse_way_points(&self, node: &Node) -> Result<Vec<DcPoint>> {
    let mut way_points = vec![];
    for child_node in node.children().filter(|n| n.tag_name().name() == NODE_DMNDI_WAYPOINT) {
      way_points.push(self.parse_point(&child_node)?)
    }
    Ok(way_points)
  }

  /// Parses DcPoint.
  fn parse_point(&self, node: &Node) -> Result<DcPoint> {
    Ok(DcPoint {
      x: required_double(node, ATTR_X)?,
      y: required_double(node, ATTR_Y)?,
    })
  }

  /// Parses Label
  fn parse_label(&self, node: &Node) -> Result<Option<DmnLabel>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == NODE_DMNDI_LABEL) {
      Ok(Some(DmnLabel {
        bounds: self.parse_optional_bounds(&child_node)?,
        text: optional_attribute(&child_node, ATTR_TEXT),
        shared_style: optional_attribute(&child_node, ATTR_SHARED_STYLE),
      }))
    } else {
      Ok(None)
    }
  }
}

/// Utility helper functions for processing XML structures.
mod xml_utils {
  use super::errors::*;
  use super::*;
  use dmntk_common::{OptHRef, Result};
  use roxmltree::Node;
  use std::str::FromStr;

  /// Returns the value of the required attribute.
  pub fn required_attribute(node: &Node, attr_name: &str) -> Result<String> {
    if let Some(attr_value) = node.attribute(attr_name) {
      Ok(attr_value.to_owned())
    } else {
      Err(xml_expected_mandatory_attribute(&node_name_pos(node), attr_name))
    }
  }

  /// Returns required name attribute for specified node.
  pub fn required_name(node: &Node) -> Result<String> {
    required_attribute(node, ATTR_NAME)
  }

  /// Returns optional `FEEL` name for specified node.
  pub fn optional_feel_name(node: &Node) -> Result<Option<Name>> {
    Ok(dmntk_feel_parser::parse_longest_name(&required_name(node)?).ok())
  }

  /// Returns the value of the mandatory color attribute.
  pub fn required_color_part(node: &Node, attr_name: &str) -> Result<u8> {
    u8::from_str(&required_attribute(node, attr_name)?).map_err(|e| invalid_color_value(&e.to_string()))
  }

  /// Returns the value of the mandatory double value.
  pub fn required_double(node: &Node, attr_name: &str) -> Result<f64> {
    f64::from_str(&required_attribute(node, attr_name)?).map_err(|e| invalid_double_value(&e.to_string()))
  }

  /// Returns the value of the optional attribute.
  pub fn optional_attribute(node: &Node, attr_name: &str) -> Option<String> {
    node.attribute(attr_name).map(|attr_value| attr_value.to_owned())
  }

  /// Returns the value of the optional string attribute or default value, when specified attribute is not defined.
  pub fn optional_string(node: &Node, attr_name: &str, def_value: &str) -> String {
    optional_attribute(node, attr_name).map_or(def_value.to_owned(), |value| value)
  }

  /// Returns the value of the optional double attribute or default value, when specified attribute is not defined.
  pub fn optional_double(node: &Node, attr_name: &str, def_value: f64) -> f64 {
    optional_attribute(node, attr_name).map_or(def_value, |value| f64::from_str(&value).map_or(def_value, |value| value))
  }

  /// Returns the value of the optional bool attribute or default value, when specified attribute is not defined.
  pub fn optional_bool(node: &Node, attr_name: &str, def_value: bool) -> bool {
    optional_attribute(node, attr_name).map_or(def_value, |value| bool::from_str(&value).map_or(def_value, |value| value))
  }

  /// Returns required textual content of the node.
  pub fn required_content(node: &Node) -> Result<String> {
    if let Some(text) = node.text() {
      Ok(text.to_owned())
    } else {
      Err(xml_expected_mandatory_text_content(node.tag_name().name()))
    }
  }

  /// Returns optional textual content of the node.
  pub fn optional_content(node: &Node) -> Option<String> {
    node.text().map(|text| text.to_owned())
  }

  /// Returns required child node or raises an error when there is no child with given name.
  pub fn required_child<'a>(node: &'a Node, child_name: &str) -> Result<Node<'a, 'a>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      Ok(child_node)
    } else {
      Err(required_child_node_is_missing(&node_name_pos(node), child_name))
    }
  }

  /// Returns child node when there is a child with the given name.
  pub fn optional_child<'a>(node: &'a Node, child_name: &str) -> Option<Node<'a, 'a>> {
    node.children().find(|n| n.tag_name().name() == child_name)
  }

  /// Returns the required text content of the required child node.
  pub fn required_child_required_content(node: &Node, child_name: &str) -> Result<String> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      required_content(&child_node)
    } else {
      Err(xml_expected_mandatory_child_node(&node_name_pos(node), child_name))
    }
  }

  /// Returns the required content of the optional child node.
  pub fn optional_child_required_content(node: &Node, child_name: &str) -> Result<Option<String>> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      Ok(Some(required_content(&child_node)?))
    } else {
      Ok(None)
    }
  }

  /// Returns the optional content of the optional child node.
  pub fn optional_child_optional_content(node: &Node, child_name: &str) -> Option<String> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      optional_content(&child_node)
    } else {
      None
    }
  }

  /// Returns the required attribute of the optional child node.
  pub fn optional_child_required_href(node: &Node, child_name: &str) -> Result<OptHRef> {
    if let Some(child_node) = node.children().find(|n| n.tag_name().name() == child_name) {
      Ok(Some(HRef::try_from(required_attribute(&child_node, ATTR_HREF)?.as_str())?))
    } else {
      Ok(None)
    }
  }

  /// XML utility function that returns node's name with node's position in the original document.
  pub fn node_name_pos(n: &Node) -> String {
    format!("`{}` at [{}]", n.tag_name().name(), n.document().text_pos_at(n.range().start))
  }
}

/// Definitions of errors raised while parsing the XML model.
mod errors {
  use dmntk_common::DmntkError;

  /// Errors related with parsing the decision model.
  enum ModelParserError {
    /// Raised when parsed text is not a valid function kind,
    /// accepted values are: `FEEL`, `Java` or `PMML`.
    InvalidFunctionKind(String),
    /// Raised when parsed text is not a valid hit policy,
    /// accepted values are: `UNIQUE`, `FIRST`, `PRIORITY`,
    /// `ANY`, `COLLECT`, `RULE ORDER`, or `OUTPUT ORDER`.
    InvalidHitPolicy(String),
    /// Raised when parsed text is not a valid aggregation for hit policy,
    /// accepted values are: `COUNT`, `SUM`, `MIN`, or `MAX`.
    InvalidAggregation(String),
    /// Invalid value for a color.
    InvalidColorValue(String),
    /// Invalid value for a double.
    InvalidDoubleValue(String),
    /// Raised when required `inputExpression` node is missing.
    RequiredInputExpressionIsMissing,
    /// Raised when required child node is missing.
    RequiredChildNodeIsMissing(String, String),
    /// Raised when required expression instance is missing.
    RequiredExpressionInstanceIsMissing,
    /// Raised when the number of elements in a row differs from the number of columns in relation.
    NumberOfElementsInRowDiffersFromNumberOfColumns,
    ///
    XmlParsingModelFailed(String),
    ///
    XmlUnexpectedNode(String, String),
    ///
    XmlExpectedMandatoryAttribute(String, String),
    ///
    XmlExpectedMandatoryChildNode(String, String),
    ///
    XmlExpectedMandatoryTextContent(String),
  }

  impl From<ModelParserError> for DmntkError {
    fn from(e: ModelParserError) -> Self {
      DmntkError::new("ModelParserError", &format!("{}", e))
    }
  }

  impl std::fmt::Display for ModelParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      match self {
        ModelParserError::InvalidFunctionKind(s) => {
          write!(f, "'{}' is not a valid function kind, accepted values are: `FEEL`, `Java`, `PMML`", s)
        }
        ModelParserError::InvalidHitPolicy(s) => {
          write!(
            f,
            "'{}' is not a valid hit policy, allowed values are: `UNIQUE`, `FIRST`, `PRIORITY`, `ANY`, `COLLECT`, `RULE ORDER`, `OUTPUT ORDER`",
            s
          )
        }
        ModelParserError::InvalidAggregation(s) => {
          write!(f, "'{}' is not a valid aggregation, allowed values are: `COUNT`, `SUM`, `MIN`, `MAX`", s)
        }
        ModelParserError::InvalidColorValue(s) => {
          write!(f, "conversion to valid color value failed with reason: {}", s)
        }
        ModelParserError::InvalidDoubleValue(reason) => {
          write!(f, "conversion to valid double value failed with reason: {}", reason)
        }
        ModelParserError::RequiredInputExpressionIsMissing => {
          write!(f, "required input expression in decision table's input clause is missing")
        }
        ModelParserError::RequiredChildNodeIsMissing(s1, s2) => {
          write!(f, "required child node '{}' in parent node '{}' is missing", s2, s1)
        }
        ModelParserError::RequiredExpressionInstanceIsMissing => {
          write!(f, "required expression instance in context entry is missing")
        }
        ModelParserError::NumberOfElementsInRowDiffersFromNumberOfColumns => {
          write!(f, "number of elements in a row differs from the number of columns defined in a relation")
        }
        ModelParserError::XmlParsingModelFailed(s) => {
          write!(f, "parsing model from XML failed with reason: {}", s)
        }
        ModelParserError::XmlUnexpectedNode(s1, s2) => {
          write!(f, "unexpected XML node, expected: {}, actual: {}", s1, s2)
        }
        ModelParserError::XmlExpectedMandatoryAttribute(s1, s2) => {
          write!(f, "expected value for mandatory attribute `{}` in node `{}`", s2, s1)
        }
        ModelParserError::XmlExpectedMandatoryChildNode(s1, s2) => {
          write!(f, "expected mandatory child node '{}' in parent node '{}'", s2, s1)
        }
        ModelParserError::XmlExpectedMandatoryTextContent(s) => {
          write!(f, "expected mandatory text content in node: {}", s)
        }
      }
    }
  }

  pub fn invalid_function_kind(s: &str) -> DmntkError {
    ModelParserError::InvalidFunctionKind(s.to_owned()).into()
  }

  pub fn invalid_hit_policy(s: &str) -> DmntkError {
    ModelParserError::InvalidHitPolicy(s.to_owned()).into()
  }

  pub fn invalid_aggregation(s: &str) -> DmntkError {
    ModelParserError::InvalidAggregation(s.to_owned()).into()
  }

  pub fn invalid_color_value(s: &str) -> DmntkError {
    ModelParserError::InvalidColorValue(s.to_owned()).into()
  }

  pub fn invalid_double_value(reason: &str) -> DmntkError {
    ModelParserError::InvalidDoubleValue(reason.to_owned()).into()
  }

  pub fn required_child_node_is_missing(s1: &str, s2: &str) -> DmntkError {
    ModelParserError::RequiredChildNodeIsMissing(s1.to_owned(), s2.to_owned()).into()
  }

  pub fn required_input_expression_is_missing() -> DmntkError {
    ModelParserError::RequiredInputExpressionIsMissing.into()
  }

  pub fn required_expression_instance_is_missing() -> DmntkError {
    ModelParserError::RequiredExpressionInstanceIsMissing.into()
  }

  pub fn number_of_elements_in_row_differs_from_number_of_columns() -> DmntkError {
    ModelParserError::NumberOfElementsInRowDiffersFromNumberOfColumns.into()
  }

  pub fn xml_parsing_model_failed(s: &str) -> DmntkError {
    ModelParserError::XmlParsingModelFailed(s.to_owned()).into()
  }

  pub fn xml_unexpected_node(s1: &str, s2: &str) -> DmntkError {
    ModelParserError::XmlUnexpectedNode(s1.to_owned(), s2.to_owned()).into()
  }

  pub fn xml_expected_mandatory_attribute(s1: &str, s2: &str) -> DmntkError {
    ModelParserError::XmlExpectedMandatoryAttribute(s1.to_owned(), s2.to_owned()).into()
  }

  pub fn xml_expected_mandatory_child_node(s1: &str, s2: &str) -> DmntkError {
    ModelParserError::XmlExpectedMandatoryChildNode(s1.to_owned(), s2.to_owned()).into()
  }

  pub fn xml_expected_mandatory_text_content(s: &str) -> DmntkError {
    ModelParserError::XmlExpectedMandatoryTextContent(s.to_owned()).into()
  }
}
