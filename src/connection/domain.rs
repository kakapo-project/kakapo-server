use std::collections::HashMap;

use diesel::prelude::*;

use plugins::v1::Domain;
use plugins::v1::DomainBuilder;

use metastore;

pub struct DomainCollection {
    map: HashMap<String, Box<Domain>>
}

impl DomainCollection {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn insert(&mut self, name: &str, domain: Box<Domain>) {
        self.map.insert(name.to_owned(), domain);
    }

    pub fn get(&self, name: &str) -> Option<&Box<Domain>> {
        self.map.get(name)
    }

    //TODO: this should be the metastore
    pub fn sync_with_database(&self, database_url: &str) -> Option<()> {
        metastore::sync_domains(database_url, &self.map)
            .map_err(|err| {
                error!("Encountered an error {:?}", &err);
                err
            })
            .ok()
    }
}


