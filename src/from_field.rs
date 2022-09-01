pub trait FormField {
    const TYPE: &'static str = "text";
    const REQUIRED: bool = true;
}

impl<T: FormField> FormField for Option<T> {
    const REQUIRED: bool = false;
}

macro_rules! impls {
    ( $type:literal, $( $ty:ident ),* ) => {
        $(
            impl FormField for $ty {
                const TYPE: &'static str = $type;
            }
        )*
    };
}

impls!("number", u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
