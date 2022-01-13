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

//! Point.

/// Point with coordinates set to `(0,0)`.
pub const POINT_ZERO: Point = Point { x: 0, y: 0 };

/// Vector of points.
pub type Points = Vec<Point>;

/// Point.
#[derive(Copy, Clone, PartialEq)]
pub struct Point {
  pub x: usize,
  pub y: usize,
}

impl Point {
  /// Creates a new point with specified coordinates.
  pub fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }
  /// Compares the coordinates of two points and returns **true**
  /// when both coordinates are pointing the same position.
  /// Another words, the point **p** overlays this point.
  pub fn overlays(&self, p: Point) -> bool {
    self.x == p.x && self.y == p.y
  }
  /// Unwraps the point coordinates to tuple with two integers.
  pub fn unwrap(&self) -> (usize, usize) {
    (self.x, self.y)
  }
}

/// Implementation of Display trait for point.
impl std::fmt::Display for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}

/// Implementation of Debug trait for point.
impl std::fmt::Debug for Point {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({},{})", self.x, self.y)
  }
}
