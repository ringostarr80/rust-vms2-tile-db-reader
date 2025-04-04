use std::path::Path;
use vms2_tile_db_reader::data_type::DataType;
use vms2_tile_db_reader::sources::{SQLite, Source};

#[test]
fn test_get_data_building_polygons() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(
            34686,
            21566,
            16,
            String::from("building"),
            Some(String::from("*")),
            Some(DataType::Polygons),
        )
        .unwrap();

    assert!(tile_data.len() >= 4);
}

#[test]
fn test_get_data_city_points() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(
            34686,
            21566,
            16,
            String::from("place"),
            Some(String::from("city")),
            Some(DataType::Points),
        )
        .unwrap();

    assert!(tile_data.len() >= 4);
}

#[test]
fn test_get_land_data() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(34686, 21566, 16, String::from("land"), None, None)
        .unwrap();

    assert!(tile_data.len() >= 4);
}

#[test]
fn test_get_terrain_data() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(34686, 21566, 16, String::from("terrain"), None, None)
        .unwrap();

    assert!(tile_data.len() >= 4);
}

#[test]
fn test_get_blue_marble_data() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(34686, 21566, 16, String::from("blue_marble"), None, None)
        .unwrap();

    assert!(tile_data.len() >= 4);
}

#[test]
fn test_get_raw_data_where_zoom_is_zero() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(0, 0, 0, String::from("land"), None, None)
        .unwrap();

    assert!(tile_data.len() >= 4);
}

#[test]
fn test_get_data_from_internal_multi_tile_query() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(1083, 673, 12, String::from("land"), None, None)
        .unwrap();

    assert!(tile_data.len() >= 4);
}

#[test]
fn test_db_file_does_not_exists() {
    let tile_db = SQLite::new(Path::new("./tests/data/invalid.sqlite"));
    assert!(tile_db.is_err());
}

#[test]
fn test_behaviour_when_max_tile_zoom_minus_z_is_negative() {
    let tile_db = SQLite::new(Path::new("./tests/data/braunschweig.sqlite")).unwrap();
    let tile_data = tile_db
        .get_raw_data(69372, 43129, 17, String::from("highway"), Some(String::from("pedestrian")), Some(DataType::Polygons))
        .unwrap();

    assert!(tile_data.len() >= 4);
}
