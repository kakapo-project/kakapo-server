
use kakapo_postgres::data::Table;
use kakapo_postgres::data::RawTableData;
use kakapo_postgres::data::ObjectValues;
use kakapo_postgres::data::ObjectKeys;
use kakapo_postgres::data::Value;
use kakapo_postgres::database::error::DbError;
use kakapo_postgres::database::DatabaseFunctions;

use diesel::r2d2::PooledConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::PgConnection;
use plugins::v1::DatastoreError;

pub struct CrudTable<'a> {
    conn: &'a PooledConnection<ConnectionManager<PgConnection>>,
    table: &'a Table,
}

impl<'a> CrudTable<'a> {
    pub fn new(table: &'a Table, conn: &'a PooledConnection<ConnectionManager<PgConnection>>) -> Self {
        Self { table, conn }
    }
}


pub trait CrudTableOps {
    fn retrieve(&self) -> Result<RawTableData, DatastoreError>;

    fn insert(&self, data: ObjectValues, fail_on_duplicate: bool) -> Result<RawTableData, DatastoreError>;

    fn upsert(&self, data: ObjectValues) -> Result<RawTableData, DatastoreError>;

    fn update(&self, keys: ObjectKeys, data: ObjectValues, fail_on_not_found: bool) -> Result<RawTableData, DatastoreError>;

    fn delete(&self, keys: ObjectKeys, fail_on_not_found: bool) -> Result<RawTableData, DatastoreError>;
}

impl<'a> CrudTableOps for CrudTable<'a> {
    fn retrieve(&self) -> Result<RawTableData, DatastoreError> {

        let query = format!("SELECT * FROM {}", &self.table.name);
        self.conn
            .exec(&query, vec![])
            .or_else(|err| Err(DatastoreError::DbError(err.to_string())))
    }

