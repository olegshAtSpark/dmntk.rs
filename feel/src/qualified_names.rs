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

//! `FEEL` qualified names.

use crate::Name;
use std::ops::Deref;

/// FEEL `QualifiedName`.
#[derive(Debug, Clone, PartialEq)]
pub struct QualifiedName(Vec<Name>);

impl QualifiedName {
  /// Creates a [QualifiedName] from [Names](Name).
  pub fn new(names: &[&Name]) -> Self {
    Self(names.iter().map(|&v| v.clone()).collect::<Vec<Name>>())
  }
}

impl ToString for QualifiedName {
  /// Converts [QualifiedName] to [String].
  fn to_string(&self) -> String {
    self.0.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(".")
  }
}

impl Deref for QualifiedName {
  type Target = Vec<Name>;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl QualifiedName {
  /// Appends this [QualifiedName] with a given [Name].
  pub fn push(&mut self, name: Name) {
    self.0.push(name);
  }
}

#[cfg(test)]
mod tests {
  use super::{Name, QualifiedName};

  /// Tests whether the constructor creates a new [QualifiedName] properly.
  #[test]
  fn qualified_name() {
    let name_a = Name::new(&["a", "+", "b"]);
    let name_b = Name::new(&["b", "-", "c"]);
    let name_c = Name::new(&["c", "/", "d"]);
    let name_d = Name::new(&["d", "*", "e"]);
    let name_e = Name::new(&["e", ".", "f"]);
    let name_f = Name::new(&["f", "'", "g"]);
    let qname = QualifiedName::new(&[]);
    assert_eq!("", qname.to_string().as_str());
    let qname = QualifiedName::new(&[&name_a]);
    assert_eq!("a+b", qname.to_string().as_str());
    let qname = QualifiedName::new(&[&name_a, &name_b]);
    assert_eq!("a+b.b-c", qname.to_string().as_str());
    let qname = QualifiedName::new(&[&name_a, &name_b, &name_c]);
    assert_eq!("a+b.b-c.c/d", qname.to_string().as_str());
    let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d]);
    assert_eq!("a+b.b-c.c/d.d*e", qname.to_string().as_str());
    let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d, &name_e]);
    assert_eq!("a+b.b-c.c/d.d*e.e.f", qname.to_string().as_str());
    let qname = QualifiedName::new(&[&name_a, &name_b, &name_c, &name_d, &name_e, &name_f]);
    assert_eq!("a+b.b-c.c/d.d*e.e.f.f'g", qname.to_string().as_str());
  }
}
