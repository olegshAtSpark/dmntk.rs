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

use std::fs;
use std::fs::File;
use std::io::Write;

/// Name of the target directory.
const TARGET_DIR: &str = "../target/gendoc";

#[test]
fn test_2_0001_html() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_2_0001, "file://2_0001.dmn").expect("parsing model 2_0001.dmn failed");
  let html = crate::generate(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);
  fs::create_dir_all(TARGET_DIR).expect("creating target directories failed");
  let mut file = File::create(format!("{}/2_0001.html", TARGET_DIR)).expect("creating file 2_0001.html failed");
  file.write_all(html.as_bytes()).expect("saving file 2_0001.html failed");
}

#[test]
fn test_3_0087_html() {
  let definitions = dmntk_model::parse(dmntk_examples::DMN_3_0087, "file://3_0087.dmn").expect("parsing model 3_0087.dmn failed");
  let html = crate::generate(&definitions);
  assert_eq!("<!DOCTYPE html>", &html[0..15]);
  fs::create_dir_all(TARGET_DIR).expect("creating target directories failed");
  let mut file = File::create(format!("{}/3_0087.html", TARGET_DIR)).expect("creating file 3_0087.html failed");
  file.write_all(html.as_bytes()).expect("saving file 2_0001.html failed");
}
