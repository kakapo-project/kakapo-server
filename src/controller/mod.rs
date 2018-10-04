
pub mod crud;
pub mod types;
pub mod rows;
pub mod query;
pub mod schema;
pub mod repository;
pub mod meta;

use self::schema::{Schema, Reference, Constraint};
use self::types::DataType::*;
use self::repository::{Error, Repository, Transaction};

pub fn initialize_management_tables(repository: &Repository) -> Result<(), String> {

    repository.transaction()
        .and_then::<(), _>(|tr| {
            //TODO: check if already created.

            tr.create_table(&meta::user_account())?
                .create_table(&meta::scope())?
                .create_table(&meta::entity())?
                .create_table(&meta::entity())?
                .create_table(&meta::table_history())?
                .create_table(&meta::query())?
                .create_table(&meta::query_history())?
                .create_table(&meta::script())?
                .create_table(&meta::script_history())?
                .create_table(&meta::tag())?
                .create_table(&meta::entity_tag())?
                .create_table(&meta::role())?
                .create_table(&meta::user_account_role())?
                .create_table(&meta::permission())?
                .create_table(&meta::role_permission())?
                .create_table(&meta::transaction())?
                .create_table(&meta::version())?
                .commit()?;

            Ok(())
        }).or_else::<String, _>(|err| {
        match err {
            Error::TransactionError(transaction, msg) => {
                transaction.rollback();
                Err(msg)
            },
            Error::UsageError(msg) => Err(msg),
            Error::SystemError(msg) => Err(msg),
        }
    })
}