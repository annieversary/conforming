use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

pub fn to_string_serialize<T: ToString>(t: &T) -> Result<String, ()> {
    Ok(t.to_string())
}

pub fn datetime_ser(t: &NaiveDateTime) -> Result<String, ()> {
    Ok(t.format("%Y-%m-%dT%H:%M").to_string())
}
pub fn date_ser(t: &NaiveDate) -> Result<String, ()> {
    Ok(t.format("%Y-%m-%d").to_string())
}
pub fn time_ser(t: &NaiveTime) -> Result<String, ()> {
    Ok(t.format("%H:%M").to_string())
}
