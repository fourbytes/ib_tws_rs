#[derive(Debug, Clone, Default)]
pub struct SoftDollarTier {
    pub name: String,
    pub value: String,
    pub display_name: String,
}

impl SoftDollarTier {
    pub fn new(name: &str, value: &str, display_name: &str) -> Self {
        SoftDollarTier {
            name: name.to_string(),
            value: value.to_string(),
            display_name: display_name.to_string(),
        }
    }
}
