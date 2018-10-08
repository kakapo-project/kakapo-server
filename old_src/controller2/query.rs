
use super::types::DataPoint;

#[derive(Clone)]
pub enum OrderType {
    Ascending,
    Descending,
}

#[derive(Clone)]
pub enum GetQuery {
    All,
    Filter {
        where_clause: Option<(String, DataPoint)>,
        order_by: Option<(String, OrderType)>,
    }
}

impl GetQuery {
    pub fn all() -> Self {
        GetQuery::All
    }

    pub fn new() -> Self {
        GetQuery::Filter {
            where_clause: None,
            order_by: None,
        }
    }

    pub fn column_equals(&self, column_name: &str, data_point: &DataPoint) -> Self {
        let new_where_clause = Some((column_name.to_owned(), data_point.to_owned()));
        match self {
            GetQuery::Filter { order_by, .. } => GetQuery::Filter {
                where_clause: new_where_clause,
                order_by: order_by.to_owned(),
            },
            _ => GetQuery::Filter {
                where_clause: new_where_clause,
                order_by: None
            },
        }
    }

    pub fn order_by(&self, column_name: &str, order_type: &OrderType) -> Self {
        let new_order_by = Some((column_name.to_owned(), order_type.to_owned()));
        match self {
            GetQuery::Filter { where_clause, .. } => GetQuery::Filter {
                where_clause: where_clause.to_owned(),
                order_by: new_order_by,
            },
            _ => GetQuery::Filter {
                where_clause: None,
                order_by: new_order_by,
            },
        }
    }
}

pub trait CreateQuery {

}

pub trait UpdateQuery {

}

pub trait DeleteQuery {

}
