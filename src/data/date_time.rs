use typemap_rev::TypeMapKey;

pub struct Time {
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

pub struct Date {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl TypeMapKey for Time {
    type Value = Self;
}
impl TypeMapKey for Date {
    type Value = Self;
}
