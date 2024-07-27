# ghiqc (GitHub Issues Quality Checker)

[![DevOps By Rultor.com](http://www.rultor.com/b/h1alexbel/ghiqc)](http://www.rultor.com/p/h1alexbel/ghiqc)
[![We recommend IntelliJ IDEA](https://www.elegantobjects.org/intellij-idea.svg)](https://www.jetbrains.com/idea/)

[![just](https://github.com/h1alexbel/ghiqc/actions/workflows/just.yml/badge.svg)](https://github.com/h1alexbel/ghiqc/actions/workflows/just.yml)
[![Crates.io Version](https://img.shields.io/crates/v/ghiqc)](https://crates.io/crates/ghiqc)
[![codecov](https://codecov.io/gh/h1alexbel/ghiqc/graph/badge.svg?token=)](https://codecov.io/gh/h1alexbel/ghiqc)
[![PDD status](http://www.0pdd.com/svg?name=h1alexbel/ghiqc)](http://www.0pdd.com/p?name=h1alexbel/ghiqc)
[![Hits-of-Code](https://hitsofcode.com/github/h1alexbel/ghiqc)](https://hitsofcode.com/view/github/h1alexbel/ghiqc)
[![License](https://img.shields.io/badge/license-MIT-green.svg)](https://github.com/h1alexbel/ghiqc/blob/master/LICENSE.txt)
[![Known Vulnerabilities](https://snyk.io/test/github/h1alexbel/ghiqc/badge.svg)](https://snyk.io/test/github/h1alexbel/ghiqc)

Command-line tool for checking quality of bug reports in GitHub Issues.

**Motivation**.
The quality of bug reports is paramount for the overall quality of a
software project: [poorly formulated bug reports][right-way-to-report-bugs]
often lead to wasted time, programmers frustration, and delays. This repository
is a command-line tool that would scan given issue for quality problems in its
formulations, generate recommendation, and report to the bug report author
asking to fix the report.

## How to use?

First, install it from [crate][ghiqc-crate]:

```bash
cargo install ghiqc
```

or with [homebrew] (macOS):

```bash
brew install ghiqc
```

Then, run it:

```bash
ghiqc --repo h1alexbel/fakehub --issue 1
```

### CLI Options

You can use the following options within `ghiqc` command-line tool:

| Name         | Value   | Default | Description                                                    |
|--------------|---------|---------|----------------------------------------------------------------|
| `repo`, `r`  | String  | -       | Repository to check, in @owner/repo format, i.e. jeff/foo.     |
| `issue`, `i` | int     | -       | Issue number to check.                                         |
| `stdout`     | boolean | `false` | Print the result to the console, instead of posting on GitHub. |
| `verbose`    | boolean | `false` | Verbose run output, i.e. debug logs, etc.                      |

There are two more arguments `ghiqc` will look for: `GITHUB_TOKEN` and
`DEEPINFRA_TOKEN`. They should be located in your environment variables. Export
them like that:

```bash
export GITHUB_TOKEN=...
export DEEPINFRA_TOKEN=...
```

## How to contribute?

Make sure that you have [Rust] and [just] installed on your system, then fork
this repository, make changes, send us a [pull request][guidelines]. We will
review your changes and apply them to the `master` branch shortly, provided
they don't violate our quality standards. To avoid frustration, before sending
us your pull request please run full build:

```bash
just full
```

Here is the [contribution vitals][Zerocracy Vitals], made by [zerocracy/judges-action]
(updated every hour!).

[right-way-to-report-bugs]: https://www.yegor256.com/2018/04/24/right-way-to-report-bugs.html
[ghiqc-crate]: https://crates.io/crates/ghiqc
[homebrew]: https://brew.sh
[Rust]: https://www.rust-lang.org/tools/install
[guidelines]: https://www.yegor256.com/2014/04/15/github-guidelines.html
[just]: https://just.systems/man/en/chapter_4.html
[Zerocracy Vitals]: https://www.h1alexbel.xyz/ghiqc/zerocracy/ghiqc-vitals.html
[zerocracy/judges-action]: https://github.com/zerocracy/judges-action
