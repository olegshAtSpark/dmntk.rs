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

use crate::examples::*;
use crate::{DMNTK_DESCRIPTION, DMNTK_VERSION};
use clap::{load_yaml, App, AppSettings};
use dmntk_common::ascii_ctrl::*;
use dmntk_common::{ascii256, Jsonify};
use dmntk_feel::values::Value;
use dmntk_feel::Scope;
use dmntk_model::model::{DmnElement, NamedElement, RequiredVariable};

/// Available command-line actions.
enum Action {
  /// Parse `FEEL` expression.
  ParseFeelExpression(String, String),
  /// Evaluate `FEEL` expression.
  EvaluateFeelExpression(String, String),
  /// Test `FEEL` expression.
  TestFeelExpression(String, String, bool),
  /// Export `FEEL` expression to HTML.
  ExportFeelExpression(String, String),
  /// Parse decision table.
  ParseDecisionTable(String),
  /// Evaluate decision table.
  EvaluateDecisionTable(String, String),
  /// Test decision table.
  TestDecisionTable(String, String, bool),
  /// Export decision table.
  ExportDecisionTable(String, String),
  /// Recognize decision table.
  RecognizeDecisionTable(String),
  /// Parse `DMN` model`.
  ParseDmnModel(String),
  /// Evaluate `DMN` model`.
  EvaluateDmnModel(String, String, String),
  /// Test `DMN` model`.
  TestDmnModel(String, String, String, bool),
  /// Export `DMN` model`.
  ExportDmnModel(String, String),
  /// Start `dmntk` as a service.
  StartService(Option<String>, Option<String>, Option<String>),
  /// Generate examples.
  GenerateExamples,
  /// Do nothing, no action was specified.
  DoNothing,
}

