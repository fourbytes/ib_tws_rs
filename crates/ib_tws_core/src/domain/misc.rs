#[derive(Debug, Clone)]
pub struct FamilyCode {
    pub account_id: String,
    pub family_code: String,
}

#[derive(Debug, Clone)]
pub struct PriceIncrement {
    pub low_edge: f64,
    pub increment: f64,
}

#[derive(Debug, Copy, Clone)]
#[repr(i32)]
pub enum ServerLogLevel {
    System = 1,
    Error = 2,
    Warning = 3,
    Information = 4,
    Detail = 5,
}

impl TryFrom<i32> for ServerLogLevel {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, <Self as TryFrom<i32>>::Error> {
        match v {
            x if x == Self::System as i32 => Ok(Self::System),
            x if x == Self::Error as i32 => Ok(Self::Error),
            x if x == Self::Warning as i32 => Ok(Self::Warning),
            x if x == Self::Information as i32 => Ok(Self::Information),
            x if x == Self::Detail as i32 => Ok(Self::Detail),
            _ => Err(()),
        }
    }
}
