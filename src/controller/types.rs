
use serde_json;

#[derive(Deserialize, Serialize, Clone)]
pub enum DataPoint {
    Integer(i64),
    UnsignedInteger(u64),
    FloatingPoint(f64),
    String(String),
    Json(serde_json::Value),
    Null,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum DataType {
    DecimalType {
        precision: u8,
        scale: u8,
    },
    TimestampType,
    DateType,
    SerialType,
    IntegerType,
    StringType,
    JsonType,
}

pub trait DataTypeChecker {
    fn check_type(&self, data_point: &DataPoint) -> bool;
}

impl DataTypeChecker for DataType {
    fn check_type(&self, data_point: &DataPoint) -> bool {
        match self {
            DataType::SerialType => match data_point {
                DataPoint::Integer(x) => true,
                _ => false,
            },
            DataType::DecimalType { precision, scale } => match data_point {
                DataPoint::Integer(x) => true,
                _ => false,
            },
            DataType::StringType => match data_point {
                DataPoint::String(x) => true,
                _ => false,
            }
            DataType::TimestampType => false,
            DataType::DateType => false,
            DataType::IntegerType => false,
            DataType::JsonType => false,
        }
    }
}