/// Executes command-line action.
pub async fn do_action() -> std::io::Result<()> {
  match get_cli_action() {
    Action::ParseFeelExpression(ctx_file_name, feel_file_name) => {
      parse_feel_expression(&ctx_file_name, &feel_file_name);
      Ok(())
    }
    Action::EvaluateFeelExpression(input_file_name, feel_file_name) => {
      evaluate_feel_expression(&input_file_name, &feel_file_name);
      Ok(())
    }
    Action::TestFeelExpression(test_file_name, feel_file_name, summary_only) => {
      test_feel_expression(&test_file_name, &feel_file_name, summary_only);
      Ok(())
    }
    Action::ExportFeelExpression(feel_file_name, html_file_name) => {
      export_feel_expression(&feel_file_name, &html_file_name);
      Ok(())
    }
    Action::ParseDecisionTable(dectab_file_name) => {
      parse_decision_table(&dectab_file_name);
      Ok(())
    }
    Action::EvaluateDecisionTable(input_file_name, dectab_file_name) => {
      evaluate_decision_table(&input_file_name, &dectab_file_name);
      Ok(())
    }
    Action::TestDecisionTable(test_file_name, dectab_file_name, summary_only) => {
      test_decision_table(&test_file_name, &dectab_file_name, summary_only);
      Ok(())
    }
    Action::ExportDecisionTable(dectab_file_name, html_file_name) => {
      export_decision_table(&dectab_file_name, &html_file_name);
      Ok(())
    }
    Action::RecognizeDecisionTable(dectab_file_name) => {
      recognize_decision_table(&dectab_file_name);
      Ok(())
    }
    Action::ParseDmnModel(dectab_file_name) => {
      parse_dmn_model(&dectab_file_name);
      Ok(())
    }
    Action::EvaluateDmnModel(dectab_file_name, ctx_file_name, invocable_name) => {
      evaluate_dmn_model(&dectab_file_name, &ctx_file_name, &invocable_name);
      Ok(())
    }
    Action::TestDmnModel(test_file_name, dectab_file_name, invocable_name, summary_only) => {
      test_dmn_model(&test_file_name, &dectab_file_name, &invocable_name, summary_only);
      Ok(())
    }
    Action::ExportDmnModel(dectab_file_name, html_file_name) => {
      export_dmn_model(&dectab_file_name, &html_file_name);
      Ok(())
    }
    Action::StartService(opt_host, opt_port, opt_dir) => dmntk_server::start_server(opt_host, opt_port, opt_dir).await,
    Action::GenerateExamples => {
      generate_examples();
      Ok(())
    }
    Action::DoNothing => Ok(()),
  }
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to valid argument.
fn get_cli_action() -> Action {
  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml)
    .version(DMNTK_VERSION)
    .about(DMNTK_DESCRIPTION)
    .setting(AppSettings::SubcommandRequiredElseHelp)
    .get_matches();
  // parse FEEL expression subcommand
  if let Some(matches) = matches.subcommand_matches("pfe") {
    return Action::ParseFeelExpression(
      matches.value_of("CONTEXT_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("FEEL_FILE").unwrap_or("unknown.feel").to_string(),
    );
  }
  // evaluate FEEL expression subcommand
  if let Some(matches) = matches.subcommand_matches("efe") {
    return Action::EvaluateFeelExpression(
      matches.value_of("INPUT_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("FEEL_FILE").unwrap_or("unknown.feel").to_string(),
    );
  }
  // test FEEL expression subcommand
  if let Some(matches) = matches.subcommand_matches("tfe") {
    return Action::TestFeelExpression(
      matches.value_of("TEST_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("FEEL_FILE").unwrap_or("unknown.feel").to_string(),
      matches.is_present("summary"),
    );
  }
  // export FEEL expression subcommand
  if let Some(matches) = matches.subcommand_matches("xfe") {
    return Action::ExportFeelExpression(
      matches.value_of("FEEL_FILE").unwrap_or("unknown.feel").to_string(),
      matches.value_of("HTML_FILE").unwrap_or("unknown.html").to_string(),
    );
  }
  // parse decision table subcommand
  if let Some(matches) = matches.subcommand_matches("pdt") {
    return Action::ParseDecisionTable(matches.value_of("DECTAB_FILE").unwrap_or("unknown.dtb").to_string());
  }
  // evaluate decision table subcommand
  if let Some(matches) = matches.subcommand_matches("edt") {
    return Action::EvaluateDecisionTable(
      matches.value_of("INPUT_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("DECTAB_FILE").unwrap_or("unknown.dtb").to_string(),
    );
  }
  // test decision table subcommand
  if let Some(matches) = matches.subcommand_matches("tdt") {
    return Action::TestDecisionTable(
      matches.value_of("TEST_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("DECTAB_FILE").unwrap_or("unknown.dtb").to_string(),
      matches.is_present("summary"),
    );
  }
  // export decision table subcommand
  if let Some(matches) = matches.subcommand_matches("xdt") {
    return Action::ExportDecisionTable(
      matches.value_of("DECTAB_FILE").unwrap_or("unknown.dtb").to_string(),
      matches.value_of("HTML_FILE").unwrap_or("unknown.html").to_string(),
    );
  }
  // recognize decision table subcommand
  if let Some(matches) = matches.subcommand_matches("rdt") {
    return Action::RecognizeDecisionTable(matches.value_of("DECTAB_FILE").unwrap_or("unknown.dtb").to_string());
  }
  // parse DMN model subcommand
  if let Some(matches) = matches.subcommand_matches("pdm") {
    return Action::ParseDmnModel(matches.value_of("DMN_FILE").unwrap_or("unknown.dmn").to_string());
  }
  // evaluate DMN model subcommand
  if let Some(matches) = matches.subcommand_matches("edm") {
    return Action::EvaluateDmnModel(
      matches.value_of("INPUT_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("DMN_FILE").unwrap_or("unknown.dmn").to_string(),
      matches.value_of("invocable").unwrap_or("unknown").to_string(),
    );
  }
  // test DMN model subcommand
  if let Some(matches) = matches.subcommand_matches("tdm") {
    return Action::TestDmnModel(
      matches.value_of("TEST_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("DMN_FILE").unwrap_or("unknown.dmn").to_string(),
      matches.value_of("invocable").unwrap_or("unknown").to_string(),
      matches.is_present("summary"),
    );
  }
  // export DMN model subcommand
  if let Some(matches) = matches.subcommand_matches("xdm") {
    return Action::ExportDmnModel(
      matches.value_of("DMN_FILE").unwrap_or("unknown.dmn").to_string(),
      matches.value_of("HTML_FILE").unwrap_or("unknown.html").to_string(),
    );
  }
  // start server subcommand
  if let Some(matches) = matches.subcommand_matches("srv") {
    return Action::StartService(
      matches.value_of("host").map(|host| host.to_string()),
      matches.value_of("port").map(|port| port.to_string()),
      matches.value_of("dir").map(|dir| dir.to_string()),
    );
  }
  // generate examples
  if let Some(_matches) = matches.subcommand_matches("exs") {
    return Action::GenerateExamples;
  }
  Action::DoNothing
}

/// Parses `FEEL` expression loaded from file and prints the parsed `AST` to standard output.
fn parse_feel_expression(ctx_file_name: &str, feel_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(feel_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&Scope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_expression(&ctx.into(), &feel_expression, false) {
          Ok(ast_root_node) => {
            println!("    AST:{}", ast_root_node.to_string().trim_end());
          }
          Err(reason) => println!("parsing expression failed with reason: {}", reason),
        },
        Err(reason) => println!("evaluating context failed with reason: {}", reason),
      },
      Err(reason) => println!("loading context file `{}` failed with reason: {:?}", ctx_file_name, reason),
    },
    Err(reason) => println!("loading expression file `{}` failed with reason: {:?}", feel_file_name, reason),
  }
}

/// Evaluates `FEEL` expression loaded from file and prints the result to standard output.
fn evaluate_feel_expression(ctx_file_name: &str, feel_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(textual_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&Scope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_expression(&ctx.clone().into(), &textual_expression, false) {
          Ok(ast_root_node) => match dmntk_evaluator::evaluate(&ctx.into(), &ast_root_node) {
            Ok(result) => {
              println!("{}", result);
            }
            Err(reason) => println!("evaluating expression failed with reason: {}", reason),
          },
          Err(reason) => println!("parsing expression failed with reason: {}", reason),
        },
        Err(reason) => println!("evaluating context failed with reason: {}", reason),
      },
      Err(reason) => println!("loading context file `{}` failed with reason: {:?}", ctx_file_name, reason),
    },
    Err(reason) => println!("loading expression file `{}` failed with reason: {:?}", feel_file_name, reason),
  }
}

/// Tests `FEEL` expression loaded from file and prints the test result to standard output.
fn test_feel_expression(test_file_name: &str, feel_file_name: &str, summary_only: bool) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(feel_file_content) => match std::fs::read_to_string(test_file_name) {
      Ok(test_file_content) => match dmntk_evaluator::evaluate_test_cases(&test_file_content) {
        Ok(test_cases) => {
          let mut passed = 0_usize;
          let mut failed = 0_usize;
          for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
            let scope = input_data.clone().into();
            match dmntk_feel_parser::parse_expression(&scope, &feel_file_content, false) {
              Ok(node) => match dmntk_evaluator::evaluate(&scope, &node) {
                Ok(actual) => display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only),
                Err(reason) => println!("evaluating expression failed with reason: {}", reason),
              },
              Err(reason) => println!("parsing expression failed with reason: {}", reason),
            }
          }
          display_test_summary(passed, failed, summary_only);
        }
        Err(reason) => println!("evaluation of test cases failed with reason: {}", reason),
      },
      Err(reason) => println!("loading test file `{}` failed with reason: {:?}", test_file_name, reason),
    },
    Err(reason) => println!("loading expression file `{}` failed with reason: {:?}", feel_file_name, reason),
  }
}

