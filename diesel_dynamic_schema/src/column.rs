use diesel::backend::Backend;
use diesel::expression::{AppearsOnTable, Expression, NonAggregate, SelectableExpression};
use diesel::prelude::*;
use diesel::query_builder::*;
use std::borrow::Borrow;
use std::marker::PhantomData;

use diesel::deserialize::FromSql;
use diesel::deserialize::FromSqlRow;
use diesel::deserialize::Result;
use diesel::sql_types::*;
use diesel::row::Row;
use diesel::pg::Pg;

use serde_json;

#[derive(Debug, Clone, Copy)]
pub struct Column<T, U, ST> {
    table: T,
    name: U,
    _sql_type: PhantomData<ST>,
}

impl<T, U, ST> Column<T, U, ST> {
    pub(crate) fn new(table: T, name: U) -> Self {
        Self {
            table,
            name: name,
            _sql_type: PhantomData,
        }
    }
}

pub struct VecColumn<T, U, ST>(Vec<Column<T, U, ST>>);

impl<T, U, ST> VecColumn<T, U, ST> {
    pub fn new(vec: Vec<Column<T, U, ST>>) -> Self {
        VecColumn(vec)
    }
}

impl<T, U, ST> QueryId for Column<T, U, ST> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}

impl<T, U, ST> QueryId for VecColumn<T, U, ST> {
    type QueryId = ();
    const HAS_STATIC_QUERY_ID: bool = false;
}

impl<T, U, ST, QS> SelectableExpression<QS> for Column<T, U, ST> {}

impl<T, U, ST, QS> SelectableExpression<QS> for VecColumn<T, U, ST> {}

impl<T, U, ST, QS> AppearsOnTable<QS> for Column<T, U, ST> {}

impl<T, U, ST, QS> AppearsOnTable<QS> for VecColumn<T, U, ST> {}

impl<T, U, ST> Expression for Column<T, U, ST> {
    type SqlType = ST;
}

impl<T, U, ST> Expression for VecColumn<T, U, ST> {
    type SqlType = ST; //TODO: is this right??
}

impl<T, U, ST> NonAggregate for Column<T, U, ST> {}

impl<T, U, ST, DB> QueryFragment<DB> for Column<T, U, ST>
where
    DB: Backend,
    T: QueryFragment<DB>,
    U: Borrow<str>,
{
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        out.unsafe_to_cache_prepared();
        self.table.walk_ast(out.reborrow())?;
        out.push_sql(".");
        out.push_identifier(self.name.borrow())?;
        Ok(())
    }
}

impl<T, U, ST, DB> QueryFragment<DB> for VecColumn<T, U, ST>
where
    DB: Backend,
    T: QueryFragment<DB>,
    U: Borrow<str>,
{
    fn walk_ast(&self, mut out: AstPass<DB>) -> QueryResult<()> {
        let VecColumn(iterator) = &self;
        let mut is_first = true;
        for item in iterator {
            if is_first {
                is_first = false;
            } else {
                out.push_sql(", ");
            }

            out.unsafe_to_cache_prepared();
            item.table.walk_ast(out.reborrow())?;
            out.push_sql(".");
            out.push_identifier(item.name.borrow())?;


        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ValueList<T>(Vec<T>);

#[derive(Debug, Clone)]
pub enum DynamicValueType {
    Text,
    Integer,
    Json,
}

#[derive(Debug, Clone)]
pub enum DynamicValue {
    Text(String),
    Integer(i32),
    Json(serde_json::Value),
}

impl DynamicValueType {
    pub fn decode(&self, bytes: Option<&[u8]>) -> Result<DynamicValue> {
        let result = match self {
            DynamicValueType::Integer => DynamicValue::Integer(
                <i32 as FromSql<Integer, Pg>>::from_sql(bytes)?
            ),
            DynamicValueType::Text => DynamicValue::Text(
                <String as FromSql<Text, Pg>>::from_sql(bytes)?
            ),
            DynamicValueType::Json => DynamicValue::Json(
                <serde_json::Value as FromSql<Json, Pg>>::from_sql(bytes)?
            ),
        };

        Ok(result)
    }
}

impl<T> ValueList<T> {
    pub fn new(vec: Vec<T>) -> Self {
        ValueList(vec)
    }

    pub fn to_vector(self) -> Vec<T> {
        let ValueList(vec) = self;
        vec
    }
}

impl ValueList<Vec<u8>> {
    pub fn decode(&self, types: &Vec<DynamicValueType>) -> Result<Vec<DynamicValue>> {
        types.iter()
            .zip(&self.0)
            .map(|(dyn_type, binary_object)| {
                dyn_type.decode(Some(&binary_object))
            })
            .collect()
    }
}

impl<U, ST> Queryable<ST, Pg> for ValueList<U>
where
    U: FromSql<ST, Pg>
{
    type Row = ValueList<U>;
    fn build(row: Self::Row) -> Self {
        row
    }
}

impl<U, ST> FromSqlRow<ST, Pg> for ValueList<U>
where
    U: FromSql<ST, Pg>,
{
    const FIELDS_NEEDED: usize = 1; //TODO: not really 1, just need to set something. Shouldn't cause any issues unless it's Sqlite

    fn build_from_row<T: Row<Pg>>(row: &mut T) -> Result<Self> {
        let mut results: Vec<U> = vec![];
        while !row.next_is_null(1) {
            let a: U = FromSql::<ST, Pg>::from_sql(row.take())?;
            results.push(a);
        }
        Ok(ValueList::new(results))
    }
}
