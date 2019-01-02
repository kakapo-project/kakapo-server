
pub enum Upserted<T> {
    Update {
        old: T,
        new: T,
    },
    Create {
        new: T,
    },
}
pub enum Created<T> {
    Success {
        new: T,
    },
    Fail {
        existing: T,
    }
}

pub enum Updated<T> {
    Success {
        old: T,
        new: T,
    },
    Fail
}

pub enum Deleted<T> {
    Success {
        old: T,
    },
    Fail
}

pub type UpsertedSet<T> = Vec<Upserted<T>>;
pub type UpdatedSet<T> = Vec<Updated<T>>;
pub type CreatedSet<T> = Vec<Created<T>>;
pub type DeletedSet<T> = Vec<Deleted<T>>;

