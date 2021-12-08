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

use super::super::point::*;

#[test]
fn point_zero() {
  let point = POINT_ZERO;
  assert_eq!(point.x, 0);
  assert_eq!(point.y, 0);
}

#[test]
fn point_new() {
  let point = Point::new(1, 2);
  assert_eq!(point.x, 1);
  assert_eq!(point.y, 2);
}

#[test]
fn point_overlays() {
  let p1 = Point::new(1, 2);
  let p2 = Point::new(1, 2);
  let p3 = Point::new(2, 1);
  assert!(p2.overlays(p1));
  assert!(!p3.overlays(p2));
}

#[test]
fn point_display() {
  assert_eq!(format!("{}", Point::new(10, 20)), "(10,20)");
  assert_eq!(format!("{}", Point::new(0, 0)), "(0,0)");
}

#[test]
fn point_debug() {
  assert_eq!(format!("{:?}", Point::new(10, 20)), "(10,20)");
  assert_eq!(format!("{:?}", Point::new(0, 0)), "(0,0)");
}
