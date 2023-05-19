use std::{
    hash::Hash,
    ops::{Add, AddAssign, Sub, SubAssign},
    time::{Duration, Instant},
};

/// Defines a time source used to retrieve the
/// current time and check the expiry of a
/// [`Value`](crate::Value).
pub trait TimeSource:
    Add<Duration, Output = Self>
    + AddAssign<Duration>
    + Sub<Duration, Output = Self>
    + SubAssign<Duration>
    + PartialOrd
    + Ord
    + Hash
    + PartialEq
    + Eq
    + Clone
{
    fn now() -> Self;
}

impl TimeSource for Instant {
    fn now() -> Self {
        Instant::now()
    }
}

#[cfg(test)]
impl TimeSource for mock_instant::Instant {
    fn now() -> Self {
        mock_instant::Instant::now()
    }
}
