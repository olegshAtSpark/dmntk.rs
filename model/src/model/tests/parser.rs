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

const DMNTK_NAMESPACE: &str = "https://dmntk.io";

#[test]
fn _2_0001() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0001).unwrap();
  assert_eq!("_c910c9ba-c584-4ac9-a773-1e6de185cd85", definitions.id().as_ref().unwrap().as_str());
  assert_eq!("Compliance level 2. Test 0001.", definitions.description().as_ref().unwrap().as_str());
  assert!(definitions.label().is_none());
  assert_eq!("compliance-level-2-test-0001", definitions.name());
  assert_eq!("https://dmntk.io/2_0001", definitions.namespace());
  assert!(definitions.expression_language().is_none());
  assert!(definitions.type_language().is_none());
  assert!(definitions.exporter().is_none());
  assert!(definitions.exporter_version().is_none());
  assert_eq!(2, definitions.drg_elements.len());
  assert!(definitions.decision_by_id("_75b3add2-4d36-4a19-a76c-268b49b2f436").is_some());
  // <decision>
  let decision = definitions.decision_by_id("_75b3add2-4d36-4a19-a76c-268b49b2f436").unwrap();
  assert_eq!("_75b3add2-4d36-4a19-a76c-268b49b2f436", decision.id().as_ref().unwrap().as_str());
  assert!(decision.description().is_none());
  assert!(decision.label().is_none());
  assert_eq!("Greeting Message", decision.name());
  // <decision>.<variable>
  let decision_variable = decision.variable();
  assert!(decision_variable.id().is_none());
  assert!(decision_variable.description().is_none());
  assert!(decision_variable.label().is_none());
  assert_eq!("Greeting Message", decision_variable.name());
  assert_eq!("string", decision_variable.type_ref().as_ref().unwrap().as_str());
  // <decision>.<informationRequirement>
  let information_requirement_items = decision.information_requirements();
  assert_eq!(1, information_requirement_items.len());
  let information_requirement = &information_requirement_items[0];
  assert_eq!("_8c935b50-10b7-426b-80a9-dddb4264b4a9", information_requirement.id().as_ref().unwrap().as_str());
  assert!(information_requirement.description().is_none());
  assert!(information_requirement.label().is_none());
  assert!(information_requirement.required_decision().is_none());
  let required_input: &str = information_requirement.required_input().as_ref().unwrap().into();
  assert_eq!("_cba86e4d-e91c-46a2-9176-e9adf88e15db", required_input);
  // <decision>.<literalExpression>
  let expression = decision.decision_logic().as_ref().unwrap();
  match expression {
    ExpressionInstance::LiteralExpression(literal_expression) => {
      assert_eq!(r#""Hello " + Full Name"#, literal_expression.text().as_ref().unwrap().as_str());
    }
    _ => unimplemented!(),
  }
  // <inputData>
  let input_data = definitions.input_data_by_id("_cba86e4d-e91c-46a2-9176-e9adf88e15db").unwrap();
  assert_eq!("_cba86e4d-e91c-46a2-9176-e9adf88e15db", input_data.id().as_ref().unwrap().as_str());
  assert!(input_data.description().is_none());
  assert!(input_data.label.is_none());
  assert_eq!("Full Name", input_data.name());
  // <inputData>.<variable>
  let input_data_variable = input_data.variable();
  assert!(input_data_variable.id().is_none());
  assert!(input_data_variable.description().is_none());
  assert!(input_data_variable.label().is_none());
  assert_eq!("Full Name", input_data_variable.name());
  assert_eq!("string", input_data_variable.type_ref().as_ref().unwrap().as_str());
}

#[test]
fn _2_0002() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0002).unwrap();
  assert_eq!("_072005e3-2635-47c9-8dec-5aca4b869376", definitions.id().as_ref().unwrap().as_str());
  assert_eq!("https://dmntk.io/2_0002", definitions.namespace());
}

