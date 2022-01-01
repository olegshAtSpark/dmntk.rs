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

//! `FEEL` name.

use dmntk_common::Jsonify;
use std::ops::Deref;

/// Common type definition for optional name.
pub type OptName = Option<Name>;

/// `FEEL` name.
#[derive(Debug, Default, Clone, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Name(Vec<String>);

impl From<Vec<String>> for Name {
  /// Converts a vector of strings into [Name].
  fn from(value: Vec<String>) -> Self {
    Self(value.iter().map(|v| v.trim().to_string()).collect::<Vec<String>>())
  }
}

impl From<Vec<&str>> for Name {
  /// Converts a vector of strings into [Name].
  fn from(value: Vec<&str>) -> Self {
    Self(value.iter().map(|v| v.trim().to_string()).collect::<Vec<String>>())
  }
}

impl From<String> for Name {
  /// Converts a [String] into [Name].
  fn from(value: String) -> Self {
    Self(vec![value])
  }
}

impl From<&str> for Name {
  /// Converts a reference to [str] into [Name].
  fn from(value: &str) -> Self {
    Self::from(value.to_string())
  }
}

impl From<Name> for String {
  /// Converts [Name] to its [String] representation.
  fn from(value: Name) -> Self {
    value.to_string()
  }
}

impl From<&Name> for String {
  /// Converts a reference to [Name] to its [String] representation.
  fn from(value: &Name) -> Self {
    value.to_string()
  }
}

impl std::fmt::Display for Name {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let mut result = String::new();
    let mut current;
    let mut prev = false;
    for (index, part) in self.0.iter().enumerate() {
      current = matches!(part.as_str(), "." | "/" | "-" | "'" | "+" | "*");
      if index > 0 && !prev && !current && !part.is_empty() {
        result.push(' ');
      }
      result.push_str(part);
      prev = current;
    }
    write!(f, "{}", result)
  }
}

impl Jsonify for Name {
  /// Converts [Name] to its `JSON` representation.
  fn jsonify(&self) -> String {
    format!("[{}]", self.0.iter().map(|s| format!(r#""{}""#, s)).collect::<Vec<String>>().join(","))
  }
}

impl Name {
  /// Creates a [Name] from name parts.
  pub fn new(parts: &[&str]) -> Self {
    Self(parts.iter().map(|&v| v.trim().to_string()).collect::<Vec<String>>())
  }
  /// Returns `true` when the specified character is an additional name symbol.
  pub fn is_additional_name_symbol(ch: char) -> bool {
    matches!(ch, '.' | '/' | '-' | '\'' | '+' | '*')
  }
}

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
  use std::collections::HashMap;

  /// Tests if the default value for [Name] is an empty vector of strings.
  #[test]
  fn default_name() {
    let name: Name = Default::default();
    assert_eq!("", name.to_string().as_str());
  }

  /// Tests creating a [Name] from vector of strings.
  #[test]
  fn from_string_vector() {
    let name: Name = vec!["".to_string(), "".to_string(), "".to_string()].into();
    assert_eq!("", name.to_string().as_str());
    let name: Name = vec!["x".to_string(), "y".to_string()].into();
    assert_eq!("x y", name.to_string().as_str());
    let name: Name = vec!["x".to_string(), "+".to_string(), "y".to_string()].into();
    assert_eq!("x+y", name.to_string().as_str());
    let name: Name = vec!["a".to_string(), "b".to_string(), "c".to_string()].into();
    assert_eq!("a b c", name.to_string().as_str());
  }

  /// Tests creating a [Name] from vector of str.
  #[test]
  fn from_str_vector() {
    let name: Name = vec!["", "", ""].into();
    assert_eq!("", name.to_string().as_str());
    let name: Name = vec!["x", "y"].into();
    assert_eq!("x y", name.to_string().as_str());
    let name: Name = vec!["x", "+", "y"].into();
    assert_eq!("x+y", name.to_string().as_str());
    let name: Name = vec!["a", "b", "c"].into();
    assert_eq!("a b c", name.to_string().as_str());
  }

  #[test]
  fn additional_symbols() {
    let name: Name = vec!["x", "y"].into();
    assert_eq!("x y", name.to_string().as_str());
    let name: Name = vec!["x", ".", "y"].into();
    assert_eq!("x.y", name.to_string());
    let name: Name = vec![".", "x", "y"].into();
    assert_eq!(".x y", name.to_string());
    let name: Name = vec!["x", "y", "."].into();
    assert_eq!("x y.", name.to_string());
    let name: Name = vec!["x", "/", "y"].into();
    assert_eq!("x/y", name.to_string());
    let name: Name = vec!["x", "-", "y"].into();
    assert_eq!("x-y", name.to_string());
    let name: Name = vec!["x", "'", "y"].into();
    assert_eq!("x'y", name.to_string());
    let name: Name = vec!["x", "+", "y"].into();
    assert_eq!("x+y", name.to_string());
    let name: Name = vec!["x", "*", "y"].into();
    assert_eq!("x*y", name.to_string());
  }

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

  #[test]
  fn test_name_as_hash_map_key() {
    let name_alpha: Name = "alpha".into();
    let name_beta: Name = "beta".into();
    let name_gamma: Name = "gamma".into();
    let mut map = HashMap::new();
    map.insert(name_alpha.clone(), "ALPHA".to_string());
    map.insert(name_beta.clone(), "BETA".to_string());
    map.insert(name_gamma.clone(), "GAMMA".to_string());
    assert_eq!(3, map.len());
    assert!(map.contains_key(&name_alpha));
    assert_eq!("ALPHA", map.get(&name_alpha).unwrap());
    assert!(map.contains_key(&name_beta));
    assert_eq!("BETA", map.get(&name_beta).unwrap());
    assert!(map.contains_key(&name_gamma));
    assert_eq!("GAMMA", map.get(&name_gamma).unwrap());
    assert!(!map.contains_key(&"delta".into()));
  }
}
