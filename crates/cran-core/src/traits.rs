use bitvec::prelude::*;

use crate::types::{TestId, TestResult};
use crate::CranCoreError;

/// The default significance level (α = 0.01).
pub const DEFAULT_SIGNIFICANCE_LEVEL: f64 = 0.01;

/// A test that evaluates the randomness of a bit sequence.
///
/// Every randomness test in `cran-math` (or user-defined) should implement this.
/// This is the fundamental building block of the library.
pub trait RandomnessTest: Send + Sync {
    /// The type of identifier this test uses.
    type Id: TestId;

    /// The identifier for this test.
    fn id(&self) -> Self::Id;

    /// The minimum number of bits required for this test to be meaningful.
    fn min_bits_required(&self) -> usize;

    /// Run the test on a bit sequence with the given significance level.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is too short or the test cannot be performed.
    fn evaluate(
        &self,
        data: &BitSlice<u8, Msb0>,
        significance_level: f64,
    ) -> Result<TestResult<Self::Id>, CranCoreError>;

    /// Convenience method: run with the default significance level.
    ///
    /// # Errors
    ///
    /// Returns an error if the input is too short or the test cannot be performed.
    fn run(&self, data: &BitSlice<u8, Msb0>) -> Result<TestResult<Self::Id>, CranCoreError> {
        self.evaluate(data, DEFAULT_SIGNIFICANCE_LEVEL)
    }
}
