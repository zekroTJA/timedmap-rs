#[cfg(feature = "actix-rt")]
pub mod actixrt;
#[cfg(feature = "actix-rt")]
use self::actixrt::_start_cleaner;

#[cfg(feature = "tokio")]
mod tokio;
#[cfg(feature = "tokio")]
use self::tokio::_start_cleaner;

/// Cleanup defines an implementation where expired
/// elements can be removed.
pub trait Cleanup: Send + Sync {
    /// Cleanup removes all elements
    /// which have been expired.
    fn cleanup(&self);
}

#[cfg(any(feature = "tokio", feature = "actix-rt"))]
/// Start a new cleanup cycle on the given [`Cleanup`](crate::Cleanup)
/// implementation instance and returns a function to cancel the
/// cleanup cycle.
///
/// On each elapse, the map ich checked for expired
/// key-value pairs and removes them from the map.
///
/// # Example
/// ```
/// use timedmap::{TimedMap, start_cleaner};
/// use std::time::Duration;
/// use std::sync::Arc;
///
/// let tm = Arc::new(TimedMap::new());
/// tm.insert("foo", "bar", Duration::from_secs(60));
///
/// # #[cfg(feature = "tokio")]
/// # tokio_test::block_on(async {
/// let cancel = start_cleaner(tm, Duration::from_secs(10));
///
/// cancel();
/// # });
/// ```
pub fn start_cleaner(
    m: std::sync::Arc<dyn Cleanup>,
    interval: std::time::Duration,
) -> Box<dyn Fn()> {
    _start_cleaner(m, interval)
}
