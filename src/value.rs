use crate::time::TimeSource;
use std::time::Duration;

/// Wraps a map value with a specified
/// expiry [`TimeSource`].
#[derive(Debug, Clone)]
pub struct Value<V, TS> {
    value: V,
    expires: TS,
}

impl<V, TS> Value<V, TS>
where
    V: Clone,
    TS: TimeSource,
{
    /// Creates a new [`Value`] with the given inner
    /// value and lifetime as [`Duration`].
    ///
    /// The values expiry is calculated by adding the
    /// specified lifetime to the result of `TS:now()`.
    pub fn new(value: V, lifetime: Duration) -> Self {
        Self {
            value,
            expires: TS::now() + lifetime,
        }
    }

    /// Returns `true` when the specified expiry is
    /// after the current time.
    pub fn is_expired(&self) -> bool {
        TS::now() > self.expires
    }

    /// Returns a reference to the values expiry
    /// [`TimeSource`].
    pub fn expires(&self) -> &TS {
        &self.expires
    }

    /// Sets the expiry of the value to now plus the
    /// given lifetime.
    pub fn set_expiry(&mut self, lifetime: Duration) {
        self.expires = TS::now() + lifetime;
    }

    /// Adds the given duration to the values
    /// expiry.
    pub fn add_expiry(&mut self, lifetime: Duration) {
        self.expires += lifetime;
    }

    /// Returns a copy of the inner value.
    pub fn value(&self) -> V {
        self.value.clone()
    }

    /// Returns a reference to the inner value.
    pub fn value_ref(&self) -> &V {
        &self.value
    }

    /// Returns a copy of the inner value if
    /// the expiry has not yet exceeded.
    pub fn value_checked(&self) -> Option<V> {
        if self.is_expired() {
            None
        } else {
            Some(self.value())
        }
    }

    /// Returns a reference to the inner value if
    /// the expiry has not yet exceeded.
    pub fn value_ref_checked(&self) -> Option<&V> {
        if self.is_expired() {
            None
        } else {
            Some(self.value_ref())
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use mock_instant::{Instant, MockClock};

    #[test]
    fn expiry() {
        let v: Value<_, Instant> = Value::new("foo", Duration::from_millis(100));
        assert_eq!(v.expires(), &(Instant::now() + Duration::from_millis(100)));
        assert!(!v.is_expired());
        assert_eq!(v.value_checked(), Some("foo"));

        MockClock::advance(Duration::from_millis(100));

        assert!(!v.is_expired());
        assert_eq!(v.value_checked(), Some("foo"));

        MockClock::advance(Duration::from_millis(1));

        assert!(v.is_expired());
        assert_eq!(v.value_checked(), None);
    }
}
