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

//! Rectangle.

/// Rectangle with coordinates set to zeros `(0,0,0,0)`.
pub const RECT_ZERO: Rect = Rect {
  top: 0,
  right: 0,
  bottom: 0,
  left: 0,
};

/// Vector of rectangles.
pub type Rectangles = Vec<Rect>;

/// Rectangle.
#[derive(Copy, Clone, PartialEq)]
pub struct Rect {
  /// Left edge coordinate (inclusive).
  pub left: usize,
  /// Top edge coordinate (inclusive).
  pub top: usize,
  /// Right edge coordinate (exclusive).
  pub right: usize,
  /// Bottom edge coordinate (exclusive).
  pub bottom: usize,
}

impl Rect {
  /// Creates a new rectangle from four coordinates.
  pub fn new(left: usize, top: usize, right: usize, bottom: usize) -> Rect {
    Rect { left, top, right, bottom }
  }

  /// Return a copy of the rectangle with top value incremented with offset.
  pub fn inc_top(&self, offset: usize) -> Rect {
    Rect {
      left: self.left,
      top: self.top + offset,
      right: self.right,
      bottom: self.bottom,
    }
  }

  /// Unpacks the rectangle coordinates to a tuple of four integers.
  pub fn unpack(&self) -> (usize, usize, usize, usize) {
    (self.left, self.top, self.right, self.bottom)
  }

  /// Checks if the specified rectangle is contained in this rectangle.
  pub fn contains(&self, r: &Rect) -> bool {
    r.left >= self.left && r.top >= self.top && r.right <= self.right && r.bottom <= self.bottom
  }

  /// Returns the width of the rectangle.
  pub fn width(&self) -> usize {
    self.right - self.left
  }

  /// Returns the height of the rectangle.
  pub fn height(&self) -> usize {
    self.bottom - self.top
  }
}

/// Implementation of Display trait for Rect.
impl std::fmt::Display for Rect {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({},{};{},{})", self.left, self.top, self.right, self.bottom)
  }
}

/// Implementation of Debug trait for Rect.
impl std::fmt::Debug for Rect {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    write!(f, "({},{};{},{})", self.left, self.top, self.right, self.bottom)
  }
}
