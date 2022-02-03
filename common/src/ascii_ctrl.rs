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

//! ASCII control sequences.

pub const ASCII_BLACK: &str = "\u{001b}[30m";
pub const ASCII_RED: &str = "\u{001b}[31m";
pub const ASCII_GREEN: &str = "\u{001b}[32m";
pub const ASCII_YELLOW: &str = "\u{001b}[33m";
pub const ASCII_BLUE: &str = "\u{001b}[34m";
pub const ASCII_MAGENTA: &str = "\u{001b}[35m";
pub const ASCII_CYAN: &str = "\u{001b}[36m";
pub const ASCII_WHITE: &str = "\u{001b}[37m";

pub const ASCII_BRIGHT_BLACK: &str = "\u{001b}[30;1m";
pub const ASCII_BRIGHT_RED: &str = "\u{001b}[31;1m";
pub const ASCII_BRIGHT_GREEN: &str = "\u{001b}[32;1m";
pub const ASCII_BRIGHT_YELLOW: &str = "\u{001b}[33;1m";
pub const ASCII_BRIGHT_BLUE: &str = "\u{001b}[34;1m";
pub const ASCII_BRIGHT_MAGENTA: &str = "\u{001b}[35;1m";
pub const ASCII_BRIGHT_CYAN: &str = "\u{001b}[36;1m";
pub const ASCII_BRIGHT_WHITE: &str = "\u{001b}[37;1m";

pub const ASCII_BG_BLACK: &str = "\u{001b}[40m";
pub const ASCII_BG_RED: &str = "\u{001b}[41m";
pub const ASCII_BG_GREEN: &str = "\u{001b}[42m";
pub const ASCII_BG_YELLOW: &str = "\u{001b}[43m";
pub const ASCII_BG_BLUE: &str = "\u{001b}[44m";
pub const ASCII_BG_MAGENTA: &str = "\u{001b}[45m";
pub const ASCII_BG_CYAN: &str = "\u{001b}[46m";
pub const ASCII_BG_WHITE: &str = "\u{001b}[47m";

pub const ASCII_BG_BRIGHT_BLACK: &str = "\u{001b}[40;1m";
pub const ASCII_BG_BRIGHT_RED: &str = "\u{001b}[41;1m";
pub const ASCII_BG_BRIGHT_GREEN: &str = "\u{001b}[42;1m";
pub const ASCII_BG_BRIGHT_YELLOW: &str = "\u{001b}[43;1m";
pub const ASCII_BG_BRIGHT_BLUE: &str = "\u{001b}[44;1m";
pub const ASCII_BG_BRIGHT_MAGENTA: &str = "\u{001b}[45;1m";
pub const ASCII_BG_BRIGHT_CYAN: &str = "\u{001b}[46;1m";
pub const ASCII_BG_BRIGHT_WHITE: &str = "\u{001b}[47;1m";

pub const ASCII_RESET: &str = "\u{001b}[0m";

#[macro_export]
macro_rules! ascii256 {
  ($l:literal) => {{
    format!("\u{001b}[38;5;{}m", $l)
  }};
}

#[macro_export]
macro_rules! ascii_none {
  () => {{
    "".to_string()
  }};
}

#[cfg(test)]
mod tests {
  use super::*;

  fn test_display_8_colors() {
    print!("{}0{} ", ASCII_BLACK, ASCII_RESET);
    print!("{}1{} ", ASCII_RED, ASCII_RESET);
    print!("{}2{} ", ASCII_GREEN, ASCII_RESET);
    print!("{}3{} ", ASCII_YELLOW, ASCII_RESET);
    print!("{}4{} ", ASCII_BLUE, ASCII_RESET);
    print!("{}5{} ", ASCII_MAGENTA, ASCII_RESET);
    print!("{}6{} ", ASCII_CYAN, ASCII_RESET);
    print!("{}7{} ", ASCII_WHITE, ASCII_RESET);
    print!("\n\n");
  }

  fn test_display_8_bright_colors() {
    print!("{}0{} ", ASCII_BRIGHT_BLACK, ASCII_RESET);
    print!("{}1{} ", ASCII_BRIGHT_RED, ASCII_RESET);
    print!("{}2{} ", ASCII_BRIGHT_GREEN, ASCII_RESET);
    print!("{}3{} ", ASCII_BRIGHT_YELLOW, ASCII_RESET);
    print!("{}4{} ", ASCII_BRIGHT_BLUE, ASCII_RESET);
    print!("{}5{} ", ASCII_BRIGHT_MAGENTA, ASCII_RESET);
    print!("{}6{} ", ASCII_BRIGHT_CYAN, ASCII_RESET);
    print!("{}7{} ", ASCII_BRIGHT_WHITE, ASCII_RESET);
    print!("\n\n");
  }

  fn test_display_8_bg_colors() {
    print!("{} 0 {} ", ASCII_BG_BLACK, ASCII_RESET);
    print!("{} 1 {} ", ASCII_BG_RED, ASCII_RESET);
    print!("{} 2 {} ", ASCII_BG_GREEN, ASCII_RESET);
    print!("{} 3 {} ", ASCII_BG_YELLOW, ASCII_RESET);
    print!("{} 4 {} ", ASCII_BG_BLUE, ASCII_RESET);
    print!("{} 5 {} ", ASCII_BG_MAGENTA, ASCII_RESET);
    print!("{} 6 {} ", ASCII_BG_CYAN, ASCII_RESET);
    print!("{} 7 {} ", ASCII_BG_WHITE, ASCII_RESET);
    print!("\n\n");
  }

  fn test_display_8_bg_bright_colors() {
    print!("{} 0 {} ", ASCII_BG_BRIGHT_BLACK, ASCII_RESET);
    print!("{} 1 {} ", ASCII_BG_BRIGHT_RED, ASCII_RESET);
    print!("{} 2 {} ", ASCII_BG_BRIGHT_GREEN, ASCII_RESET);
    print!("{} 3 {} ", ASCII_BG_BRIGHT_YELLOW, ASCII_RESET);
    print!("{} 4 {} ", ASCII_BG_BRIGHT_BLUE, ASCII_RESET);
    print!("{} 5 {} ", ASCII_BG_BRIGHT_MAGENTA, ASCII_RESET);
    print!("{} 6 {} ", ASCII_BG_BRIGHT_CYAN, ASCII_RESET);
    print!("{} 7 {} ", ASCII_BG_BRIGHT_WHITE, ASCII_RESET);
    print!("\n\n");
  }

  fn test_display_256_colors() {
    for i in 0..16 {
      for j in 0..16 {
        let code = format!("{}", i * 16 + j);
        print!("\u{001b}[38;5;{}m{:>4}{}", code, code, ASCII_RESET)
      }
      println!();
    }
    println!();
  }

  fn test_macro() {
    print!("{}9{} ", ascii256!(9), ASCII_RESET);
    print!("{}104{} ", ascii256!(104), ASCII_RESET);
    print!("{}220{}", ascii256!(220), ASCII_RESET);
    print!("\n\n");
  }

  #[test]
  fn test_display_all() {
    test_display_8_colors();
    test_display_8_bright_colors();
    test_display_8_bg_colors();
    test_display_8_bg_bright_colors();
    test_display_256_colors();
    test_macro();
  }
}