#[test]
fn _2_0003() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0003).unwrap();
  assert_eq!("_ce8a8d19-38c0-4289-8a46-ff72f881e71f", definitions.id().as_ref().unwrap().as_str());
  assert_eq!("Compliance level 2. Test 0003.", definitions.description().as_ref().unwrap().as_str());
  assert!(definitions.label().is_none());
  assert_eq!("compliance-level-2-test-0003", definitions.name());
  assert_eq!(DMNTK_NAMESPACE, definitions.namespace());
  assert_eq!(2, definitions.drg_elements.len());
  // <definitions>.<itemDefinition>
  let item_definition = definitions.item_definition_by_name("tEmploymentStatus").unwrap();
  assert!(item_definition.id().is_none());
  assert!(item_definition.description().is_none());
  assert!(item_definition.label().is_none());
  assert_eq!("tEmploymentStatus", item_definition.name());
  assert_eq!("string", item_definition.type_ref().as_ref().unwrap().as_str());
  assert!(item_definition.type_language().is_none());
  assert!(item_definition.item_components().is_empty());
  assert!(!item_definition.is_collection());
  // <definitions>.<itemDefinition>.<allowedValues>
  let allowed_values = item_definition.allowed_values().as_ref().unwrap();
  assert_eq!(
    r#""UNEMPLOYED","EMPLOYED","SELF-EMPLOYED","STUDENT""#,
    allowed_values.text().as_ref().unwrap().as_str()
  );
  assert!(allowed_values.expression_language().is_none());
  // <definitions>.<decision>
  let decision = definitions.decision_by_id("d_EmploymentStatusStatement").unwrap();
  assert_eq!("d_EmploymentStatusStatement", decision.id().as_ref().unwrap().as_str());
  assert!(decision.description().is_none());
  assert!(decision.label().is_none());
  assert_eq!("Employment Status Statement", decision.name());
  // <definitions>.<decision>.<variable>
  let decision_variable = decision.variable();
  assert!(decision_variable.id().is_none());
  assert!(decision_variable.description().is_none());
  assert!(decision_variable.label().is_none());
  assert_eq!("Employment Status Statement", decision_variable.name());
  assert_eq!("string", decision_variable.type_ref().as_ref().unwrap().as_str());
  // <definitions>.<decision>.<informationRequirement>
  let information_requirement_items = decision.information_requirements();
  assert_eq!(1, information_requirement_items.len());
  let information_requirement = &information_requirement_items[0];
  assert_eq!("f4a0451b-8db5-401a-b9b4-dc31416b6e7d", information_requirement.id().as_ref().unwrap().as_str());
  assert!(information_requirement.description().is_none());
  assert!(information_requirement.label().is_none());
  assert!(information_requirement.required_decision().is_none());
  let required_input: &str = information_requirement.required_input().as_ref().unwrap().into();
  assert_eq!("i_EmploymentStatus", required_input);
  // <definitions>.<decision>.<literalExpression>
  let expression = decision.decision_logic().as_ref().unwrap();
  match expression {
    ExpressionInstance::LiteralExpression(literal_expression) => {
      assert_eq!(r#""You are " + Employment Status"#, literal_expression.text().as_ref().unwrap().as_str());
    }
    _ => unimplemented!(),
  }
  // <definitions>.<inputData>
  let input_data = definitions.input_data_by_id("i_EmploymentStatus").unwrap();
  assert_eq!("i_EmploymentStatus", input_data.id().as_ref().unwrap().as_str());
  assert!(input_data.description().is_none());
  assert!(input_data.label().is_none());
  assert_eq!("Employment Status", input_data.name());
  // <definitions>.<inputData>.<variable>
  let input_data_variable = input_data.variable();
  assert!(input_data_variable.id().is_none());
  assert!(input_data_variable.description().is_none());
  assert!(input_data_variable.label().is_none());
  assert_eq!("Employment Status", input_data_variable.name());
  assert_eq!("tEmploymentStatus", input_data_variable.type_ref().as_ref().unwrap().as_str());
}

