
use super::schema::{Schema, Reference, Constraint};
use super::types::DataType::*;
use super::repository::{Error, Repository, Transaction};

fn initialize_meta_tables(repository: &Repository) -> () {

     repository.transaction()
        .and_then::<&str, _>(|tr| {
            //TODO: check if already created.

            let user_account = Schema::new("user_account")
                .id_column()
                .column("username", &StringType)
                .column("password", &StringType)
                .column("email", &StringType);

            tr.create_table(&user_account.to_owned())?;

            let scope = Schema::new("scope")
                .id_column()
                .column("name", &StringType)
                .column("description", &StringType)
                .column("scope_info", &JsonType)
                .reference(&Reference::table("scope"));

            tr.create_table(&scope)?;

            let entity = Schema::new("entity")
                .id_column()
                .inherited_by(&vec!["table", "query", "script"])
                .column("created_at", &TimestampType)
                .reference(&Reference::table_on_column("user_account", "created_by"))
                .reference(&Reference::table("scope"));

            tr.create_table(&entity)?;

            let table = Schema::new("table")
                .id_column()
                .inherits("entity");

            tr.create_table(&entity)?;

            let table_history = Schema::new("table_history")
                .id_column()
                .reference(&Reference::table("table"))
                .column("name", &StringType)
                .column("description", &StringType)
                .column("table_info", &JsonType)
                .column("modified_at", &TimestampType)
                .reference(&Reference::table_on_column("user_account", "modified_by"));

            tr.create_table(&table_history)?;

            let query = Schema::new("query")
                .id_column()
                .inherits("entity");

            tr.create_table(&query)?;

            let query_histroy = Schema::new("query_history")
                .id_column()
                .reference(&Reference::table("query"))
                .column("name", &StringType)
                .column("description", &StringType)
                .column("statement", &StringType)
                .column("query_info", &JsonType)
                .column("modified_at", &TimestampType)
                .reference(&Reference::table_on_column("user_account", "modified_by"));

            tr.create_table(&query_histroy)?;

            let script = Schema::new("script")
                .id_column()
                .inherits("entity");

            tr.create_table(&script)?;

            let script_history = Schema::new("script_history")
                .id_column()
                .reference(&Reference::table("script"))
                .column("name", &StringType)
                .column("description", &StringType)
                .column("language", &StringType)
                .column("script", &StringType)
                .column("script_info", &JsonType)
                .column("modified_at", &TimestampType)
                .reference(&Reference::table_on_column("user_account", "modified_by"));

            tr.create_table(&script_history)?;

            let tag = Schema::new("tag")
                .id_column()
                .column("name", &StringType)
                .column("description", &StringType)
                .column("tag_info", &JsonType);

            tr.create_table(&tag)?;

            let entity_tag = Schema::new("entity_tag")
                .id_column()
                .junction("entity", "tag");

            tr.create_table(&entity_tag)?;

            let role = Schema::new("role")
                .id_column()
                .column("name", &StringType)
                .column("description", &StringType)
                .column("role_info", &JsonType);

            tr.create_table(&role)?;

            let user_account_role = Schema::new("user_account_role")
                .id_column()
                .junction("user_account", "role");

            tr.create_table(&user_account_role)?;

            let permission = Schema::new("permission")
                .id_column()
                .column("name", &StringType)
                .column("description", &StringType)
                .column("permission_info", &JsonType);

            tr.create_table(&permission)?;

            let role_permission = Schema::new("role_permission")
                .id_column()
                .junction("role", "permission");

            tr.create_table(&role_permission)?;

            let transaction = Schema::new("transaction")
                .id_column()
                .column("version", &StringType)
                .column("action", &JsonType)
                .column("timestamp", &TimestampType)
                .reference(&Reference::table("table"))
                .reference(&Reference::table("user_account"));

            tr.create_table(&transaction)?;

            let version = Schema::new("version")
                .id_column()
                .column("version", &StringType)
                .column("timestamp", &TimestampType);

            tr.create_table(&version)?;

            tr.commit()?;

            Ok("commited")
        }).or_else::<Error, _>(|err| {
            match err {
                Error::TransactionError(transaction, msg) => {
                    transaction.rollback();
                    Ok("rolled back")
                },
                Error::UsageError(msg) => Ok("rolled back"),
                Error::SystemError(msg) => Ok("rolled back"),
            }
        });
}