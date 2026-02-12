use thiserror::Error;

/// Errors that can occur in cran-core.
#[derive(Debug, Error)]
pub enum CranCoreError {
    /// A p-value was outside the valid range [0.0, 1.0].
    #[error("invalid p-value: {0} (must be in [0.0, 1.0])")]
    InvalidPValue(f64),

    /// The input data was too short for the test.
    #[error("input too short: need at least {needed} elements, got {got}")]
    InputTooShort { needed: usize, got: usize },

    /// A generic test execution error.
    #[error("test failed: {0}")]
    TestFailed(String),
}
