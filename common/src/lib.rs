/*
 * DMNTK - Decision Model and Notation Toolkit
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
 */

//! Common definitions for components of Decision Model and Notation Toolkit.

extern crate serde;
extern crate serde_derive;
extern crate serde_yaml;
extern crate uriparse;

mod errors;
mod examples;
mod href;
mod jsonify;
mod macros;

pub use errors::{DmntkError, Result};
pub use examples::{EXAMPLE_0001_CTX, EXAMPLE_0001_DTB, EXAMPLE_0002_CTX, EXAMPLE_0002_DTB};
pub use href::{HRef, OptHRef};
pub use jsonify::Jsonify;
