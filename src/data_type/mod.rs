use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq)]
pub enum DataType {
    Points,
    Lines,
    Polygons,
}

impl FromStr for DataType {
    type Err = String;

    fn from_str(input: &str) -> Result<DataType, Self::Err> {
        match input.to_lowercase().as_str() {
            "points" => Ok(DataType::Points),
            "lines" => Ok(DataType::Lines),
            "polygons" => Ok(DataType::Polygons),
            _ => Err(format!("'{}' is not a valid DataType", input)),
        }
    }
}

impl From<DataType> for u8 {
    fn from(data_type: DataType) -> Self {
        match data_type {
            DataType::Points => 0,
            DataType::Lines => 1,
            DataType::Polygons => 2,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            DataType::Points => "Points",
            DataType::Lines => "Lines",
            DataType::Polygons => "Polygons",
        };
        write!(f, "{}", name)
    }
}
