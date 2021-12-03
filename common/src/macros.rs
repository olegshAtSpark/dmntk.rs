/*
 * DMNTK - Decision Model and Notation Toolkit
 *
 * Copyright 2018-2021 Dariusz Depta <dariusz.depta@dmntk.io>
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */

//! Common macro definitions.

/// Creates a `FEEL` **null** value with additional tracing message.
#[macro_export]
macro_rules! null_with_trace {
  ($f:expr, $($a:tt)*) => {
    Value::Null(Some(format!($f, $($a)*)))
  };
  ($l:expr) => {
    Value::Null(Some(format!("{}", $l)))
  };
  () => {
    Value::Null(None)
  };
}
