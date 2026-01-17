use chrono::{DateTime, Utc};

impl Snapshot {
    pub fn datetime(&self) -> Option<DateTime<Utc>> {
        self.snaptime
            .and_then(|t| DateTime::<Utc>::from_timestamp(t, 0))
    }
}
