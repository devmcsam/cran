use core::fmt;

use serde::{Deserialize, Serialize};

use super::id::TestId;

/// The verdict of a single randomness test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TestVerdict {
    /// The data passed the test (looks random).
    Pass,
    /// The data failed the test (looks non-random).
    Fail,
}

/// The result of a single randomness test.
#[derive(Debug, Clone)]
pub struct TestResult<Id: TestId> {
    /// Which test produced this result.
    test_id: Id,
    /// The p-value in the range [0.0, 1.0].
    p_value: f64,
    /// The significance level used for the verdict.
    significance_level: f64,
    /// The computed test statistic (test-specific).
    statistic: f64,
}

impl<Id: TestId> TestResult<Id> {
    /// Creates a new `TestResult`.
    ///
    /// # Errors
    ///
    /// Returns an error if `p_value` is not in `[0.0, 1.0]`.
    pub fn new(
        test_id: Id,
        p_value: f64,
        significance_level: f64,
        statistic: f64,
    ) -> Result<Self, crate::CranCoreError> {
        if !(0.0..=1.0).contains(&p_value) {
            return Err(crate::CranCoreError::InvalidPValue(p_value));
        }
        Ok(Self {
            test_id,
            p_value,
            significance_level,
            statistic,
        })
    }

    /// The verdict based on the significance level.
    #[must_use]
    pub fn verdict(&self) -> TestVerdict {
        if self.p_value >= self.significance_level {
            TestVerdict::Pass
        } else {
            TestVerdict::Fail
        }
    }

    /// The identifier of the test that produced this result.
    #[must_use]
    pub const fn test_id(&self) -> Id {
        self.test_id
    }

    /// Human-readable name (delegates to `TestId::name`).
    #[must_use]
    pub fn test_name(&self) -> &'static str {
        self.test_id.name()
    }

    #[must_use]
    pub const fn p_value(&self) -> f64 {
        self.p_value
    }

    #[must_use]
    pub const fn statistic(&self) -> f64 {
        self.statistic
    }

    #[must_use]
    pub const fn significance_level(&self) -> f64 {
        self.significance_level
    }
}

impl<Id: TestId> fmt::Display for TestResult<Id> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}: p={:.6}, {:?}",
            self.test_id,
            self.p_value,
            self.verdict()
        )
    }
}
