

#[derive(Debug)]
pub enum OnNotFound {
    Ignore,
    Fail
}

#[derive(Debug)]
pub enum OnDuplicate {
    Ignore,
    Fail,
    Update,
}