/// Exports `FEEL` expression loaded from file to HTML output file.
fn export_feel_expression(_feel_file_name: &str, _html_file_name: &str) {
  println!("xfe command is not implemented yet")
}

/// Parses decision table loaded from text file.
fn parse_decision_table(dectab_file_name: &str) {
  match std::fs::read_to_string(dectab_file_name) {
    Ok(text) => match dmntk_recognizer::scan(&text) {
      Ok(mut canvas) => {
        canvas.display_text_layer();
        canvas.display_thin_layer();
        canvas.display_body_layer();
        canvas.display_grid_layer();
        match canvas.plane() {
          Ok(plane) => println!("PLANE\n{}", plane),
          Err(reason) => println!("ERROR: {}", reason),
        };
      }
      Err(reason) => println!("ERROR: {}", reason),
    },
    Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dectab_file_name, reason),
  }
}

/// Evaluates context and decision table loaded from files.
fn evaluate_decision_table(input_file_name: &str, dectab_file_name: &str) {
  let input_file_content = match std::fs::read_to_string(input_file_name) {
    Ok(input_file_content) => input_file_content,
    Err(reason) => {
      println!("loading input file `{}` failed with reason: {}", input_file_name, reason);
      return;
    }
  };
  let input_data = match dmntk_evaluator::evaluate_context(&Scope::default(), &input_file_content) {
    Ok(input_data) => input_data,
    Err(reason) => {
      println!("evaluating input data failed with reason: {}", reason);
      return;
    }
  };
  let dtb_file_content = match std::fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      println!("loading input file `{}` failed with reason: {}", dectab_file_name, reason);
      return;
    }
  };
  let decision_table = match dmntk_recognizer::build(&dtb_file_content) {
    Ok(decision_table) => decision_table,
    Err(reason) => {
      println!("building decision table failed with reason: {}", reason);
      return;
    }
  };
  let scope = input_data.into();
  let evaluator = match dmntk_evaluator::build_decision_table_evaluator(&scope, &decision_table) {
    Ok(evaluator) => evaluator,
    Err(reason) => {
      println!("building decision table evaluator failed with reason: {}", reason);
      return;
    }
  };
  let result = evaluator(&scope) as Value;
  println!("{}", result.jsonify());
}

