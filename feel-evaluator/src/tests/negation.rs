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
fn test_negation() {
  let scope = &te_scope("{}");
  te_number(false, scope, "-0", 0, 0);
  te_number(false, scope, "-0", -0, 0);
  te_number(false, scope, "0", 0, 0);
  te_number(false, scope, "--0", 0, 0);
  te_number(false, scope, "-1", -1, 0);
  te_number(false, scope, "--1", 1, 0);
  te_number(false, scope, " -10.2 ", -102, 1);
  te_number(false, scope, " - 10 ", -10, 0);
  te_number(false, scope, "(-20)", -20, 0);
  te_number(false, scope, " ( -21 ) ", -21, 0);
  te_number(false, scope, " ( - 22 ) ", -22, 0);
  te_number(false, scope, " ((( - 23 ))) ", -23, 0);
  te_number(false, scope, " - - 24 ", 24, 0);
  te_number(false, scope, " - ( - 25 ) ", 25, 0);
}

#[test]
fn _0100() {
  let scope = &te_scope("{}");
  te_days_and_time_duration(false, scope, r#"duration("-PT1H")"#, true, 3600, 0)
}
