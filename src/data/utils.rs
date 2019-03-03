

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnNotFound {
    Ignore,
    Fail
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnDuplicate {
    Ignore,
    Fail,
    Update,
}
