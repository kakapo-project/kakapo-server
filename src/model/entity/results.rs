

pub trait ModResult<T> {

}

pub struct UpdateSuccess<T> {
    old: T,
    new: T,
}

pub struct CreatedSuccess<T> {
    new: T,
}

pub struct DeletedSuccess<T> {
    old: T,
}


// compounds

pub enum Upserted<T> {
    Update(UpdateSuccess<T>),
    Create(CreatedSuccess<T>),
}

pub type Updated<T> = Option<UpdateSuccess<T>>;
pub type Created<T> = Option<CreatedSuccess<T>>;
pub type Deleted<T> = Option<DeletedSuccess<T>>;

pub type UpsertedSet<T> = Vec<Upserted<T>>;
pub type UpdatedSet<T> = Vec<Updated<T>>;
pub type CreatedSet<T> = Vec<Created<T>>;
pub type DeletedSet<T> = Vec<Deleted<T>>;

