use std::convert::TryInto;

#[derive(Debug)]
pub struct Field {
    pub name: String,
    pub length: usize,
    pub decimal_precision: u8,
    pub flag: u8,
    pub field_type: FieldType,
    pub is_primary: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    RowNumber(u32), // meta field
    Deleted(bool),  // meta field
    Character(String),
    Currency(String),
    Numeric(f32),
    Float(f32),
    Date(String),
    DateTime(String),
    Double(f32),
    Integer(i32),
    Logical(bool),
    Memo(usize),
    Unknown,
}

#[derive(Debug, PartialEq)]
pub enum FieldType {
    Character,
    Currency,
    Numeric,
    Float,
    Date,
    DateTime,
    Double,
    Integer,
    Logical,
    Memo,
    Unknown,
}

impl FieldType {
    pub fn from_char(c: char) -> FieldType {
        match c {
            'C' => FieldType::Character,
            'Y' => FieldType::Currency,
            'N' => FieldType::Numeric,
            'F' => FieldType::Float,
            'D' => FieldType::Date,
            'T' => FieldType::DateTime,
            'B' => FieldType::Double,
            'I' => FieldType::Integer,
            'L' => FieldType::Logical,
            'M' => FieldType::Memo,
            _ => FieldType::Unknown,
        }
    }

    pub fn to_value(&self, data: &[u8]) -> Value {
        match self {
            FieldType::Character | FieldType::Currency | FieldType::Date | FieldType::DateTime => {
                let trimmed = String::from(
                    std::str::from_utf8(data)
                        .expect("unable to field data into str")
                        .trim(),
                );
                match self {
                    FieldType::Character => Value::Character(trimmed),
                    FieldType::Currency => Value::Currency(trimmed),
                    FieldType::Date => Value::Date(trimmed),
                    FieldType::DateTime => Value::DateTime(trimmed),
                    _ => unreachable!(),
                }
            }
            FieldType::Numeric => {
                let null_value = std::str::from_utf8(&data[0..1])
                    .expect("unable to parse default numeric value");
                let trimmed = std::str::from_utf8(&data[1..])
                    .expect("unable to field data into str")
                    .trim();
                let value = if trimmed.len() > 0 {
                    trimmed
                        .parse()
                        .expect("failed to parse string into numeric")
                } else {
                    null_value.parse().unwrap_or(0f32)
                };
                Value::Numeric(value)
            }
            FieldType::Integer => Value::Integer(i32::from_le_bytes(
                data.try_into().expect("unable to parse integer column"),
            )),
            FieldType::Logical => Value::Logical(match char::from(data[0]) {
                '1' | 'T' | 't' | 'Y' | 'y' => true,
                _ => false,
            }),
            FieldType::Memo => Value::Memo(
                std::str::from_utf8(data)
                    .expect("unable to field data into str")
                    .trim()
                    .parse()
                    .expect("unable to parse memo index"),
            ),
            _ => {
                println!("Field type: {:?}", self);
                println!("Field data: {:?}", data);
                unreachable!();
            }
        }
    }
}
