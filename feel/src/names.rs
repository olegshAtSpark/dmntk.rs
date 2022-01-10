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

/// Common type definition for optional name.
pub type OptName = Option<Name>;

/// `FEEL` name.
#[derive(Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd, Clone)]
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
  ///
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

#[cfg(test)]
mod tests {
  use super::Name;
  use std::collections::HashMap;

  #[test]
  fn test_default_name() {
    let name: Name = Default::default();
    assert_eq!("", name.to_string());
  }

  #[test]
  fn test_display() {
    let name: Name = vec!["   x   ".to_string(), " y      \t".to_string(), "  \n  z  \t  ".to_string()].into();
    assert_eq!("x y z", format!("{}", name));
  }

  #[test]
  fn test_debug() {
    let name: Name = vec!["   x   ".to_string(), " y      \t".to_string(), "  \n  z  \t  ".to_string()].into();
    assert_eq!(r#"Name(["x", "y", "z"])"#, format!("{:?}", name));
  }

  #[test]
  fn test_from_string_vector() {
    let name: Name = vec!["".to_string(), "".to_string(), "".to_string()].into();
    assert_eq!("", name.to_string());
    let name: Name = vec!["x".to_string(), "y".to_string()].into();
    assert_eq!("x y", name.to_string());
    let name: Name = vec!["x".to_string(), "+".to_string(), "y".to_string()].into();
    assert_eq!("x+y", name.to_string());
    let name: Name = vec!["x".to_string(), "    +    ".to_string(), "y".to_string()].into();
    assert_eq!("x+y", name.to_string());
    let name: Name = vec!["a".to_string(), "b".to_string(), "c".to_string()].into();
    assert_eq!("a b c", name.to_string());
    let name: Name = vec!["   x   ".to_string(), " y      \t".to_string(), "  \n  z  \t  ".to_string()].into();
    assert_eq!("x y z", name.to_string());
  }

  #[test]
  fn test_from_str_vector() {
    let name: Name = vec!["", "", ""].into();
    assert_eq!("", name.to_string());
    let name: Name = vec!["x", "y"].into();
    assert_eq!("x y", name.to_string());
    let name: Name = vec!["x", "+", "y"].into();
    assert_eq!("x+y", name.to_string());
    let name: Name = vec!["a", "b", "c"].into();
    assert_eq!("a b c", name.to_string());
    let name: Name = vec!["   x   ", " y      \t", "  \n  z  \t  "].into();
    assert_eq!("x y z", name.to_string());
  }

  #[test]
  fn additional_symbols() {
    let name: Name = vec!["x", "y"].into();
    assert_eq!("x y", name.to_string());
    let name: Name = vec!["x", ".", "y"].into();
    assert_eq!("x.y", name.to_string());
    let name: Name = vec!["x", "   .    ", "y"].into();
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

  #[test]
  fn test_equality() {
    let name_a: Name = "a".into();
    let name_b: Name = "b".into();
    let name_x_y: Name = vec!["x", "y"].into();
    let name_m_n: Name = vec!["m", "n"].into();
    let name_xy: Name = vec!["x y"].into();
    let name_mn: Name = vec!["m n"].into();
    // check the conversion to string
    assert_eq!("a", name_a.to_string());
    assert_eq!("b", name_b.to_string());
    assert_eq!("x y", name_x_y.to_string());
    assert_eq!("m n", name_m_n.to_string());
    assert_eq!("x y", name_xy.to_string());
    assert_eq!("m n", name_m_n.to_string());

    assert!((name_a == name_a));
    assert!(!(name_a != name_a));
    assert!((name_x_y == name_x_y));
    assert!(!(name_x_y != name_x_y));
    //assert!((name_x_y == name_xy));
    //assert!(!(name_x_y != name_xy));
    assert!((name_m_n == name_m_n));
    assert!(!(name_m_n != name_m_n));
    //assert!((name_m_n == name_mn));
    //assert!(!(name_m_n != name_mn));

    assert!(!(name_a == name_b));
    assert!((name_a != name_b));
    assert!(!(name_x_y == name_m_n));
    assert!((name_x_y != name_m_n));
    assert!(!(name_xy == name_mn));
    assert!((name_xy != name_mn));
    assert!(!(name_x_y == name_a));
    assert!((name_x_y != name_a));
    assert!(!(name_xy == name_a));
    assert!((name_xy != name_a));
    assert!(!(name_m_n == name_b));
    assert!((name_m_n != name_b));
    assert!(!(name_mn == name_b));
    assert!((name_mn != name_b));
  }
}
