use model::entity::results::*;

pub trait DoModification {
    fn post_modification(&self) -> Self;
}


impl<T> DoModification for Upserted<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}

impl<T> DoModification for Created<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}

impl<T> DoModification for Updated<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}

impl<T> DoModification for Deleted<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}

impl<T> DoModification for UpsertedSet<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}

impl<T> DoModification for CreatedSet<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}

impl<T> DoModification for UpdatedSet<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}

impl<T> DoModification for DeletedSet<T> {
    fn post_modification(&self) -> Self {
        unimplemented!()
    }
}
