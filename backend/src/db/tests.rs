#[test]
fn test_setup_tables() {
    let conn = super::connection::DbConnection::new_in_memory().unwrap();
    conn.setup_tables().unwrap();

    // Check that tables have been created
    let tables = conn
        .0
        .prepare("SELECT name FROM sqlite_master WHERE type='table'") // Query for table names
        .unwrap()
        .query_map([], |row| row.get(0)) // Extract table names
        .unwrap()
        .collect::<Result<Vec<String>, _>>()
        .unwrap();

    assert_eq!(
        tables,
        vec![
            "exhibits",
            "parts",
            "exhibit_parts",
            "exhibit_notes",
            "part_notes"
        ]
    );
}
