use crate::models::Exhibit;
use rand::prelude::SliceRandom;
use rocket::serde;

const EXHIBIT_DATA: &'static str = include_str!("../exhibit_dev_data/exhibits.json");

#[derive(serde::Deserialize)]
#[serde(rename_all = "PascalCase")]
struct DummyExhibitData {
    exhibit_name: String,
    _exhibit_description: String,
    building_location: String,
    current_status: String,
    cluster: String,
    _notes: String,
}

pub fn get_random_dummy_exhibit() -> Exhibit {
    let exhibits: Vec<DummyExhibitData> = serde::json::serde_json::from_str(EXHIBIT_DATA).unwrap();
    let dummy_exhibit_data = exhibits.choose(&mut rand::thread_rng()).unwrap();

    let exhibit = Exhibit {
        id: rand::random::<i64>(),
        name: dummy_exhibit_data.exhibit_name.clone(),
        cluster: dummy_exhibit_data.cluster.clone(),
        location: dummy_exhibit_data.building_location.clone(),
        status: dummy_exhibit_data.current_status.clone(),
        image_url: format!("https://picsum.photos/seed/picsum/200/300"),
        sponsor_name: None,
        sponsor_start_date: None,
        sponsor_end_date: None,
        part_ids: vec![],
        notes: vec![],
    };

    exhibit
}
