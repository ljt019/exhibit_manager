use crate::models::{FullName, Jotform, SubmissionDate};
use rocket::serde::json::serde_json::Value;
use serde::Deserialize;
use std::collections::HashMap;

const QUESTION_NAME: &str = "4";
const QUESTION_LOCATION: &str = "5";
const QUESTION_EXHIBIT_NAME: &str = "6";
const QUESTION_DESCRIPTION: &str = "7";
const QUESTION_RAW_PRIORITY: &str = "8";
const QUESTION_RAW_DEPARTMENT: &str = "9";

/// Represents an "answer" to a question in the JotForm.
///
/// This struct is used to deserialize the JSON response from the JotForm API.
/// It's used to extract the "answer" to a question, which could be a string, array of file URLs, etc.
#[derive(Debug, Deserialize)]
pub struct Answer {
    #[allow(dead_code)]
    // This is typically the "short text" of the question (e.g. "Name:" or "Description:")
    // This is not used in the current implementation, but could be useful for debugging.
    pub name: Option<String>,

    #[allow(dead_code)]
    // The question text,  "Work / Building Location:"
    // This is not used in the current implementation, but could be useful for debugging.
    pub text: Option<String>,

    // The raw "answer". Could be string, array of file URLs, etc.
    // This is the field we're interested in.
    pub answer: Option<Value>,
}

/// Represents a raw submission from the JotForm API.
///
/// This struct is used to deserialize the JSON response from the JotForm API.
/// And then has a method to convert it to our custom `Jotform` struct.
///
/// The main purpose of this is just to throw away the fields/information we don't need.
#[derive(Debug, Deserialize)]
pub struct RawSubmission {
    pub id: String,

    // The "created_at" field is your "submission date".
    #[serde(rename = "created_at")]
    pub created_at: String,

    // "answers" is a map from question ID ("1", "2", "3", ...) to an `Answer`.
    pub answers: HashMap<String, Answer>,
}

impl RawSubmission {
    /// Converts the raw submission to our custom `Jotform` struct.
    pub fn to_jotform(&self) -> Jotform {
        // Extract the "name" field (question #4) using the `extract_name` function.
        let answer_4 = self
            .answers
            .get(QUESTION_NAME)
            .expect("Question #4 (Name) is missing; did the form change?");

        let name_value = answer_4.answer.as_ref().expect("Q#4 has no `answer` value");

        let submitter_name = extract_name(name_value);

        // Extract other fields by question ID using the `get_str` helper function.
        let location = get_str(&self.answers, QUESTION_LOCATION);
        let exhibit_name = get_str(&self.answers, QUESTION_EXHIBIT_NAME);
        let description = get_str(&self.answers, QUESTION_DESCRIPTION);
        let raw_priority = get_str(&self.answers, QUESTION_RAW_PRIORITY);
        let raw_department = get_str(&self.answers, QUESTION_RAW_DEPARTMENT);

        // Convert them to the desired short strings using the other helper functions.
        let priority_level = parse_priority_level(&raw_priority);
        let department = parse_department(&raw_department);

        // Extract the date and time from the "created_at" field using `parse_submission_date`.
        let submission_date = parse_submission_date(&self.created_at);

        // Build the final Jotform struct, defaulting `status` to "Open".
        Jotform {
            id: self.id.clone(),
            submitter_name,
            created_at: submission_date,
            location,
            exhibit_name,
            description,
            priority_level,
            department,
            status: "Open".to_string(), // your default
        }
    }
}

/// Helper function to extract a string answer from the `answers` map.
fn get_str(answers: &HashMap<String, Answer>, q_id: &str) -> String {
    answers
        .get(q_id)
        .expect(&format!("Question #{} missing; did the form change?", q_id))
        .answer
        .as_ref()
        .expect("No answer value for question")
        .as_str()
        .expect("Answer was not a string")
        .trim()
        .to_string()
}

/// Helper function to extract a `FullName` from the "name" field.
/// Main use is just to split the first and last name into seperate fields.
///
/// So I don't have to do any string manipulation if i just need the first name.
///
/// This also helps with the "Kenneth" problem where spaces are added
/// around the first and last name which leaves to a wonky printout.
fn extract_name(answer_value: &Value) -> FullName {
    let obj = answer_value
        .as_object()
        .expect("Expected an object for name field");

    let first = obj
        .get("first")
        .expect("Missing 'first' key in name field")
        .as_str()
        .expect("'first' wasn't a string")
        .trim()
        .to_string();

    let last = obj
        .get("last")
        .expect("Missing 'last' key in name field")
        .as_str()
        .expect("'last' wasn't a string")
        .trim()
        .to_string();

    FullName { first, last }
}

/// Helper function to parse the raw strings into the desired short strings.
/// The long strings are just the dropdown options in the Jotform that was already created
/// by someone else.
///
/// If for some reason the jotform has any changes, these would need to change or it would
/// always return the default value of "N/A".
fn parse_department(raw_department: &str) -> String {
    match raw_department.trim() {
        "Building Maintenance/Repair request - Operations" => "Operations".to_string(),
        "Exhibit Maintenance/Repair request - Exhibits" => "Exhibits".to_string(),
        _ => "N/A".to_string(),
    }
}

/// Helper function to parse the raw strings into the desired short strings.
/// The long strings are just the dropdown options in the Jotform that was already created
/// by someone else.
///
/// If for some reason the jotform has any changes, these would need to change or it would
/// always return the default value of "N/A".
fn parse_priority_level(raw_priority: &str) -> String {
    match raw_priority.trim() {
        "High - ASAP" => "High".to_string(),
        "Low - as soon as possible" => "Low".to_string(),
        "Medium - within 1-2 weeks" => "Medium".to_string(),
        _ => "N/A".to_string(),
    }
}

/// Helper function to parse the raw submission date into a `SubmissionDate` struct.
/// This is just to split the date and time into seperate fields.
///
/// This way either the date or time can be accessed independently
/// and without any string manipulation.
///
/// This will usually only be used to access the date but still useful.
fn parse_submission_date(raw_date: &str) -> SubmissionDate {
    let date = raw_date.split(' ').next().unwrap().to_string();
    let time = raw_date.split(' ').last().unwrap().to_string();

    SubmissionDate { date, time }
}