#[test]
fn _2_0004() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0004).unwrap();
  assert_eq!("_edbd2d8e-a5a8-4660-9bb9-adaa792d900c", definitions.id().as_ref().unwrap().as_str());
  assert!(definitions.description().is_none());
  assert!(definitions.label().is_none());
  assert_eq!("compliance-level-2-test-0004", definitions.name());
  assert_eq!("https://dmntk.io", definitions.namespace());
  assert_eq!(4, definitions.drg_elements.len());
  // checking node: <definitions>.<decision>
  let decision = definitions.decision_by_id("_3b2953a3-745f-4d2e-b55d-75c8c5ae653c").unwrap();
  assert_eq!("_3b2953a3-745f-4d2e-b55d-75c8c5ae653c", decision.id().as_ref().unwrap().as_str());
  assert!(decision.description().is_none());
  assert!(decision.label().is_none());
  assert_eq!("Approval Status", decision.name());
  // checking node: <definitions>.<decision>.<variable>
  let decision_variable = decision.variable();
  assert!(decision_variable.id().is_none());
  assert!(decision_variable.description().is_none());
  assert!(decision_variable.label().is_none());
  assert_eq!("Approval Status", decision_variable.name());
  assert_eq!("string", decision_variable.type_ref().as_ref().unwrap().as_str());
  // checking node: <definitions>.<decision>.<decisionTable>
  if let ExpressionInstance::DecisionTable(decision_table) = decision.decision_logic().as_ref().unwrap() {
    assert_eq!(HitPolicy::Unique, decision_table.hit_policy);
    assert_eq!(DecisionTableOrientation::RuleAsRow, decision_table.preferred_orientation);
    assert_eq!("Approval Status", decision_table.output_label.as_ref().unwrap().as_str());
    assert_eq!(3, decision_table.input_clauses.len());
    assert_eq!(1, decision_table.output_clauses.len());
    assert_eq!(4, decision_table.rules.len());
  } else {
    unimplemented!()
  }
}

#[test]
fn _2_0005() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0005).unwrap();
  assert_eq!("_6cb03678-38e5-4ee3-826b-d6622c738563", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _2_0006() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0006).unwrap();
  assert_eq!("_ecea0c06-d5d0-41c2-9ba9-a9153fb47e7b", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _2_0008() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0008).unwrap();
  assert_eq!("_e2d61fd1-c220-4359-9f7e-b42474c7983f", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _2_0009() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0009).unwrap();
  assert_eq!("_0ffc2622-d6db-4650-aac3-46df54e9d5c2", definitions.id().as_ref().unwrap().as_str());
  assert!(definitions.description().is_none());
  assert!(definitions.label().is_none());
  assert_eq!("compliance-level-2-test-0009", definitions.name());
  assert_eq!("https://dmntk.io", definitions.namespace());
  assert_eq!(4, definitions.drg_elements.len());
  // <definitions>.<decision>
  let decision = definitions.decision_by_id("d_MonthlyPayment").unwrap();
  assert_eq!("d_MonthlyPayment", decision.id().as_ref().unwrap().as_str());
  assert!(decision.description().is_none());
  assert!(decision.label().is_none());
  assert_eq!("MonthlyPayment", decision.name());
  // <definitions>.<decision>.<variable>
  let decision_variable = decision.variable();
  assert!(decision_variable.id().is_none());
  assert!(decision_variable.description().is_none());
  assert!(decision_variable.label().is_none());
  assert_eq!("MonthlyPayment", decision_variable.name());
  assert_eq!("number", decision_variable.type_ref().as_ref().unwrap().as_str());
  // <definitions>.<decision>.<informationRequirement>
  let information_requirement_items = decision.information_requirements();
  assert_eq!(2, information_requirement_items.len());
  // <definitions>.<decision>.<knowledgeRequirement>
  let knowledge_requirement_items = decision.knowledge_requirements();
  assert_eq!(1, knowledge_requirement_items.len());
  let knowledge_requirement = &knowledge_requirement_items[0];
  assert_eq!("_1684f08c-413b-4ecd-8caf-d922500940bd", knowledge_requirement.id().as_ref().unwrap().as_str());
  assert!(knowledge_requirement.description().is_none());
  assert!(knowledge_requirement.label().is_none());
  let required_knowledge: &str = knowledge_requirement.required_knowledge().as_ref().unwrap().into();
  assert_eq!("b_PMT", required_knowledge);
  // <definitions>.<businessKnowledgeModel>
  let bkm = definitions.business_knowledge_model_by_id("b_PMT").unwrap();
  assert_eq!("b_PMT", bkm.id().as_ref().unwrap().as_str());
  assert!(bkm.description().is_none());
  assert!(bkm.label().is_none());
  assert_eq!("PMT", bkm.name());
  // <definitions>.<businessKnowledgeModel>.<variable>
  let bkm_variable = bkm.variable();
  assert!(bkm_variable.id().is_none());
  assert!(bkm_variable.description().is_none());
  assert!(bkm_variable.label().is_none());
  assert_eq!("PMT", bkm_variable.name());
  //assert!(bkm_variable.type_ref().is_none());
  // <definitions>.<businessKnowledgeModel>.<encapsulateLogic>
  let bkm_function_definition = bkm.encapsulated_logic().as_ref().unwrap();
  assert_eq!(3, bkm_function_definition.formal_parameters().len());
  let formal_parameters = bkm_function_definition.formal_parameters();
  assert_eq!("p", formal_parameters[0].name());
  assert_eq!("r", formal_parameters[1].name());
  assert_eq!("n", formal_parameters[2].name());
  let bkm_body = bkm_function_definition.body().as_ref().unwrap();
  match bkm_body {
    ExpressionInstance::LiteralExpression(literal_expression) => {
      assert_eq!(r#"(p*r/12)/(1-(1+r/12)**-n)"#, literal_expression.text().as_ref().unwrap().as_str());
    }
    _ => unimplemented!(),
  }
}

