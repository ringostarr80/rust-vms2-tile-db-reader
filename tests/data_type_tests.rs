use std::str::FromStr;
use vms2_tile_db_reader::data_type::DataType;

#[test]
fn test_from_str_points() {
    let result = DataType::from_str("points");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DataType::Points);
}

#[test]
fn test_from_str_lines() {
    let result = DataType::from_str("lines");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DataType::Lines);
}

#[test]
fn test_from_str_polygons() {
    let result = DataType::from_str("polygons");
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), DataType::Polygons);
}

#[test]
fn test_invalid_data_type() {
    let result = DataType::from_str("invalid");
    assert!(result.is_err());
}

#[test]
fn test_display_points() {
    let data_type = DataType::Points;
    assert_eq!(data_type.to_string(), "Points");
}

#[test]
fn test_display_lines() {
    let data_type = DataType::Lines;
    assert_eq!(data_type.to_string(), "Lines");
}

#[test]
fn test_display_polygons() {
    let data_type = DataType::Polygons;
    assert_eq!(data_type.to_string(), "Polygons");
}

#[test]
fn test_data_type_to_int_via_into() {
    let points_int: u8 = DataType::Points.into();
    let lines_int: u8 = DataType::Lines.into();
    let polygons_int: u8 = DataType::Polygons.into();

    assert_eq!(points_int, 0);
    assert_eq!(lines_int, 1);
    assert_eq!(polygons_int, 2);
}
