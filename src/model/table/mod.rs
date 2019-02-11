
pub mod error;

use data;
use data::Named;

use model::table::error::TableError;
use database::error::DbError;
use connection::executor::Conn;

pub struct TableAction<'a> {
    pub conn: &'a Conn,
}

pub trait DatabaseFunctions {
    fn exec(&self, query: &str, params: Vec<data::Value>) -> Result<data::RawTableData, DbError>;
}

pub trait TableActionFunctions {
    fn query(&self, table: &data::Table) -> Result<data::RawTableData, TableError>;

    fn insert_row(&self, table: &data::Table, data: &data::ObjectValues, fail_on_duplicate: bool) -> Result<data::RawTableData, TableError>;

    fn upsert_row(&self, table: &data::Table, data: &data::ObjectValues) -> Result<data::RawTableData, TableError>;

    fn update_row(&self, table: &data::Table, keys: &data::ObjectKeys, data: &data::ObjectValues, fail_on_not_found: bool) -> Result<data::RawTableData, TableError>;

    fn delete_row(&self, table: &data::Table, keys: &data::ObjectKeys, fail_on_not_found: bool) -> Result<data::RawTableData, TableError>;
}

impl<'a> TableActionFunctions for TableAction<'a> {
    fn query(&self, table: &data::Table) -> Result<data::RawTableData, TableError> {

        let query = format!("SELECT * FROM {}", &table.my_name());
        self.conn
            .exec(&query, vec![])
            .or_else(|err| Err(TableError::db_error(err)))
    }

    fn insert_row(&self, table: &data::Table, data: &data::ObjectValues, fail_on_duplicate: bool) -> Result<data::RawTableData, TableError> {

        let table_column_names = table.get_column_names();
        let raw_data = data.as_list();
        let mut results = data::RawTableData::new(vec![], table_column_names.to_owned());

        for row in raw_data {
            let column_names: Vec<String> = row.keys().map(|x| x.to_owned()).collect();
            let column_counts: Vec<String> = column_names.iter().enumerate()
                .map(|(i, _)| format!("${}", i+1))
                .collect();
            let values = row.values().map(|x| x.to_owned()).collect();
            let query = format!(
                r#"INSERT INTO "{name}" ("{columns}") VALUES ({params}) RETURNING *;"#,
                name=table.my_name(),
                columns=column_names.join(r#"", ""#),
                params=column_counts.join(r#", "#),
            );

            let new_row = self.conn
                .exec(&query, values)
                .or_else(|err| {
                    match err {
                        DbError::AlreadyExists => if !fail_on_duplicate {
                            Ok(data::RawTableData::new(vec![], table_column_names.to_owned()))
                        } else {
                            Err(TableError::db_error(err))
                        },
                        _ => Err(TableError::db_error(err)),
                    }
                })?;

            results.append(new_row)
                .or_else(|err| {
                    error!("columns names are mismatched");
                    Err(TableError::Unknown)
                })?;
        }

        Ok(results)
    }

    fn upsert_row(&self, table: &data::Table, data: &data::ObjectValues) -> Result<data::RawTableData, TableError> {
        //TODO: doing this because I want to know whether it was an insert or update so that I can put in the correct data in the transactions table
        // otherise, maybe ON CONFLICT with triggers would have been the proper choice
        self.conn
            .exec( "SELECT id FROM table WHERE id = my_id", vec![]);
        self.conn
            .exec("INSERT INTO table (value1, value2, value3) VALUES (1, 2, 3)", vec![]);
        self.conn
            .exec("UPDATE table SET value1 = 1, value2 = 2 WHERE id = my_id", vec![]);
        unimplemented!()
    }

    fn update_row(&self, table: &data::Table, keys: &data::ObjectKeys, data: &data::ObjectValues, fail_on_not_found: bool) -> Result<data::RawTableData, TableError> {

        let table_column_names = table.get_column_names();
        let raw_keys = keys.as_list();
        let raw_data = data.as_list();
        let mut results = data::RawTableData::new(vec![], table_column_names.to_owned());

        for (key, row) in raw_keys.iter().zip(raw_data) {
            let column_names: Vec<String> = row.keys().map(|x| x.to_owned()).collect();
            let key_names: Vec<String> = key.keys().map(|x| x.to_owned()).collect();

            let mut values: Vec<data::Value> = row.values().map(|x| x.to_owned()).collect();
            let key_values: Vec<data::Value> = key.values().map(|x| x.to_owned().into_value()).collect();
            values.extend(key_values);

            let val_index = 1;
            let key_index = column_names.len() + 1;

            let query = format!(
                "UPDATE {name} SET {sets} WHERE {id} RETURNING *", //"UPDATE table SET value1 = 1, value2 = 2 WHERE id = my_id"
                name=table.my_name(),
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
                            Ok(data::RawTableData::new(vec![], table_column_names.to_owned()))
                        } else {
                            Err(TableError::db_error(err))
                        },
                        _ => Err(TableError::db_error(err)),
                    }
                })?;

            results.append(new_row)
                .or_else(|err| {
                    error!("columns names are mismatched");
                    Err(TableError::Unknown)
                })?;
        }

        Ok(results)

    }

    fn delete_row(&self, table: &data::Table, keys: &data::ObjectKeys, fail_on_not_found: bool) -> Result<data::RawTableData, TableError> {

        let table_column_names = table.get_column_names();
        let raw_keys = keys.as_list();
        let mut results = data::RawTableData::new(vec![], table_column_names.to_owned());

        for key in raw_keys {
            let key_names: Vec<String> = key.keys().map(|x| x.to_owned()).collect();
            let values: Vec<data::Value> = key.values().map(|x| x.to_owned().into_value()).collect();

            let query = format!(
                "DELETE {name} WHERE {id} RETURNING *", //"DELETE table WHERE id = my_id"
                name=table.my_name(),
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
                            Ok(data::RawTableData::new(vec![], table_column_names.to_owned()))
                        } else {
                            Err(TableError::db_error(err))
                        },
                        _ => Err(TableError::db_error(err)),
                    }
                })?;

            results.append(new_row)
                .or_else(|err| {
                    error!("columns names are mismatched");
                    Err(TableError::Unknown)
                })?;
        }

        Ok(results)
    }
}