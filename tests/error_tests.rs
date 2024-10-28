use vms2_tile_db_reader::error::Error;

#[test]
fn test_error_display_implementation() {
    let result = Error::DbError {
        message: String::from("some test error"),
        source: None,
    };
    assert_eq!("Database error: some test error", format!("{}", result));
}
