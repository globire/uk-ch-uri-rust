use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serializer, Deserializer};
use std::cmp::Ordering;

const FORMAT: &'static str = "%d/%m/%Y";

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let s = format!("{}", date.format(FORMAT));
    serializer.serialize_str(&s)
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let parts = s.split("/").collect::<Vec<&str>>();
    let y = parts[2].parse::<i32>().unwrap();
    let m = parts[1].parse::<u32>().unwrap();
    let d = parts[0].parse::<u32>().unwrap();
    Ok(Utc.ymd(y,m,d).and_hms(0, 0, 0))
}

pub fn empty_value() -> DateTime<Utc> {
    Utc.ymd(1970, 1, 1).and_hms(0, 0, 0)
}

pub fn is_empty(dt: DateTime<Utc>) -> bool {
    let zero = empty_value();
    zero.cmp(&dt) == Ordering::Equal
}