/// Tests decision table loaded from file.
fn test_decision_table(test_file_name: &str, dectab_file_name: &str, summary_only: bool) {
  let dtb_file_content = match std::fs::read_to_string(dectab_file_name) {
    Ok(dtb_file_content) => dtb_file_content,
    Err(reason) => {
      println!("loading decision table file `{}` failed with reason: {}", dectab_file_name, reason);
      return;
    }
  };
  let decision_table = match dmntk_recognizer::build(&dtb_file_content) {
    Ok(decision_table) => decision_table,
    Err(reason) => {
      println!("building decision table failed with reason: {}", reason);
      return;
    }
  };
  let test_file_content = match std::fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      println!("loading test file `{}` failed with reason: {}", test_file_name, reason);
      return;
    }
  };
  let test_cases = match dmntk_evaluator::evaluate_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      println!("evaluating test file failed with reason: {}", reason);
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let scope = input_data.clone().into();
    let evaluator = match dmntk_evaluator::build_decision_table_evaluator(&scope, &decision_table) {
      Ok(evaluator) => evaluator,
      Err(reason) => {
        println!("building decision table evaluator failed with reason: {}", reason);
        return;
      }
    };
    let actual = evaluator(&scope) as Value;
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only);
  }
  display_test_summary(passed, failed, summary_only);
}

/// Exports decision table loaded from text file to HTML output file.
fn export_decision_table(_dectab_file_name: &str, _html_file_name: &str) {
  println!("xdt command is not implemented yet")
}

/// Recognizes the decision table loaded from text file.
fn recognize_decision_table(dtb_file_name: &str) {
  match std::fs::read_to_string(dtb_file_name) {
    Ok(ref text) => match dmntk_recognizer::Recognizer::recognize(text) {
      Ok(recognizer) => {
        recognizer.trace();
      }
      Err(reason) => println!("ERROR: {}", reason),
    },
    Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
  }
}