#[test]
fn _2_0010() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0010).unwrap();
  assert_eq!("_a4a902f2-1948-4e06-8035-da8098345536", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _2_0106() {
  let definitions = crate::parse(dmntk_examples::DMN_2_0106).unwrap();
  assert_eq!("_09efab6a-892b-452f-9182-492a89540a20", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0001() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0001).unwrap();
  assert_eq!("_8e0912d6-54f3-4276-81a5-eebc712af710", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0002() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0002).unwrap();
  assert_eq!("_536af77f-8f8b-4339-b00d-28116bb0c3f8", definitions.id().as_ref().unwrap().as_str());
  // <definitions>.<decision>
  let decision = definitions.decision_by_id("_de5529b1-ed4c-4b39-9e36-e0e056aec20c").unwrap();
  assert_eq!("_de5529b1-ed4c-4b39-9e36-e0e056aec20c", decision.id().as_ref().unwrap().as_str());
  assert!(decision.description().is_none());
  assert!(decision.label().is_none());
  assert_eq!("Basic", decision.name());
  // <definitions>.<decision>.<variable>
  let decision_variable = decision.variable();
  assert!(decision_variable.id().is_none());
  assert!(decision_variable.description().is_none());
  assert!(decision_variable.label().is_none());
  assert_eq!("Basic", decision_variable.name());
  assert_eq!("tBasic", decision_variable.type_ref().as_ref().unwrap().as_str());
  let expression = decision.decision_logic().as_ref().unwrap();
  match expression {
    ExpressionInstance::Context(context) => {
      assert_eq!(12, context.context_entries.len());
    }
    _ => unimplemented!(),
  }
}

#[test]
fn _3_0003() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0003).unwrap();
  assert_eq!("_54863c52-2fa7-4a3d-b383-d4eb2eb88771", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0004() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0004).unwrap();
  assert_eq!("_52650d27-fc13-463e-a55a-2080e8a1c8da", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0006() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0006).unwrap();
  assert_eq!("_40a27b80-d50e-4e60-9905-178d5e3065e3", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0007() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0007).unwrap();
  assert_eq!("_0705a5f4-80df-41b5-9553-79c1ba3dff6c", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0008() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0008).unwrap();
  assert_eq!("_15af8727-45c8-4dc8-8435-ed087bfb35d9", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0014() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0014).unwrap();
  assert_eq!("_56c7d4a5-e6db-4bba-ac5f-dc082a16f719", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0016() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0016).unwrap();
  assert_eq!("_51b7609d-c550-4660-b4c1-6ee5b4f1e8fe", definitions.id().as_ref().unwrap().as_str());
  // <definitions>.<decision>
  let decision = definitions.decision_by_id("_a471e76a-64b1-44af-9ede-623f6c15b72e").unwrap();
  assert_eq!("_a471e76a-64b1-44af-9ede-623f6c15b72e", decision.id().as_ref().unwrap().as_str());
  assert!(decision.description().is_none());
  assert!(decision.label().is_none());
  assert_eq!("priceTable1", decision.name());
  // <definitions>.<decision>.<variable>
  let decision_variable = decision.variable();
  assert!(decision_variable.id().is_none());
  assert!(decision_variable.description().is_none());
  assert!(decision_variable.label().is_none());
  assert_eq!("priceTable1", decision_variable.name());
  assert_eq!("tPriceTable", decision_variable.type_ref().as_ref().unwrap().as_str());
  let expression = decision.decision_logic().as_ref().unwrap();
  match expression {
    ExpressionInstance::Relation(relation) => {
      assert_eq!(2, relation.columns().len());
      assert_eq!("itemName", relation.columns()[0].name);
      assert_eq!("price", relation.columns()[1].name);
      assert_eq!(3, relation.rows().len());
      assert_eq!(2, relation.rows()[0].elements().len());
      assert_eq!(2, relation.rows()[1].elements().len());
      assert_eq!(2, relation.rows()[2].elements().len());
    }
    _ => unimplemented!(),
  }
}

