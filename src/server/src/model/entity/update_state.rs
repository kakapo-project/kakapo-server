use model::entity::results::*;
use model::entity::error::EntityError;


/// This trait does something action specific after the database updates
/// The name is a little bit confusing because the database store is also modification
/// But this module is responsible for all the other modifications
pub trait UpdateState: Sized {
    fn update_state(&self) -> Result<Self, EntityError>;
}


impl<T> UpdateState for Upserted<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}

impl<T> UpdateState for Created<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}

impl<T> UpdateState for Updated<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}

impl<T> UpdateState for Deleted<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}

impl<T> UpdateState for UpsertedSet<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}

impl<T> UpdateState for CreatedSet<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}

impl<T> UpdateState for UpdatedSet<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}

impl<T> UpdateState for DeletedSet<T> {
    fn update_state(&self) -> Result<Self, EntityError> {
        unimplemented!()
    }
}
