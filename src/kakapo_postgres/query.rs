
use kakapo_postgres::database::error::DbError;
use kakapo_postgres::database::DatabaseFunctions;
use data;

/*

impl QueryError {
    pub fn db_error(err: DbError) -> Self {
        unimplemented!()
    }
}

impl QueryActionFunctions<ActionState> for QueryAction {
    fn run_query(state: &ActionState, query: &data::DataQueryEntity, params: &serde_json::Value, format: &serde_json::Value) -> Result<serde_json::Value, QueryError>  {
        /* TODO:...
        let params = serde_json::from_value(params)
            .map_error(...)?;
        let username = state.get_authorization().username();
        let db_params = params.value_list();

        if let Some(db_user) = username.to_owned() {
            state
                .get_database()
                .exec("SET SESSION AUTHORIZATION $1", vec![Value::String(db_user)])
                .or_else(|err| Err(QueryError::db_error(err)))?;
        }

        let result = state
            .get_database()
            .exec(&query.statement, db_params)
            .or_else(|err| Err(QueryError::db_error(err)))?;

        if let Some(db_user) = username {
            state
                .get_database()
                .exec("RESET SESSION AUTHORIZATION", vec![])
                .or_else(|err| Err(QueryError::db_error(err)))?;
        }

        Ok(result)

        //...
         .and_then(|table_data| {
            Ok(table_data.format_with(&self.format))
        })

        serd_json::to_value(...)
        */
        unimplemented!()
    }
}
*/