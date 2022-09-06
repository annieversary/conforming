pub fn to_string_serialize<T: ToString>(t: &T) -> Result<String, ()> {
    Ok(t.to_string())
}
