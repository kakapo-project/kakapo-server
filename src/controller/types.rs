
#[derive(Clone)]
pub struct Identifier(String);

/// Identifier
impl Identifier {
    pub fn new(name: &str) -> Self {
        Identifier(name.to_owned())
    }

    pub fn get_name(&self) -> String {
        let Identifier(name) = self;
        name.to_owned()
    }
}

#[derive(Clone)]
pub enum DataPoint {
    Integer(i64),
    UnsignedInteger(u64),
    FloatingPoint(f64),
    String(String),
    Null,
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Column {
    name: Identifier,
    data_type: DataType,
}

impl Column {
    pub fn new(name: &str, data_type: &DataType) -> Self {
        Column {
            name: Identifier::new(name),
            data_type: data_type.to_owned()
        }
    }
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