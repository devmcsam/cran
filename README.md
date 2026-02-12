# Cran (Workspace root)

## Overview

This is the root directory for the Cran project. The Cran project is a tool to check if a list or stream of data is
random or not. To do this is it takes in the data and runs a suite of tests on it. Sometimes it can give a full true or
false answer but sometimes it can not. In that case it will return a probability that said data is random.

- - - 

## Sub crates

### cran-core

- This defines the core types, methods and traits of the project. Every other crate will depend on this crate.

### cran-cli

- This is the command line interface for the project.
- This relies only on the api crate (cran).
- This is mainly an example of how to implement stuff but also genuinely serves a purpose for those who need it.

### cran-math

- This crate contains the math-related types, functions and traits.
- This relies on the core crate only.
- Almost every thing in this crate should be pure functions.

### cran

- This is the api or lib crate for the project.
- This relies on every other crate in this workspace.
- This crate contains both a simple user facing api as well as some work to simplify the api.

- - - 

## License

This whole project is licensed under the MIT license. See the LICENSE file for the details of what that entails.