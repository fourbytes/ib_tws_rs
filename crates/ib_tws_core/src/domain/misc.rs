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
