use core::fmt;

/// Trait bound for test identifiers.
///
/// `cran-core` does not know the specific tests — it only requires that
/// an identifier can be copied, displayed, compared, hashed, and serialized.
///
/// The concrete enum lives in `cran-math` (or any downstream crate that
/// defines its own family of tests).
pub trait TestId:
    Copy + Eq + core::hash::Hash + fmt::Debug + fmt::Display + Send + Sync + 'static
{
    /// A human-readable name for this test.
    fn name(&self) -> &'static str;
}
