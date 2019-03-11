
use diesel::prelude::*;
use diesel;
use diesel::result::Error as DbError;
use diesel::result::DatabaseErrorKind as DbErrKind;


use state::DomainManagement;
use state::domain_management::DomainManagementOps;

use data::DomainInfo;
use state::error::DomainManagementError;
use metastore::schema;
use metastore::dbdata;

impl<'a> DomainManagementOps for DomainManagement<'a> {
    fn get_all_domains(&self) -> Result<Vec<DomainInfo>, DomainManagementError> {
        debug!("Getting all the domains");
        let domains = schema::domain::table
            .get_results::<dbdata::RawDomainInfo>(self.conn)
            .map_err(|err| {
                DomainManagementError::InternalError(err.to_string())
            })?;

        Ok(domains
            .into_iter()
            .map(|raw_domain| DomainInfo {
                name: raw_domain.name,
                type_info: raw_domain.type_,
                description: raw_domain.description,
            })
            .collect())
    }
}
