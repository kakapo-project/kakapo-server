
use diesel::RunQueryDsl;

use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

use data;
use data::Named;
use data::permissions::Permission;

use state::user_management::UserManagementOps;

use kakapo_postgres::data::DataType;
use kakapo_postgres::data::Table;

use plugins::v1::DatastoreError;

fn get_sql_data_type(data_type: &DataType) -> String {
    match data_type {
        DataType::SmallInteger => format!("SMALLINT"),
        DataType::Integer => format!("INTEGER"),
        DataType::BigInteger => format!("BIGINT"),
        //DataType::Decimal { precision: u32, scale: u32 },
        DataType::Float => format!("REAL"),
        DataType::DoubleFloat => format!("DOUBLE PRECISION"),

        DataType::String => format!("TEXT"),
        DataType::VarChar { length } => format!("VARCHAR({})", length),

        DataType::Byte => format!("BYTEA"),

        DataType::Timestamp { with_tz } => match with_tz {
            true => format!("TIMESTAMP WITH TIME ZONE"),
            false => format!("TIMESTAMP"),
        },
        DataType::Date => format!("SMALLINT"),
        DataType::Time { with_tz } => format!("SMALLINT"), //TODO: with_tz
        //DataType::TimeInterval,

        DataType::Boolean => format!("BOOLEAN"),

        DataType::Json => format!("JSON"),
    }
}

pub struct UpdateTable<'a> {
    conn: &'a PooledConnection<ConnectionManager<PgConnection>>,
}

impl<'a> UpdateTable<'a> {
    pub fn new(conn: &'a PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self { conn }
    }
}

pub trait UpdateTableOps {
    fn create_table(&self, new: &Table) -> Result<(), DatastoreError>;

    fn update_table(&self, old: &Table, new: &Table) -> Result<(), DatastoreError>;

    fn delete_table(&self, old: &Table) -> Result<(), DatastoreError>;
}

//modify table in database here
impl<'a> UpdateTableOps for UpdateTable<'a> {
    fn create_table(&self, new: &Table) -> Result<(), DatastoreError> {

        let schema = &new.schema;
        let columns = &schema.columns;

        if columns.len() == 0 {
            Err(DatastoreError::NoColumns)?;
        }

        let formatted_columns: Vec<String> = columns.iter().map(|column| {
            let col_name = &column.name;
            let col_type = get_sql_data_type(&column.data_type);
            //TODO: nullable + default + serial
            format!("\"{}\" {}", col_name, col_type)
        }).collect();
        let command = format!("CREATE TABLE \"{}\" ({});", &new.name, formatted_columns.join(", "));
        info!("DSL command: `{}`", &command);

        //TODO: constraints...

        diesel::sql_query(command)
            .execute(self.conn)
            .or_else(|err|
                Err(DatastoreError::DbError(err.to_string())))?;

        //TODO: run DSL command to add permission to role

        Ok(())
    }

    fn update_table(&self, old: &Table, new: &Table) -> Result<(), DatastoreError> {
        unimplemented!();
        let command = format!("ALTER TABLE \"{}\";", &old.name);
        diesel::sql_query(command)
            .execute(self.conn)
            .or_else(|err|
                Err(DatastoreError::DbError(err.to_string())))?;

        Ok(())
    }

    fn delete_table(&self, old: &Table) -> Result<(), DatastoreError> {
        let command = format!("DROP TABLE \"{}\";", &old.name);
        diesel::sql_query(command)
            .execute(self.conn)
            .or_else(|err|
                Err(DatastoreError::DbError(err.to_string())))?;

        Ok(())
    }
}