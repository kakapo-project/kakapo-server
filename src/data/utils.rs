

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnNotFound {
    Ignore,
    Fail
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OnDuplicate {
    Ignore,
    Fail,
    Update,
}


//TODO: Add output format: indexed, rows (default), flat rows, columns, schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableDataFormat {
    Rows,
    FlatRows,
}