use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct TodoModel {
    pub id: u64,
    pub title: Option<String>,
    pub contents: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub is_completed: Option<String>,
    pub is_deleted: Option<String>
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct TodoModelResponse {
    pub id: u64,
    pub title: String,
    pub contents: String,
    #[serde(with = "datetime_tz_format")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(with = "datetime_tz_format")]
    pub updated_at: Option<DateTime<Utc>>,
    #[serde(with = "datetime_tz_format")]
    pub completed_at: Option<DateTime<Utc>>,
    pub is_completed: String,
    pub is_deleted: String
}

mod datetime_tz_format {
    use chrono::{DateTime, TimeZone, Utc, NaiveDateTime};
    use serde::{self, Deserialize, Serializer, Deserializer};
    use chrono_tz::Asia::Seoul;

    pub fn serialize<S>(
        date: &Option<DateTime<Utc>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(date) => {
                let seoul_time = date.with_timezone(&Seoul);
                let s = seoul_time.format("%Y-%m-%d %H:%M:%S").to_string();
                serializer.serialize_str(&s)
            }
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(
        deserializer: D,
    ) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: Option<String> = Option::deserialize(deserializer)?;
        match s {
            Some(s) => {
                let naive = NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
                    .map_err(serde::de::Error::custom)?;
                let seoul_time = Seoul.from_local_datetime(&naive)
                    .single() // 가능한 경우 단일 결과를 선택
                    .ok_or_else(|| serde::de::Error::custom("Ambiguous or invalid local time"))?;
                Ok(Some(seoul_time.with_timezone(&Utc)))
            }
            None => Ok(None),
        }
    }
}