//! `timedmap` provides a thread-safe hash map with expiring key-value pairs and
//! automatic cleanup mechnaisms for popular async runtimes.
//!
//! # Basic Example
//! ```
//! use timedmap::TimedMap;
//! use std::time::Duration;
//!
//! let tm = TimedMap::new();
//! tm.insert("foo", 1, Duration::from_millis(100));
//! tm.insert("bar", 2, Duration::from_millis(200));
//! tm.insert("baz", 3, Duration::from_millis(300));
//! assert_eq!(tm.get(&"foo"), Some(1));
//! assert_eq!(tm.get(&"bar"), Some(2));
//! assert_eq!(tm.get(&"baz"), Some(3));
//!
//! std::thread::sleep(Duration::from_millis(120));
//! assert_eq!(tm.get(&"foo"), None);
//! assert_eq!(tm.get(&"bar"), Some(2));
//! assert_eq!(tm.get(&"baz"), Some(3));
//!
//! std::thread::sleep(Duration::from_millis(100));
//! assert_eq!(tm.get(&"foo"), None);
//! assert_eq!(tm.get(&"bar"), None);
//! assert_eq!(tm.get(&"baz"), Some(3));
//! ```
//!
//! # Cleanup Example
//!
//! You can use the `start_cleaner` function to automatically clean up
//! expired key-value pairs in given time intervals using popular
//! async runtimes.
//!
//! > Currently, only implementations for `tokio` and `actix-rt`
//! are available. Implentations for other popular runtimes are
//! planned in the future. If you want to contribute an implementation,
//! feel free to create a
//! [pull request](https://github.com/zekroTJA/timedmap-rs). 😄
//!
//! ```
//! use timedmap::{TimedMap, start_cleaner};
//! use std::time::Duration;
//! use std::sync::Arc;
//!
//! let tm = Arc::new(TimedMap::new());
//! tm.insert("foo", 1, Duration::from_secs(60));
//!
//! # #[cfg(feature = "tokio")]
//! # tokio_test::block_on(async {
//! let cancel = start_cleaner(tm, Duration::from_secs(10));
//!
//! cancel();
//! # });
//! ```

mod timedmap;
pub use crate::timedmap::*;

mod value;
pub use crate::value::*;

mod cleanup;
pub use crate::cleanup::*;

pub mod time;
