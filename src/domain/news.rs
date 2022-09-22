#[derive(Debug, Clone)]
pub struct NewsProvider {
    pub code: String,
    pub name: String,
}

impl NewsProvider {
    pub fn new(code: &str, name: &str) -> Self {
        NewsProvider {
            code: code.to_string(),
            name: name.to_string(),
        }
    }
}
