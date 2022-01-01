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

use super::*;

#[test]
fn _0001() {
  let scope = &te_scope("{}");
  te_none(false, scope, "2 in ()");
}

#[test]
fn _0002() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "2 in []", false);
}

#[test]
fn test_comparison_in() {
  let scope = &te_scope("{ a: 100.0, b: 99.0, c: 101.0}");
  te_none(false, scope, "2 in ()");
  te_bool(false, scope, "2 in []", false);
  te_bool(false, scope, "2 in [1..5]", true);
  te_bool(false, scope, "99 in <=100", true);
  te_bool(false, scope, "(b) in <=100", true);
  te_bool(false, scope, "b in <=100", true);
  te_bool(false, scope, "100 in <=100", true);
  te_bool(false, scope, "(a) in <=100", true);
  te_bool(false, scope, "a in <=100", true);
  te_bool(false, scope, "101 in <=100", false);
  te_bool(false, scope, "99 in <100", true);
  te_bool(false, scope, "(b) in <100", true);
  te_bool(false, scope, "b in <100", true);
  te_bool(false, scope, "100 in <100", false);
  te_bool(false, scope, "(a) in <100", false);
  te_bool(false, scope, "101 in >=100", true);
  te_bool(false, scope, "100 in >=100", true);
  te_bool(false, scope, "(a) in >=100", true);
  te_bool(false, scope, "99 in >=100", false);
  te_bool(false, scope, "(b) in >=100", false);
  te_bool(false, scope, "b in >=100", false);
  te_bool(false, scope, "101 in >100", true);
  te_bool(false, scope, "100 in >100", false);
  te_bool(false, scope, "(a) in >100", false);
  te_bool(false, scope, "a in >100", false);
  te_bool(false, scope, "2 in (2)", true);
  te_bool(false, scope, "2 in (3)", false);
  te_bool(false, scope, "2 in (1,2,3,4,5)", true);
  te_bool(false, scope, "7 in (1,2,3,4,5)", false);
  te_bool(false, scope, "(a) in (1,2,3,4,5)", false);
  te_bool(false, scope, "2 in (<3)", true);
  te_bool(false, scope, "6 in (>5)", true);
  te_bool(false, scope, "2 in (<3,>5)", true);
  te_bool(false, scope, "3 in (<3,>5)", false);
  te_bool(false, scope, "4.12 in (<3,>5)", false);
  te_bool(false, scope, "5 in (<3,>5)", false);
  te_bool(false, scope, "2 in (>5,<3)", true);
  te_bool(false, scope, "5 in (>5,<3)", false);
  te_bool(false, scope, "4.5 in (>5,<3)", false);
  te_bool(false, scope, "3 in (>5,<3)", false);
  te_bool(false, scope, "2 in (<=3)", true);
  te_bool(false, scope, "2 in (<=3,>=5)", true);
  te_bool(false, scope, "3 in (<=3,>=5)", true);
  te_bool(false, scope, "5 in (<=3,>=5)", true);
  te_bool(false, scope, "4 in (<=3,>=5)", false);
  te_bool(false, scope, "2 in (>=5,<=3)", true);
  te_bool(false, scope, "3 in (>=5,<=3)", true);
  te_bool(false, scope, "5 in (>=5,<=3)", true);
  te_bool(false, scope, "4 in (>=5,<=3)", false);
  te_bool(false, scope, "not(4 in (1,3))", true);
  te_bool(false, scope, "not(5.25 in (1.32,2.45,4.12,5.25))", false);
  te_bool(false, scope, "5 in (<=5)", true);
  te_bool(false, scope, "5 in ((5..10])", false);
  te_bool(false, scope, "5 in ([5..10])", true);
  te_bool(false, scope, "5 in (4,5,6)", true);
  te_bool(false, scope, "5 in (<5,>5)", false);
  te_bool(false, scope, "1 in [2,3,1]", true);
  te_bool(false, scope, r#""k" in ["j".."l"]"#, true);
  te_bool(false, scope, r#""b" in [["f".."h"], ["a".."c"]]"#, true);
  te_bool(false, scope, r#""a" in <= "b""#, true);
  te_bool(false, scope, r#"true in [false, 2, 3]"#, false);
  te_bool(false, scope, r#"true in true"#, true);
  te_bool(
    false,
    scope,
    r#"date("2018-12-08") in [date("2018-12-08"),date("2018-12-09"),date("2018-12-10")]"#,
    true,
  );
  te_bool(false, scope, r#"date("2018-12-04") in <= date("2018-12-05")"#, true);
  te_bool(false, scope, r#"[1,2,3] in [[1,2,3,4], [1,2,3]]"#, true);
  te_bool(false, scope, r#"[1,2,2] in [[1,2,3,4], [1,2,3]]"#, false);
  te_bool(false, scope, r#"{a: "foo"} in [{b: "bar"}, {a: "foo"}]"#, true);
  te_bool(false, scope, r#"duration("P11Y") in [duration("P8Y"),duration("P9Y"),duration("P10Y")]"#, false);
  te_bool(
    false,
    scope,
    r#"duration("P11Y") in [[duration("P5Y") .. duration("P7Y")], [duration("P10Y") .. duration("P12Y")]]"#,
    true,
  );
  te_bool(false, scope, r#"duration("P11Y") in > duration("P10Y")"#, true);
}
