

//TODO: Add output format: indexed, rows (default), flat rows, columns
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TableDataFormat {
    Rows,
    FlatRows,
}