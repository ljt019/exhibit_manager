use crate::db::DbConnection;
use crate::models::exhibit::Exhibit;
use crate::repositories::exhibit_repository::ExhibitRepository;

fn get_test_exhibit() -> Exhibit {
    let exhibit = Exhibit {
        id: None,
        name: "Test Exhibit".to_string(),
        cluster: "Test Cluster".to_string(),
        location: "Test Location".to_string(),
        status: "Test Status".to_string(),
        image_url: "Test Image URL".to_string(),
        sponsor_name: None,
        sponsor_start_date: None,
        sponsor_end_date: None,
        part_ids: vec![],
        notes: vec![],
    };

    exhibit
}

#[test]
fn test_create_and_retrieve_exhibit() {
    let db_conn = DbConnection::new_in_memory().expect("Failed to create in-memory database");
    db_conn.setup_tables().expect("Failed to setup tables");

    let exhibit_repo = ExhibitRepository::new(&db_conn);

    let mut exhibit = get_test_exhibit();

    let exhibit_id = exhibit_repo
        .create_exhibit(&exhibit)
        .expect("Failed to create exhibit");

    exhibit.id = Some(exhibit_id);

    let retrieved_exhibit = exhibit_repo
        .get_exhibit(exhibit_id)
        .expect("Failed to retrieve exhibit")
        .expect("Exhibit not found");

    assert_eq!(exhibit, retrieved_exhibit);
}

#[test]
fn update_exhibit_test() {
    // Setup the database and repository
    let db_conn = DbConnection::new_in_memory().expect("Failed to create in-memory database");
    db_conn.setup_tables().expect("Failed to setup tables");

    let exhibit_repo = ExhibitRepository::new(&db_conn);

    // Create an exhibit
    let mut exhibit = get_test_exhibit();

    let exhibit_id = exhibit_repo
        .create_exhibit(&exhibit)
        .expect("Failed to create exhibit");

    // Update the exhibit id to match the created exhibit
    exhibit.id = Some(exhibit_id);

    // Create a copy of the exhibit
    let pre_update_exhibit = exhibit.clone();

    // Update the exhibit
    exhibit.name = "Updated Test Exhibit".to_string();

    exhibit_repo
        .update_exhibit(exhibit_id, &exhibit)
        .expect("Failed to update exhibit");

    // Retrieve the updated exhibit
    let updated_exhibit = exhibit_repo
        .get_exhibit(exhibit_id)
        .expect("Failed to retrieve exhibit")
        .expect("Exhibit not found");

    // Check that they are the same exhibit
    assert_eq!(updated_exhibit.id, pre_update_exhibit.id);

    // assert name not equal
    assert_ne!(updated_exhibit.name, pre_update_exhibit.name);
}

#[test]
fn delete_exhibit_test() {
    // Setup the database and repository
    let db_conn = DbConnection::new_in_memory().expect("Failed to create in-memory database");
    db_conn.setup_tables().expect("Failed to setup tables");

    let exhibit_repo = ExhibitRepository::new(&db_conn);

    let mut exhibit = get_test_exhibit();

    // Insert the exhibit into the database
    let exhibit_id = exhibit_repo
        .create_exhibit(&exhibit)
        .expect("Failed to create exhibit");

    exhibit.id = Some(exhibit_id);

    // Verify that the exhibit was inserted
    let retrieved_exhibit = exhibit_repo
        .get_exhibit(exhibit_id)
        .expect("Failed to retrieve exhibit")
        .expect("Exhibit not found");

    assert_eq!(exhibit, retrieved_exhibit);

    // Delete the exhibit
    exhibit_repo
        .delete_exhibit(exhibit_id)
        .expect("Failed to delete exhibit");

    // Verify that the exhibit was deleted
    let retrieved_exhibit = exhibit_repo
        .get_exhibit(exhibit_id)
        .expect("Failed to retrieve exhibit");

    assert!(retrieved_exhibit.is_none());
}