/// Parses `DMN` model loaded from XML file.
fn parse_dmn_model(dmn_file_name: &str) {
  let c_a = ascii256!(255);
  let c_b = ascii256!(82);
  let c_c = ascii256!(184);
  let c_d = ascii256!(208);
  let none = "(none)".to_string();
  match std::fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match dmntk_model::parse(&dmn_file_content) {
      Ok(definitions) => {
        println!("\n{}Model{}", c_a, ASCII_RESET);
        println!("{} ├─ name:{} {}{}", c_a, c_b, definitions.name(), ASCII_RESET);
        println!("{} ├─ namespace:{} {}{}", c_a, c_b, definitions.namespace(), ASCII_RESET);
        println!("{} └─ id:{} {}{}", c_a, c_b, definitions.id().as_ref().unwrap_or(&none), ASCII_RESET);
        // definitions
        if definitions.decisions().is_empty() {
          println!("\n{}Decisions{} {}{}", c_a, c_c, none, ASCII_RESET);
        } else {
          println!("\n{}Decisions{}", c_a, ASCII_RESET);
          let decision_count = definitions.decisions().len();
          for (i, decision) in definitions.decisions().iter().enumerate() {
            if i < decision_count - 1 {
              print!(" {}├─{}", c_a, ASCII_RESET);
            } else {
              print!(" {}└─{}", c_a, ASCII_RESET);
            }
            println!(" {}{}{}", c_c, decision.name(), ASCII_RESET);
          }
        }
        // item data
        if definitions.input_data().is_empty() {
          println!("\n{}Input data{} {}{}", c_a, c_c, none, ASCII_RESET);
        } else {
          println!("\n{}Input data{}", c_a, ASCII_RESET);
          let input_data_count = definitions.input_data().len();
          for (i, input_data) in definitions.input_data().iter().enumerate() {
            if i < input_data_count - 1 {
              print!(" {}├─{}", c_a, ASCII_RESET);
            } else {
              print!(" {}└─{}", c_a, ASCII_RESET);
            }
            println!(
              " {}{} ({}){}",
              c_d,
              input_data.name(),
              input_data.variable().type_ref().as_ref().unwrap_or(&none),
              ASCII_RESET
            );
          }
        }
        // more...
        print!("\n{}MORE DETAILS WILL BE IMPLEMENTED...{}\n\n", ASCII_RED, ASCII_RESET);
      }
      Err(reason) => println!("parsing model file failed with reason: {}", reason),
    },
    Err(reason) => println!("loading model file `{}` failed with reason: {:?}", dmn_file_name, reason),
  }
}

/// Evaluates `DMN` model loaded from XML file.
fn evaluate_dmn_model(input_file_name: &str, dmn_file_name: &str, invocable_name: &str) {
  match std::fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => match std::fs::read_to_string(input_file_name) {
      Ok(input_file_content) => match dmntk_evaluator::evaluate_context(&Scope::default(), &input_file_content) {
        Ok(input_data) => match dmntk_model::parse(&dmn_file_content) {
          Ok(definitions) => match dmntk_evaluator::ModelEvaluator::new(&definitions) {
            Ok(model_evaluator) => {
              let result = model_evaluator.evaluate_invocable(invocable_name, &input_data);
              println!("{}", result.jsonify())
            }
            Err(reason) => println!("evaluating invocable {} failed with reason: {}", invocable_name, reason),
          },
          Err(reason) => println!("parsing model failed with reason: {}", reason),
        },
        Err(reason) => println!("evaluating input data failed with reason: {}", reason),
      },
      Err(reason) => println!("loading input data file `{}` failed with reason: {:?}", input_file_name, reason),
    },
    Err(reason) => println!("loading model file `{}` failed with reason: {:?}", dmn_file_name, reason),
  }
}

