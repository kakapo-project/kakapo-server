

pub trait ModResult<T> {

}

pub struct UpdateSuccess<T> {
    old: T,
    new: T,
}

pub struct DeletedSuccess<T> {
    old: T,
}



// compounds
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
        old: T,
        requested: T,
    }
}

pub enum Updated<T> {
    Success {
        old: T,
        new: T,
    },
    Fail {
        name: String,
        requested: T, //TODO: can this be a partial update?
    }
}

pub enum Deleted<T> {
    Success {
        old: T,
    },
    Fail {
        name: String,
    }
}

pub type UpsertedSet<T> = Vec<Upserted<T>>;
pub type UpdatedSet<T> = Vec<Updated<T>>;
pub type CreatedSet<T> = Vec<Created<T>>;
pub type DeletedSet<T> = Vec<Deleted<T>>;

