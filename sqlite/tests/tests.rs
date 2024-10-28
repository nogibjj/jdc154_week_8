use sqlite::{extract, query, transform_load};

#[test]
fn test_extract() {
    let url =
        "https://github.com/fivethirtyeight/data/raw/refs/heads/master/nfl-wide-receivers/advanced-historical.csv";
    let file_path = "data/nfl-wide-receivers.csv";
    let directory = "data";

    extract(url, file_path, directory);

    assert!(std::fs::metadata(file_path).is_ok());
}

#[test]
fn test_transform_load() {
    let dataset = "data/nfl-wide-receivers.csv";
    let result = transform_load(dataset);

    assert_eq!(result.unwrap(), "nfl_Receivers.db");
}

#[test]
fn test_query() {
    // Execute a SELECT query
    let select_query = "SELECT * FROM nfl_Receivers WHERE id = 1;";
    let result = query(select_query);

    assert!(result.is_ok());
}