use crate::Cleanup;
use std::{sync::Arc, time::Duration};

pub fn _start_cleaner(m: Arc<dyn Cleanup>, interval: Duration) -> Box<dyn Fn()> {
    let job = tokio::spawn(async move {
        loop {
            tokio::time::sleep(interval).await;
            m.cleanup();
        }
    });
    Box::new(move || job.abort())
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::TimedMap;
    use tokio::time;

    #[tokio::test]
    async fn cleanup() {
        let tm = Arc::new(TimedMap::new());
        tm.insert("a", 1, Duration::from_millis(100));
        tm.insert("b", 2, Duration::from_millis(200));

        let _ = _start_cleaner(tm.clone(), Duration::from_millis(10));

        assert!(tm.get_value_unchecked(&"a").is_some());
        assert!(tm.get_value_unchecked(&"b").is_some());

        time::sleep(Duration::from_millis(150)).await;

        assert!(tm.get_value_unchecked(&"a").is_none());
        assert!(tm.get_value_unchecked(&"b").is_some());

        time::sleep(Duration::from_millis(60)).await;
        assert!(tm.get_value_unchecked(&"a").is_none());
        assert!(tm.get_value_unchecked(&"b").is_none());
    }
}
