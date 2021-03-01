extern crate serde;

use chrono::NaiveDateTime;

use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Feed {
    #[serde(rename = "entry")]
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Author {
    pub name: String,
    pub uri: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Entry {
    pub title: String,
    pub author: Author,
    pub link: Link,
    #[serde(with = "date_serializer")]
    pub updated: NaiveDateTime,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub href: String,
}

mod date_serializer {
    use chrono::NaiveDateTime;

    use serde::{de::Error, Deserialize, Deserializer};

    pub fn deserialize<'de, D: Deserializer<'de>>(
        deserializer: D,
    ) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(
            NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S+00:00")
                .map_err(D::Error::custom)?,
        )
    }
}

impl Feed {
    pub fn empty() -> Self {
        Feed { entries: vec![] }
    }

    pub fn combine(feeds: Vec<Feed>) -> Feed {
        let mut entries: Vec<Entry> = Vec::new();

        for mut feed in feeds {
            entries.append(&mut feed.entries);
        }

        entries.sort_by_key(|e| e.updated);
        entries.reverse();

        Feed { entries }
    }
}