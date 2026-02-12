# Contributing

Thank you for contributing to the Cran project.

## Lints

The lints in each crates lib.rs file can not be changed under almost any circumstance. If some code does not pass the lints, please try to re factor so it does.
If after re factoring the code can still not pass the lints, please disable the lint just for that function/block and leave a descriptive comment about why the lint must be turned off.

## Formatting

This project uses the standard rust fmt formatting, before pushing please ensure that your code is formatted.

## Tests

If your code has a possibility of failing (math or formatting type stuff) with edge cases, please test your code thoroughly through those edge cases.
If your code really has more to do with just glue no tests are necessary most of the time.

## Crate Specific

### Cran Math

This needs to follow all the rules above. The Cran Math crate also needs to follow the following guidelines:
1. If it uses a non trivial algorithm, please document: the algorithm name, a breif description, and all edge cases it must handle documented
2. In general it should be generic (via num-traits), unless a specific valid reason not to be (maybe some algorithm), if this happens you are responsible for documenting why and making a clean interface.

### Cran 

This needs to follow all the rules above. The Cran Api crate also needs to follow the following guidelines:
1. All public functions need to have doc comments fully to Rust language spec with all possible panics and information documented
2. The code needs to be tested thoroughly as it is what the public will use.
3. Ensure that all public facing types and functions use descriptive names and follow proper conventions
