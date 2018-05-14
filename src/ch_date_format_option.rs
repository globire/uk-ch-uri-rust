use chrono::{DateTime, Utc, TimeZone};
use serde::{Deserialize, Serializer, Deserializer};

const FORMAT: &'static str = "%d/%m/%Y";

// The signature of a serialize_with function must follow the pattern:
//
//    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
//    where
//        S: Serializer
//
// although it may also be generic over the input types T.
pub fn serialize<S>(o: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match *o {
        Some(date) => {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        },
        None => { serializer.serialize_str(&"".to_string()) }
    }
    
}

// The signature of a deserialize_with function must follow the pattern:
//
//    fn deserialize<'de, D>(D) -> Result<T, D::Error>
//    where
//        D: Deserializer<'de>
//
// although it may also be generic over the output types T.
pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error> 
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let parts = s.split("/").collect::<Vec<&str>>();
    let y = parts[2].parse::<i32>().unwrap();
    let m = parts[1].parse::<u32>().unwrap();
    let d = parts[0].parse::<u32>().unwrap();
    Ok(Some(Utc.ymd(y,m,d).and_hms(0, 0, 0)))
}