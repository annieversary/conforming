use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::Serialize;

pub trait FormField: Serialize {
    const TYPE: &'static str = "text";
    const REQUIRED: bool = true;
    const SERIALIZER: fn(&Self) -> Result<String, Self::SerializerError>;
    type SerializerError;
}

impl<T: FormField> FormField for Option<T> {
    const SERIALIZER: fn(&Self) -> Result<String, Self::SerializerError> = serde_json::to_string;
    type SerializerError = serde_json::Error;
    const REQUIRED: bool = false;
}

macro_rules! impls {
    ( $type:literal, $ser:path, $error:ty, $( $ty:ident ),* ) => {
        $(
            impl FormField for $ty {
                const SERIALIZER: fn(&Self) -> Result<String, Self::SerializerError> = $ser;
                type SerializerError = $error;
                const TYPE: &'static str = $type;
            }
        )*
    };
}

impls!("text", crate::helpers::to_string_serialize, (), String);
impls!(
    "number",
    serde_json::to_string,
    serde_json::Error,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32,
    f64
);

impl<'a> FormField for &'a str {
    const SERIALIZER: fn(&Self) -> Result<String, Self::SerializerError> =
        crate::helpers::to_string_serialize;
    type SerializerError = ();
}

// TODO impls for chrono and time and stuff

// TODO this will eventually be a checkbox or something
impls!("text", serde_json::to_string, serde_json::Error, bool);

impls!(
    "datetime-local",
    serde_json::to_string,
    serde_json::Error,
    NaiveDateTime
);

impls!("date", serde_json::to_string, serde_json::Error, NaiveDate);
impls!("time", serde_json::to_string, serde_json::Error, NaiveTime);
