use std::{sync::Arc, time::Duration};

#[cfg(feature = "actix-rt")]
pub mod actixrt;

#[cfg(feature = "tokio")]
pub mod tokio;

/// Create a cleanup cycle which automatically checks for
/// expired values and removes them from the map.
pub trait Cleanup {
    /// Start a new cleanup cycle on the given [`TimedMap`](crate::TimedMap)
    /// instance and returns a function to cancel the
    /// cleanup cycle.
    ///
    /// On each elapse, the map ich checked for expired
    /// key-value pairs and removes them from the map.
    ///
    /// # Example
    /// ```
    /// use timedmap::{TimedMap, Cleanup};
    /// use std::time::Duration;
    /// use std::sync::Arc;
    ///
    /// let tm = Arc::new(TimedMap::new());
    /// tm.insert("foo", "bar", Duration::from_secs(60));
    ///
    /// # #[cfg(feature = "tokio")]
    /// # tokio_test::block_on(async {
    /// let cancel = Cleanup::start_cycle(tm, Duration::from_secs(10));
    ///
    /// cancel();
    /// # });
    /// ```
    fn start_cycle(m: Arc<Self>, interval: Duration) -> Box<dyn Fn()>;
}
