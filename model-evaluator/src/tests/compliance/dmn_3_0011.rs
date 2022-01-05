/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * DMN model evaluator
 *
 * Copyright 2018-2022 Dariusz Depta Engos Software <dariusz.depta@engos.software>
 *
 * THE SOFTWARE IS PROVIDED "AS IS",  WITHOUT WARRANTY OF ANY KIND,  EXPRESS OR
 * IMPLIED,  INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM,  DAMAGES OR OTHER
 * LIABILITY,  WHETHER IN AN ACTION OF CONTRACT,  TORT OR OTHERWISE,  ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

use crate::tests::{assert_decision, context};

lazy_static! {
  static ref DEFINITIONS: dmntk_model::model::Definitions = dmntk_model::parse(dmntk_examples::DMN_3_0011).unwrap();
}

#[test]
fn _0001() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&DEFINITIONS, "literalNestedList", &ctx, r#"[["a", "b"], ["b", "c"]]"#);
}

#[test]
fn _0002() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&DEFINITIONS, "remove1", &ctx, r#"["a", "c"]"#);
}

#[test]
fn _0003() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&DEFINITIONS, "insert3", &ctx, r#"[["o"], ["a", "b", "c"], ["p", "q"]]"#);
}

#[test]
fn _0004() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&DEFINITIONS, "insert2", &ctx, r#"[["a", "b"], ["a", "b", "c"], ["b", "c"]]"#);
}

#[test]
fn _0005() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&DEFINITIONS, "remove2", &ctx, r#"[["a", "b"]]"#);
}

#[test]
fn _0006() {
  let ctx = context(r#"{nestedList:  [["o"], ["p", "q"]], position:  2, simpleList:  ["a", "b", "c"]}"#);
  assert_decision(&DEFINITIONS, "insert1", &ctx, r#"["a", "x", "b", "c"]"#);
}
