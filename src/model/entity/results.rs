use std::fmt::Debug;

#[derive(Debug)]
pub enum Upserted<T: Debug> {
    Update {
        old: T,
        new: T,
    },
    Create {
        new: T,
    },
}

#[derive(Debug)]
pub enum Created<T: Debug> {
    Success {
        new: T,
    },
    Fail {
        existing: T,
    }
}

#[derive(Debug)]
pub enum Updated<T: Debug> {
    Success {
        old: T,
        new: T,
    },
    Fail
}

#[derive(Debug)]
pub enum Deleted<T: Debug> {
    Success {
        old: T,
    },
    Fail
}

