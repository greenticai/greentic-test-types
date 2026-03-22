#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub kind: &'static str,
}

pub fn schema_version() -> &'static str {
    "v2"
}