#[test]
fn _3_0020() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0020).unwrap();
  assert_eq!("_819c015c-7ede-4404-876e-e96417aed322", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0030() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0030).unwrap();
  assert_eq!("_3cc4a1ee-68b8-4fc3-b686-e690f8fa7dcc", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0082() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0082).unwrap();
  assert_eq!("_6cd7f5d2-7cbb-45ea-b9bd-58ab1b6f678c", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0085() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0085).unwrap();
  assert_eq!("_3e0df83c-a19e-4d31-98e0-b73db25df89c", definitions.id().as_ref().unwrap().as_str());
}

#[test]
fn _3_0086() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0086).unwrap();
  assert_eq!("_8bb2d2bb-b981-415c-a5c3-cdb255f2d967", definitions.id().as_ref().unwrap().as_str());
  let imports = definitions.imports();
  assert_eq!(1, imports.len());
  let import = &imports[0];
  assert_eq!(None, import.id().as_ref());
  assert_eq!(None, import.description().as_ref());
  assert_eq!(None, import.label().as_ref());
  assert_eq!(None, import.extension_elements().as_ref());
  assert_eq!(0, import.extension_attributes().len());
  assert_eq!("myimport", import.name());
  assert_eq!("myimport", import.feel_name().as_ref().unwrap().to_string());
  assert_eq!(URI_MODEL, import.import_type());
  assert_eq!(None, import.location_uri().as_ref());
  assert_eq!("http://www.trisotech.com/definitions/_f27bb64b-6fc7-4e1f-9848-11ba35e0df36", import.namespace());
}

#[test]
fn _3_0087() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0087).unwrap();
  assert_eq!("_9d01a0c4-f529-4ad8-ad8e-ec5fb5d96ad4", definitions.id().as_ref().unwrap().as_str());
  // <definitions>.<knowledgeSource>
  let knowledge_source = definitions.knowledge_source_by_id("_989d137f-86ff-4249-813f-af67c08a2762").unwrap();
  assert_eq!("_989d137f-86ff-4249-813f-af67c08a2762", knowledge_source.id().as_ref().unwrap().as_str());
  assert_eq!("Credit officer", knowledge_source.name());
  assert_eq!("Credit officer", knowledge_source.feel_name().as_ref().unwrap().to_string());
  assert_eq!(
    r#"Name(["Credit", "officer"])"#,
    format!("{:?}", knowledge_source.feel_name().as_ref().unwrap())
  );
}

#[test]
fn _3_0088() {
  let definitions = crate::parse(dmntk_examples::DMN_3_0088).unwrap();
  assert_eq!("_67ff55d6-2882-4432-89a9-354faba866be", definitions.id().as_ref().unwrap().as_str());
}
