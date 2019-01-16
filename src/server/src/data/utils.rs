

#[derive(Debug)]
pub enum OnNotFound {
    Ignore,
    Fail
}

#[derive(Debug)]
pub enum OnDuplicate {
    Ignore,
    Fail,
    Update,
}


//TODO: Add output format: indexed, rows (default), flat rows, columns, schema
#[derive(Clone, Copy, Debug)]
pub enum TableDataFormat {
    Rows,
    FlatRows,
}