    fn insert(&self, data: ObjectValues, fail_on_duplicate: bool) -> Result<RawTableData, DatastoreError> {

        let table_column_names = self.table.get_column_names();
        let raw_data = data.as_list();
        let mut results = RawTableData::new(vec![], table_column_names.to_owned());

        for row in raw_data {
            let sql_column_names: Vec<String> = row.keys().map(|x| x.to_owned()).collect();
            let column_counts: Vec<String> = sql_column_names.iter().enumerate()
                .map(|(i, _)| format!("${}", i+1))
                .collect();
            let values = row.values().map(|x| x.to_owned()).collect();
            let query = format!(
                r#"INSERT INTO "{name}" ("{columns}") VALUES ({params}) RETURNING *;"#,
                name=&self.table.name,
                columns=sql_column_names.join(r#"", ""#),
                params=column_counts.join(r#", "#),
            );

            let new_row = self.conn
                .exec(&query, values)
                .or_else(|err| {
                    match err {
                        DbError::ConstraintError(_) => if !fail_on_duplicate {
                            Ok(RawTableData::new(vec![], table_column_names.to_owned()))
                        } else {
                            Err(DatastoreError::DbError(err.to_string()))
                        },
                        _ => Err(DatastoreError::DbError(err.to_string())),
                    }
                })?;

            results.append(new_row)
                .or_else(|_| {
                    error!("columns names are mismatched");
                    Err(DatastoreError::Unknown)
                })?;
        }

        Ok(results)
    }

    fn upsert(&self, data: ObjectValues) -> Result<RawTableData, DatastoreError> {
        //Note: doing this because I want to know whether it was an insert or update so that I can put in the correct data in the transactions table
        // otherise, maybe ON CONFLICT with triggers would have been the proper choice
        let table_column_names = self.table.get_column_names();
        let raw_data = data.as_list();
        let mut results = RawTableData::new(vec![], table_column_names.to_owned());

        for row in raw_data {
            let sql_column_names: Vec<String> = row.keys().map(|x| x.to_owned()).collect();
            let column_counts: Vec<String> = sql_column_names.iter().enumerate()
                .map(|(i, _)| format!("${}", i+1))
                .collect();
            let values = row.values().map(|x| x.to_owned()).collect();
            let query = format!(
                r#"INSERT INTO "{name}" ("{columns}") VALUES ({params}) RETURNING *;"#,
                name=&self.table.name,
                columns=sql_column_names.join(r#"", ""#),
                params=column_counts.join(r#", "#),
            );

            let new_row = self.conn
                .exec(&query, values)
                .or_else(|err| {
                    match err {
                        DbError::ConstraintError(_) => {
                            //TODO: update value
                            Err(DatastoreError::DbError(err.to_string()))
                        },
                        _ => Err(DatastoreError::DbError(err.to_string())),
                    }
                })?;

            results.append(new_row)
                .or_else(|_| {
                    error!("columns names are mismatched");
                    Err(DatastoreError::Unknown)
                })?;
        }

        Ok(results)
    }

    fn update(&self, keys: ObjectKeys, data: ObjectValues, fail_on_not_found: bool) -> Result<RawTableData, DatastoreError> {

        let table_column_names = self.table.get_column_names();
        let raw_keys = keys.as_list();
        let raw_data = data.as_list();
        let mut results = RawTableData::new(vec![], table_column_names.to_owned());

        for (key, row) in raw_keys.iter().zip(raw_data) {
            let column_names: Vec<String> = row.keys().map(|x| x.to_owned()).collect();
            let key_names: Vec<String> = key.keys().map(|x| x.to_owned()).collect();

            let mut values: Vec<Value> = row.values().map(|x| x.to_owned()).collect();
            let key_values: Vec<Value> = key.values().map(|x| x.to_owned().into_value()).collect();
            values.extend(key_values);

            let val_index = 1;
            let key_index = column_names.len() + 1;

            let query = format!(
                "UPDATE {name} SET {sets} WHERE {id} RETURNING *", //"UPDATE table SET value1 = 1, value2 = 2 WHERE id = my_id"
                name=&self.table.name,
                sets=column_names.iter().enumerate()
                    .map(|(i, x)| format!("{} = ${}", x, i+val_index))
                    .collect::<Vec<String>>()
                    .join(","),
                id=key_names.iter().enumerate()
                    .map(|(i, x)| format!("{} = ${}", x, i+key_index))
                    .collect::<Vec<String>>()
                    .join(" AND "),
            );

            let new_row = self.conn
                .exec(&query, values)
                .or_else(|err| {
                    match err {
                        DbError::NotFound => if !fail_on_not_found {
                            Ok(RawTableData::new(vec![], table_column_names.to_owned()))
                        } else {
                            Err(DatastoreError::DbError(err.to_string()))
                        },
                        _ => Err(DatastoreError::DbError(err.to_string())),
                    }
                })?;

            results.append(new_row)
                .or_else(|_| {
                    error!("columns names are mismatched");
                    Err(DatastoreError::Unknown)
                })?;
        }

        Ok(results)

    }

    fn delete(&self, keys: ObjectKeys, fail_on_not_found: bool) -> Result<RawTableData, DatastoreError> {

        let table_column_names = self.table.get_column_names();
        let raw_keys = keys.as_list();
        let mut results = RawTableData::new(vec![], table_column_names.to_owned());

        for key in raw_keys {
            let key_names: Vec<String> = key.keys().map(|x| x.to_owned()).collect();
            let values: Vec<Value> = key.values().map(|x| x.to_owned().into_value()).collect();

            let query = format!(
                "DELETE FROM {name} WHERE {id} RETURNING *", //"DELETE table WHERE id = my_id"
                name=&self.table.name,
                id=key_names.iter().enumerate()
                    .map(|(i, x)| format!("{} = ${}", x, i+1))
                    .collect::<Vec<String>>()
                    .join(" AND "),
            );

            let new_row = self.conn
                .exec(&query, values)
                .or_else(|err| {
                    match err {
                        DbError::NotFound => if !fail_on_not_found {
                            Ok(RawTableData::new(vec![], table_column_names.to_owned()))
                        } else {
                            Err(DatastoreError::DbError(err.to_string()))
                        },
                        _ => Err(DatastoreError::DbError(err.to_string())),
                    }
                })?;

            results.append(new_row)
                .or_else(|_| {
                    error!("columns names are mismatched");
                    Err(DatastoreError::Unknown)
                })?;
        }

        Ok(results)
    }
}
