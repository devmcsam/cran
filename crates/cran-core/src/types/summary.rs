use super::id::TestId;
use super::result::{TestResult, TestVerdict};

/// Aggregated summary of multiple randomness test results.
#[derive(Debug, Clone)]
pub struct TestSummary<Id: TestId> {
    /// The individual test results.
    results: Vec<TestResult<Id>>,
    /// Total number of bits that were tested.
    bits_tested: usize,
}

impl<Id: TestId> TestSummary<Id> {
    /// Creates a new empty summary for a given input size.
    #[must_use]
    pub const fn new(bits_tested: usize) -> Self {
        Self {
            results: Vec::new(),
            bits_tested,
        }
    }

    /// Creates a summary from an existing list of results.
    #[must_use]
    pub const fn from_results(results: Vec<TestResult<Id>>, bits_tested: usize) -> Self {
        Self {
            results,
            bits_tested,
        }
    }

    /// Adds a result to the summary.
    pub fn push(&mut self, result: TestResult<Id>) {
        self.results.push(result);
    }

    /// Number of bits in the tested data.
    #[must_use]
    pub const fn bits_tested(&self) -> usize {
        self.bits_tested
    }

    /// All individual results.
    #[must_use]
    pub fn results(&self) -> &[TestResult<Id>] {
        &self.results
    }

    /// How many tests were run.
    #[must_use]
    pub const fn total(&self) -> usize {
        self.results.len()
    }

    /// How many tests passed.
    #[must_use]
    pub fn passed(&self) -> usize {
        self.results
            .iter()
            .filter(|r| r.verdict() == TestVerdict::Pass)
            .count()
    }

    /// How many tests failed.
    #[must_use]
    pub fn failed(&self) -> usize {
        self.total() - self.passed()
    }

    /// The overall verdict: passes only if every test passed.
    #[must_use]
    pub fn verdict(&self) -> TestVerdict {
        if self
            .results
            .iter()
            .all(|r| r.verdict() == TestVerdict::Pass)
        {
            TestVerdict::Pass
        } else {
            TestVerdict::Fail
        }
    }

    /// The minimum p-value across all results (the "weakest" test).
    #[must_use]
    pub fn min_p_value(&self) -> Option<f64> {
        self.results
            .iter()
            .map(TestResult::p_value)
            .reduce(f64::min)
    }
}
