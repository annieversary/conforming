use serde::Serialize;

#[derive(Serialize)]
pub struct UrlEncodeSerialize<T: Serialize> {
    value: T,
}

pub fn ser<T: Serialize>(value: T) -> Result<String, serde_urlencoded::ser::Error> {
    let s = serde_urlencoded::to_string(UrlEncodeSerialize { value })?;

    if let Some(s) = s.strip_prefix("value=") {
        Ok(s.to_string())
    } else {
        Ok(s)
    }
}
