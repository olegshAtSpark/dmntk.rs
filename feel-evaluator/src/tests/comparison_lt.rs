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

use super::*;

#[test]
fn test_less_than() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "1<2", true);
  te_bool(false, scope, "1.276<1.277", true);
  te_bool(false, scope, "-.5<.54635", true);
  te_bool(false, scope, "2<1", false);
  te_bool(false, scope, "(1+1)<2.01", true);
  te_bool(false, scope, "(1.1+2)<3.0", false);
  te_bool(false, scope, " ( 1 + 0.99 ) < 2.0", true);
  te_bool(false, scope, " ( ( ( 1.1 + 3.1 ) ) ) < 2.5", false);
  te_bool(false, scope, "(0.9+1)<(5.1-3)", true);
  te_bool(false, scope, "(1*2)<(10.0/4.9)", true);
}

#[test]
fn _0011() {
  let scope = &te_scope(r#"{}"#);
  te_bool(false, scope, r#"@"2021-11-10" < @"2021-11-11""#, true);
}

#[test]
fn _0012() {
  let scope = &te_scope(r#"{}"#);
  te_bool(false, scope, r#"@"2021-11-10" < @"2021-11-10""#, false);
}

#[test]
fn _0013() {
  let scope = &te_scope(r#"{}"#);
  te_bool(false, scope, r#"@"2021-11-10" < @"2021-11-09""#, false);
}
