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

use crate::{DMNTK_DESCRIPTION, DMNTK_VERSION};
use clap::{load_yaml, App, AppSettings};
use dmntk_feel::Scope;

/// Available command-line actions.
enum Action {
  /// Start decision table service in server mode.
  StartServer(Option<String>, Option<String>),
  /// Recognize decision table loaded from text file.
  //RecognizeDecisionTable(String),
  /// Parse decision table loaded from text file.
  //ParseDecisionTable(String),
  /// Evaluate decision table loaded from `DTB` and `CTX` files.
  //EvaluateDecisionTable(String, String),
  /// Test decision table loaded from `DTB` file.
  //TestDecisionTable(String),
  /// Parse `FEEL` expression loaded from text file.
  ParseFeelExpression(String, String),
  /// Evaluate `FEEL` expression loaded from text file.
  EvaluateFeelExpression(String, String),
  /// Do nothing, no action was specified.
  DoNothing,
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
  if let Some(matches) = matches.subcommand_matches("pfl") {
    let context_file_name = matches.value_of("CONTEXT_FILE").unwrap();
    let expression_file_name = matches.value_of("EXPRESSION_FILE").unwrap();
    return Action::ParseFeelExpression(context_file_name.to_string(), expression_file_name.to_string());
  }
  // evaluate FEEL expression subcommand
  if let Some(matches) = matches.subcommand_matches("efl") {
    let context_file_name = matches.value_of("CONTEXT_FILE").unwrap();
    let expression_file_name = matches.value_of("EXPRESSION_FILE").unwrap();
    return Action::EvaluateFeelExpression(context_file_name.to_string(), expression_file_name.to_string());
  }
  // start server subcommand
  if let Some(matches) = matches.subcommand_matches("run") {
    let host = matches.value_of("host");
    let port = matches.value_of("port");
    return Action::StartServer(host.map(|h| h.to_string()), port.map(|p| p.to_string()));
  }
  Action::DoNothing
}

/// Parses decision table loaded from file.
fn _parse_decision_table(dtb_file_name: &str) {
  match std::fs::read_to_string(dtb_file_name) {
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
    Err(reason) => println!("loading decision table file `{}` failed with reason: {}", dtb_file_name, reason),
  }
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

/// Evaluates context and decision table loaded from files.
fn _evaluate_decision_table_from_file(dtb_file_name: String, ctx_file_name: String) {
  // read the context input from file
  match std::fs::read_to_string(ctx_file_name.as_str()) {
    Ok(ref context_input) => {
      // read decision table input from file
      match std::fs::read_to_string(dtb_file_name.as_str()) {
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
fn _test_decision_table_from_file(dtb_file_name: String) {
  match std::fs::read_to_string(dtb_file_name.as_str()) {
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

/// Parses `FEEL` textual expression loaded from file and prints the parsed AST to standard output.
fn parse_textual_expression_from_file(ctx_file_name: &str, feel_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(feel_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&Scope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_expression(&ctx.into(), &feel_expression, false) {
          Ok(ast_root_node) => {
            println!("    AST:{}", ast_root_node.to_string().trim_end());
          }
          Err(reason) => println!("parsing textual expression failed with reason: {}", reason),
        },
        Err(reason) => println!("evaluating context failed with reason: {}", reason),
      },
      Err(reason) => println!("loading context file `{}` failed with reason: {:?}", ctx_file_name, reason),
    },
    Err(reason) => println!("loading textual expression file `{}` failed with reason: {:?}", feel_file_name, reason),
  }
}

/// Evaluates `FEEL` textual expression loaded from file and prints the result to standard output.
fn evaluate_textual_expression_from_file(ctx_file_name: &str, feel_file_name: &str) {
  match std::fs::read_to_string(feel_file_name) {
    Ok(textual_expression) => match std::fs::read_to_string(ctx_file_name) {
      Ok(context_definition) => match dmntk_evaluator::evaluate_context(&Scope::default(), &context_definition) {
        Ok(ctx) => match dmntk_feel_parser::parse_expression(&ctx.clone().into(), &textual_expression, false) {
          Ok(ast_root_node) => match dmntk_evaluator::evaluate(&ctx.into(), &ast_root_node) {
            Ok(result) => {
              println!("{}", result);
            }
            Err(reason) => println!("evaluating textual expression failed with reason: {}", reason),
          },
          Err(reason) => println!("parsing textual expression failed with reason: {}", reason),
        },
        Err(reason) => println!("evaluating context failed with reason: {}", reason),
      },
      Err(reason) => println!("loading context file `{}` failed with reason: {:?}", ctx_file_name, reason),
    },
    Err(reason) => println!("loading textual expression file `{}` failed with reason: {:?}", feel_file_name, reason),
  }
}

pub async fn do_action() -> std::io::Result<()> {
  match get_cli_action() {
    // Action::ParseDecisionTable(dtb_file_name) => {
    //   parse_decision_table(&dtb_file_name);
    //   Ok(())
    // }
    // Action::RecognizeDecisionTable(dtb_file_name) => {
    //   recognize_decision_table_from_file(&dtb_file_name);
    //   Ok(())
    // }
    // Action::EvaluateDecisionTable(dtb_file_name, ctx_file_name) => {
    //   evaluate_decision_table_from_file(dtb_file_name, ctx_file_name);
    //   Ok(())
    // }
    // Action::TestDecisionTable(dtb_file_name) => {
    //   test_decision_table_from_file(dtb_file_name);
    //   Ok(())
    // }
    Action::ParseFeelExpression(ctx_file_name, feel_file_name) => {
      parse_textual_expression_from_file(&ctx_file_name, &feel_file_name);
      Ok(())
    }
    Action::EvaluateFeelExpression(ctx_file_name, feel_file_name) => {
      evaluate_textual_expression_from_file(&ctx_file_name, &feel_file_name);
      Ok(())
    }
    Action::StartServer(opt_host, opt_port) => crate::server::start_server(opt_host, opt_port).await,
    Action::DoNothing => Ok(()),
  }
}