/// Tests `DMN` model loaded from XML file.
fn test_dmn_model(test_file_name: &str, dmn_file_name: &str, invocable_name: &str, summary_only: bool) {
  let dmn_file_content = match std::fs::read_to_string(dmn_file_name) {
    Ok(dmn_file_content) => dmn_file_content,
    Err(reason) => {
      println!("loading model file `{}` failed with reason: {}", dmn_file_name, reason);
      return;
    }
  };
  let definitions = match dmntk_model::parse(&dmn_file_content) {
    Ok(definitions) => definitions,
    Err(reason) => {
      println!("parsing model file failed with reason: {}", reason);
      return;
    }
  };
  let model_evaluator = match dmntk_evaluator::ModelEvaluator::new(&definitions) {
    Ok(model_evaluator) => model_evaluator,
    Err(reason) => {
      println!("preparing model evaluator failed with reason: {}", reason);
      return;
    }
  };
  let test_file_content = match std::fs::read_to_string(test_file_name) {
    Ok(test_file_content) => test_file_content,
    Err(reason) => {
      println!("loading test file `{}` failed with reason: {}", test_file_name, reason);
      return;
    }
  };
  let test_cases = match dmntk_evaluator::evaluate_test_cases(&test_file_content) {
    Ok(test_cases) => test_cases,
    Err(reason) => {
      println!("evaluating test file failed with reason: {}", reason);
      return;
    }
  };
  let mut passed = 0_usize;
  let mut failed = 0_usize;
  for (test_no, (input_data, expected)) in test_cases.iter().enumerate() {
    let actual = model_evaluator.evaluate_invocable(invocable_name, input_data);
    display_test_case_result(&actual, expected, &test_no, &mut passed, &mut failed, summary_only);
  }
  display_test_summary(passed, failed, summary_only);
}

/// Exports `DMN` model loaded from XML file to HTML output file.
fn export_dmn_model(_dmn_file_name: &str, _html_file_name: &str) {
  println!("xdm command is not implemented yet")
}

/// Generates examples in current directory.
fn generate_examples() {
  let create_dir = |path| {
    std::fs::create_dir_all(path).unwrap_or_else(|_| panic!("creating '{}' directory failed", path));
  };
  let write_file = |path, contents| {
    std::fs::write(path, contents).unwrap_or_else(|_| panic!("saving example file '{}' failed", path));
  };
  create_dir("examples");
  create_dir("examples/e1");
  write_file("./examples/e1/e1.ctx", E1_CTX);
  write_file("./examples/e1/e1.feel", E1_FEEL);
  create_dir("examples/e2");
  write_file("./examples/e2/e2.ctx", E2_CTX);
  write_file("./examples/e2/e2.dmn", E2_DMN);
  create_dir("examples/e3");
  write_file("./examples/e3/e3.ctx", E3_CTX);
  write_file("./examples/e3/e3.dtb", E3_DTB);
}

/// Utility function for displaying test case result.
fn display_test_case_result(actual: &Value, expected: &Value, test_no: &usize, passed: &mut usize, failed: &mut usize, summary_only: bool) {
  if dmntk_evaluator::evaluate_equals(actual, expected) {
    *passed += 1;
    if !summary_only {
      println!("test {} ... {}ok{}", test_no + 1, ASCII_GREEN, ASCII_RESET);
    }
  } else {
    *failed += 1;
    if !summary_only {
      println!("test {} ... {}FAILED{}", test_no + 1, ASCII_RED, ASCII_RESET);
      println!("  expected: {}", expected);
      println!("    actual: {}", actual);
    }
  }
}

/// Utility function for displaying test summary.
fn display_test_summary(passed: usize, failed: usize, summary_only: bool) {
  if failed > 0 {
    if summary_only {
      println!("test result: FAILED. {} passed; {} failed.", passed, failed);
    } else {
      println!("\ntest result: {}FAILED{}. {} passed; {} failed.\n", ASCII_RED, ASCII_RESET, passed, failed);
    }
  } else if summary_only {
    println!("test result: ok. {} passed; {} failed.", passed, failed);
  } else {
    println!("\ntest result: {}ok{}. {} passed; {} failed.\n", ASCII_GREEN, ASCII_RESET, passed, failed);
  }
}
