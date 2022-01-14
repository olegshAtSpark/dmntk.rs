# Decision Model and Notation Toolkit

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Apache 2.0 licensed][apache-badge]][apache-url]
![build][build-badge]
![tests][tests-badge]
![Code coverage][coverage-badge]

[crates-badge]: https://img.shields.io/crates/v/dmntk.svg
[crates-url]: https://crates.io/crates/dmntk
[mit-badge]: https://img.shields.io/badge/License-MIT-blue.svg
[mit-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT
[apache-badge]: https://img.shields.io/badge/License-Apache%202.0-blue.svg
[apache-url]: https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE
[build-badge]: https://github.com/dmntk/dmntk.rs/actions/workflows/build.yml/badge.svg
[tests-badge]: https://github.com/dmntk/dmntk.rs/actions/workflows/tests.yml/badge.svg
[coverage-badge]: https://img.shields.io/badge/Coverage-0%25-green.svg

## Overview

**DMNTK** is a set of tools for building, testing and evaluating decision models.

**DMNTK** is based on the [Decision Model and Notation (DMN™)](https://www.omg.org/dmn/),
the industry standard led by the [Object Management Group (OMG®)](https://www.omg.org/),
the institution behind such standards like UML®, BPMN™ and CORBA®.

**DMNTK** is written in [Rust](https://www.rust-lang.org/), a programming language that empowers
building reliable and efficient software.

**DMNTK** aims to be fully compliant with [DMN™ specification](https://www.omg.org/spec/DMN).

## Installation

Install DMNTK using `cargo`:

```shell
$ cargo install dmntk
```

Check available commands:

```shell
$ dmntk --help
```

## Overview

**DMNTK** may be used to:

- parse `FEEL` expressions,
- parse `DMN` models,
- parse decision tables,
- evaluate `FEEL` expressions,
- evaluate `DMN` models,
- evaluate decision tables,
- test `FEEL` expressions,
- test `DMN` models,
- test decision tables,

and last but not least:

- to evaluate `DMN` models as a service.

## Examples

**DMNTK** provides examples ready to play with.

To install the examples run:

```
$ dmntk exs
```

### Evaluate `FEEL` expression

```text
$ cd ./examples/e1
$ dmntk efe e1.ctx e1.feel
3
```

### Evaluate `DMN` model

```text
$ cd ./examples/e2
$ dmntk edm e2.ctx e2.dmn -i "Greeting Message"
"Hello John Doe"
```

### Evaluate decision table

```text
$ cd ./examples/e3
$ dmntk edt e3.ctx e3.dtb
0.15
```

### Run as a service

```text
$ cd ./examples/e2
$ dmntk srv -D .
Loaded 1 file(s) from directory: .
dmntk 0.0.0.0:22022
```

Switch to another terminal window and run: 

```text
$ curl -s -d '{"Full Name":"John Doe"}' -H "Content-Type: application/json" -X POST http://0.0.0.0:22022/evaluate/compliance-level-2-test-0001/Greeting%20Message
{"data":"Hello John Doe"}
```

## License

Licensed under either of

- [MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT))
- [Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE))

at your option.

### Contribution

All contributions intentionally submitted for inclusion in the work by you,
shall be dual licensed as above, without any additional terms or conditions.