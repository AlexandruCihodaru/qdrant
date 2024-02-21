use std::fmt;

use serde::de::{self, MapAccess, Visitor};
use serde::{Deserialize, Deserializer};

#[derive(Debug, Deserialize, Clone)]
pub struct S3Config {
    _bucket: String,
    _region: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LocalPath {
    _snapshots_path: String,
}

impl LocalPath {
    pub fn new(snapshots_path: String) -> Self {
        Self {
            _snapshots_path: snapshots_path,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SnapshotStorage {
    LocalPath(LocalPath),
    S3(S3Config),
}

// Custom deserialization for `snapshot_path`
pub fn deserialize_snapshot_path<'de, D>(deserializer: D) -> Result<SnapshotStorage, D::Error>
where
    D: Deserializer<'de>,
{
    struct SnapshotPathVisitor;

    impl<'de> Visitor<'de> for SnapshotPathVisitor {
        type Value = SnapshotStorage;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string or a map with S3 configuration")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(SnapshotStorage::LocalPath(LocalPath::new(
                value.to_string(),
            )))
        }

        fn visit_map<V>(self, mut map: V) -> Result<Self::Value, V::Error>
        where
            V: MapAccess<'de>,
        {
            let mut bucket = None;
            let mut region = None;
            while let Some(key) = map.next_key::<String>()? {
                match key.as_str() {
                    "bucket" => bucket = Some(map.next_value()?),
                    "region" => region = Some(map.next_value()?),
                    _ => return Err(de::Error::unknown_field(&key, &["bucket", "region"])),
                }
            }
            let bucket = bucket.ok_or_else(|| de::Error::missing_field("bucket"))?;
            let region = region.ok_or_else(|| de::Error::missing_field("region"))?;
            Ok(SnapshotStorage::S3(S3Config {
                _bucket: bucket,
                _region: region,
            }))
        }
    }

    deserializer.deserialize_any(SnapshotPathVisitor)
}
