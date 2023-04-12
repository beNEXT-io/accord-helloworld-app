use chrono::{ DateTime, TimeZone, Utc };
use serde::{ Deserialize, Serialize, Deserializer, Serializer };
   
#[derive(Debug, Serialize, Deserialize)]
struct Helper(
   #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
   chrono::DateTime<Utc>,
 );
   
pub fn serialize_datetime_option<S>(datetime: &Option<chrono::DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
   S: Serializer,
{
   datetime.map(Helper).serialize(serializer)
}

pub fn deserialize_datetime_option<'de, D>(deserializer: D) -> Result<Option<chrono::DateTime<Utc>>, D::Error>
where
   D: Deserializer<'de>,
{
   let helper: Option<Helper> = Option::deserialize(deserializer)?;
    Ok(helper.map(|Helper(i)| i))
}

pub fn deserialize_datetime<'de, D>(deserializer: D) -> Result<chrono::DateTime<Utc>, D::Error>
where
   D: Deserializer<'de>,
{
   let datetime_str = String::deserialize(deserializer)?;
   Utc.datetime_from_str(&datetime_str, "%Y-%m-%dT%H:%M:%S%.3fZ").map_err(serde::de::Error::custom)
}
   
pub fn serialize_datetime<S>(datetime: &chrono::DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
   S: Serializer,
{
   let datetime_str = datetime.format("%Y-%m-%dT%H:%M:%S%.3fZ").to_string();
   serializer.serialize_str(&datetime_str)
}
