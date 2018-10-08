
use objekt::Clone;

use super::types::DataPoint;

#[derive(Clone)]
pub struct Row(Vec<DataPoint>);

impl Row {
    pub fn new(data: &Vec<DataPoint>) -> Self {
        Row(data.to_owned())
    }

    pub fn y(&self, i: usize) -> Option<DataPoint> {
        let Row(vector) = self;
        match vector.get(i) {
            Some(out) => Some(out.to_owned()),
            None => None,
        }
    }
}

#[derive(Clone)]
pub struct Rows(Vec<Row>);

impl Rows {
    pub fn new(rows: &Vec<Row>) -> Self {
        Rows(rows.to_owned())
    }

    pub fn x(&self, i: usize) -> Option<Row> {
        let Rows(vector) = self;
        match vector.get(i) {
            Some(out) => Some(out.to_owned()),
            None => None,
        }
    }
}