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
**DMNTK** is based on the [Decision Model and Notation (DMN™)](https://www.omg.org/dmn/) specification,
the industry standard led by the [Object Management Group (OMG®)](https://www.omg.org/),
the institution behind such standards like UML®, BPMN™ and CORBA®.
**DMNTK** is written in [Rust](https://www.rust-lang.org/), a programming language that empowers
building reliable and efficient software.
**DMNTK** aspires to be the fastest and fully compliant with [DMN™ specification](https://www.omg.org/spec/DMN)
decision model evaluator.

Main **DMNTK** features:

- Evaluation of DMN models as a service.
- Evaluation of decision tables.
- Evaluation of FEEL expressions.
- Parsing and validating DMN models.
- Parsing and recognizing decision tables.
- Parsing and validating FEEL expressions.
- Testing DMN models.
- Testing decision tables.
- Testing FEEL expressions.
- Exporting DMN decision models to HTML.
- Exporting decision tables to HTML.
- Exporting FEEL expressions to HTML.

## Installation

**DMNTK** ships as a single executable with no dependencies.

Binary version of **DMNTK** may be
- obtained directly from [GitHub releases](https://github.com/dmntk/dmntk.rs/releases),
- built and installed from source using [Cargo](https://crates.io/crates/dmntk).

### Installing DMNTK using [Cargo](https://crates.io/crates/dmntk):

```shell
$ cargo install dmntk
```

## Getting started

**DMNTK** provides examples ready to play with.

To install the examples run:

```
$ dmntk exs
```

### Evaluate FEEL expression

```text
$ cd ./examples/e1
$ dmntk efe e1.ctx e1.feel
3
```

### Evaluate DMN model

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

## Documentation

**DMNTK** documentation can be found on [dmntk.io](https://dmntk.io).

## License

Licensed under either of
[MIT license](https://opensource.org/licenses/MIT) ([LICENSE-MIT](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-MIT))
or
[Apache License, Version 2.0](https://www.apache.org/licenses/LICENSE-2.0) ([LICENSE-APACHE](https://github.com/dmntk/dmntk.rs/blob/main/LICENSE-APACHE))
at your option.

### Contribution

Unless you explicitly state otherwise, all contributions intentionally submitted for inclusion
in the work by you, shall be dual licensed as above, without any additional terms or conditions.