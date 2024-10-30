use crate::db::repositories::PartRepository;
use crate::db::DbConnection;
use crate::models::Part;

fn get_test_part() -> Part {
    let part = Part {
        id: None,
        name: "Test Part".to_string(),
        link: "https://www.example.com".to_string(),
        exhibit_ids: vec![],
        notes: vec![],
    };

    part
}

#[test]
fn test_create_and_retrieve_part() {
    let db_conn = DbConnection::new_in_memory().expect("Failed to create in-memory database");
    db_conn.setup_tables().expect("Failed to setup tables");

    let part_repo = PartRepository::new(&db_conn);

    let mut part = get_test_part();

    let part_id = part_repo.create_part(&part).expect("Failed to create part");

    part.id = Some(part_id);

    let retrieved_part = part_repo
        .get_part(part_id)
        .expect("Failed to retrieve part")
        .expect("Part not found");

    assert_eq!(part, retrieved_part);
}

#[test]
fn update_part_test() {
    // Setup the database and repository
    let db_conn = DbConnection::new_in_memory().expect("Failed to create in-memory database");
    db_conn.setup_tables().expect("Failed to setup tables");

    let part_repo = PartRepository::new(&db_conn);

    // Create an part
    let mut part = get_test_part();

    let part_id = part_repo.create_part(&part).expect("Failed to create part");

    // Update the part id to match the created part
    part.id = Some(part_id);

    // Create a copy of the part
    let pre_update_part = part.clone();

    // Update the part
    part.name = "Updated Test Part".to_string();

    part_repo
        .update_part(part_id, &part)
        .expect("Failed to update part");

    // Retrieve the updated part
    let updated_part = part_repo
        .get_part(part_id)
        .expect("Failed to retrieve part")
        .expect("Part not found");

    // Check that they are the same part
    assert_eq!(updated_part.id, pre_update_part.id);

    // assert name not equal
    assert_ne!(updated_part.name, pre_update_part.name);
}

#[test]
fn delete_part_test() {
    // Setup the database and repository
    let db_conn = DbConnection::new_in_memory().expect("Failed to create in-memory database");
    db_conn.setup_tables().expect("Failed to setup tables");

    let part_repo = PartRepository::new(&db_conn);

    // Create an part
    let mut part = get_test_part();

    // Insert the exhibit into the database
    let part_id = part_repo.create_part(&part).expect("Failed to create part");

    part.id = Some(part_id);

    // Verify that the exhibit was inserted
    let retrieved_part = part_repo
        .get_part(part_id)
        .expect("Failed to retrieve part")
        .expect("Part not found");

    assert_eq!(part, retrieved_part);

    // Delete the exhibit
    part_repo
        .delete_part(part_id)
        .expect("Failed to delete part");

    // Verify that the exhibit was deleted
    let retrieved_part = part_repo
        .get_part(part_id)
        .expect("Failed to retrieve part");

    assert!(retrieved_part.is_none());
}
