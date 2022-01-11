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

use crate::{DMNTK_DESCRIPTION, DMNTK_VERSION};
use clap::{load_yaml, App, AppSettings};
use dmntk_feel::Scope;

/// Available command-line actions.
enum Action {
  /// Parse `FEEL` expression.
  ParseFeelExpression(String, String),
  /// Evaluate `FEEL` expression.
  EvaluateFeelExpression(String, String),
  /// Test `FEEL` expression.
  TestFeelExpression(String, String),
  /// Export `FEEL` expression to HTML.
  ExportFeelExpression(String, String),
  /// Parse decision table.
  ParseDecisionTable(String),
  /// Evaluate decision table.
  EvaluateDecisionTable(String, String),
  /// Test decision table.
  TestDecisionTable(String, String),
  /// Export decision table.
  ExportDecisionTable(String, String),
  /// Parse `DMN` model`.
  ParseDmnModel(String),
  /// Evaluate `DMN` model`.
  EvaluateDmnModel(String, String),
  /// Test `DMN` model`.
  TestDmnModel(String, String),
  /// Export `DMN` model`.
  ExportDmnModel(String, String),
  /// Start **dmntk** as a service.
  StartService(Option<String>, Option<String>),
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
    Action::TestFeelExpression(test_file_name, feel_file_name) => {
      test_feel_expression(&test_file_name, &feel_file_name);
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
    Action::EvaluateDecisionTable(dectab_file_name, ctx_file_name) => {
      evaluate_decision_table(&dectab_file_name, &ctx_file_name);
      Ok(())
    }
    Action::TestDecisionTable(test_file_name, dectab_file_name) => {
      test_decision_table(&test_file_name, &dectab_file_name);
      Ok(())
    }
    Action::ExportDecisionTable(dectab_file_name, html_file_name) => {
      export_decision_table(&dectab_file_name, &html_file_name);
      Ok(())
    }
    Action::ParseDmnModel(dectab_file_name) => {
      parse_dmn_model(&dectab_file_name);
      Ok(())
    }
    Action::EvaluateDmnModel(dectab_file_name, ctx_file_name) => {
      evaluate_dmn_model(&dectab_file_name, &ctx_file_name);
      Ok(())
    }
    Action::TestDmnModel(test_file_name, dectab_file_name) => {
      test_dmn_model(&test_file_name, &dectab_file_name);
      Ok(())
    }
    Action::ExportDmnModel(dectab_file_name, html_file_name) => {
      export_dmn_model(&dectab_file_name, &html_file_name);
      Ok(())
    }
    Action::StartService(opt_host, opt_port) => dmntk_server::start_server(opt_host, opt_port).await,
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
    );
  }
  // export decision table subcommand
  if let Some(matches) = matches.subcommand_matches("xdt") {
    return Action::ExportDecisionTable(
      matches.value_of("DECTAB_FILE").unwrap_or("unknown.dtb").to_string(),
      matches.value_of("HTML_FILE").unwrap_or("unknown.html").to_string(),
    );
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
    );
  }
  // test DMN model subcommand
  if let Some(matches) = matches.subcommand_matches("tdm") {
    return Action::TestDmnModel(
      matches.value_of("TEST_FILE").unwrap_or("unknown.ctx").to_string(),
      matches.value_of("DMN_FILE").unwrap_or("unknown.dmn").to_string(),
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
    );
  }
  Action::DoNothing
}

/// Recognizes the decision table loaded from text file.
fn _recognize_decision_table_from_file(dtb_file_name: &str) {
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
fn test_feel_expression(_test_file_name: &str, _feel_file_name: &str) {
  println!("tfe command is not implemented yet")
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
fn evaluate_decision_table(dtb_file_name: &str, ctx_file_name: &str) {
  // read the context input from file
  match std::fs::read_to_string(ctx_file_name) {
    Ok(ref context_input) => {
      // read decision table input from file
      match std::fs::read_to_string(dtb_file_name) {
        Ok(ref decision_table_input) => match dmntk_evaluator::evaluate_decision_table_and_context(decision_table_input, context_input) {
          Ok(value) => {
            println!("{}", value)
          }
          Err(reason) => println!("{}", reason),
        },
        Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
      }
    }
    Err(reason) => println!("loading context file `{}` failed with reason: {}", ctx_file_name, reason),
  }
}

/// Tests decision table loaded from file.
fn test_decision_table(_test_file_name: &str, dtb_file_name: &str) {
  match std::fs::read_to_string(dtb_file_name) {
    Ok(dtb_input) => match dmntk_evaluator::evaluate_decision_table_and_test(&dtb_input, "%") {
      Ok((result, expected, actual)) => {
        if !result {
          println!("FAILURE");
          println!("Expected: {}", expected);
          println!("  Actual: {}", actual);
        } else {
          println!("SUCCESS!");
        }
      }
      Err(reason) => println!("{:?}", reason),
    },
    Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
  }
}

/// Exports decision table loaded from text file to HTML output file.
fn export_decision_table(_dectab_file_name: &str, _html_file_name: &str) {
  println!("xdt command is not implemented yet")
}

/// Parses `DMN` model loaded from XML file.
fn parse_dmn_model(_dmn_file_name: &str) {
  println!("pdm command is not implemented yet")
}

/// Evaluates `DMN` model loaded from XML file.
fn evaluate_dmn_model(_input_file_name: &str, _dmn_file_name: &str) {
  println!("edm command is not implemented yet")
}

/// Tests `DMN` model loaded from XML file.
fn test_dmn_model(_test_file_name: &str, _dmn_file_name: &str) {
  println!("tdm command is not implemented yet")
}

/// Exports `DMN` model loaded from XML file to HTML output file.
fn export_dmn_model(_dmn_file_name: &str, _html_file_name: &str) {
  println!("xdm command is not implemented yet")
}
