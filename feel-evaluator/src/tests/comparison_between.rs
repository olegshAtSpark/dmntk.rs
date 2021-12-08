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
fn test_0001() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "2 between 1 and 4", true);
}

#[test]
fn test_0002() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "1 between 1 and 4", true);
}

#[test]
fn test_0003() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "4 between 1 and 4", true);
}

#[test]
fn test_0004() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "0.99 between 1 and 4", false);
}

#[test]
fn test_0005() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "4.01 between 1 and 4", false);
}

#[test]
fn test_0006() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "6 between 1 and 4 + 2", true);
}

#[test]
fn test_0007() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "2 between 1 + 1 and 4 + 2", true);
}

#[test]
fn test_0008() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "5 - 2 between 1 + 2 and 10.2/2", true);
}

#[test]
fn test_0009() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "5 - 2 - 0.1 between 1 + 2 and 10.2/2", false);
}

#[test]
fn test_0010() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "true = (2 between 1 and 4)", true);
}

#[test]
fn test_0011() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "(2 between 1 and 4) = true", true);
}

#[test]
fn test_0012() {
  let scope = &te_scope("{}");
  te_bool(false, scope, "(2 between 1 and 4) = (5 between 3 and 8)", true);
}
