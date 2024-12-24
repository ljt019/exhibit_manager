use crate::models::{Jotform, SubmissionDate};
use rocket::serde::json::serde_json::Value;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Answer {
    // This is typically the "short text" of the question (e.g. "Name:" or "Description:")
    pub name: Option<String>,

    // The question text, e.g. "Work / Building Location:"
    pub text: Option<String>,

    // The raw "answer". Could be string, array of file URLs, etc.
    pub answer: Option<Value>,

    // Some fields from JotForm use "prettyFormat" (e.g. combined first+last name).
    #[serde(rename = "prettyFormat")]
    pub pretty_format: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct RawSubmission {
    pub id: String,
    #[serde(rename = "form_id")]
    pub form_id: String,

    // The "created_at" field is your "submission date".
    #[serde(rename = "created_at")]
    pub created_at: String,

    // "answers" is a map from question ID ("1", "2", "3", ...) to an `Answer`.
    pub answers: HashMap<String, Answer>,
}

impl RawSubmission {
    pub fn get_answer(&self, question_id: &str) -> Option<&Answer> {
        self.answers.get(question_id)
    }

    fn get_str_from_answer(&self, question_id: &str) -> String {
        self.get_answer(question_id)
            .and_then(|ans| ans.answer.as_ref())
            .and_then(|val| val.as_str()) // Convert JSON value to &str if it’s indeed a string
            .unwrap_or("")
            .trim()
            .to_string()
    }

    pub fn to_jotform(&self) -> Jotform {
        // 1) Extract the "name" field (question #4).
        //    We assume it always exists, is always an object, and contains "first" & "last".
        let answer_4 = self
            .answers
            .get("4")
            .expect("Question #4 is missing; did the form change?");

        let name_value = answer_4.answer.as_ref().expect("Q#4 has no `answer` value");

        let name_obj = name_value
            .as_object()
            .expect("Expected Q#4 `answer` to be a JSON object with `first`/`last`");

        let first = name_obj
            .get("first")
            .expect("Missing 'first' key in Q#4 answer object")
            .as_str()
            .expect("'first' wasn't a string")
            .trim();

        let last = name_obj
            .get("last")
            .expect("Missing 'last' key in Q#4 answer object")
            .as_str()
            .expect("'last' wasn't a string")
            .trim();

        let submitter_name = format!("{} {}", first, last).trim().to_string();

        // 2) Helper to get a string from the "answer" field of any question.
        let get_str = |q_id: &str| {
            self.answers
                .get(q_id)
                .expect(&format!("Question #{} missing; did the form change?", q_id))
                .answer
                .as_ref()
                .expect("No answer value for question")
                .as_str()
                .expect("Answer was not a string")
                .trim()
                .to_string()
        };

        // 3) Extract other fields by question ID.
        let location = get_str("5");
        let exhibit_name = get_str("6");
        let description = get_str("7");
        let raw_priority = get_str("8");
        let raw_department = get_str("9");

        // 4) Convert them to your desired short strings, e.g. "High", "Operations", etc.
        let priority_level = match raw_priority {
            ref s if s == "High - ASAP" => "High".to_string(),
            ref s if s == "Low - as soon as possible" => "Low".to_string(),
            ref s if s == "Medium - within 1-2 weeks" => "Medium".to_string(),
            _ => "N/A".to_string(),
        };

        let department = match raw_department.as_str() {
            "Building Maintenance/Repair request - Operations" => "Operations".to_string(),
            "Exhibit Maintenance/Repair request - Exhibits" => "Exhibits".to_string(),
            _ => "N/A".to_string(),
        };

        let submission_date = SubmissionDate {
            date: self.created_at.split(' ').next().unwrap().to_string(),
            time: self.created_at.split(' ').last().unwrap().to_string(),
        };

        // 5) Build the final Jotform struct, defaulting `status` to "Open".
        Jotform {
            id: self.id.clone(),
            submitter_name,
            submission_date: submission_date,
            location,
            exhibit_name,
            description,
            priority_level,
            department,
            status: "Open".to_string(), // your default
        }
    }

    fn extract_name(answer_value: &Value) -> String {
        let obj = answer_value
            .as_object()
            .expect("Expected an object for name field");

        // Unwrap "first" and "last" (both guaranteed to be JSON strings),
        // trim trailing spaces, and combine with a single space in between.
        // thanks kenneth...
        let first = obj
            .get("first")
            .expect("Missing 'first' key in name field")
            .as_str()
            .expect("'first' wasn't a string")
            .trim();

        let last = obj
            .get("last")
            .expect("Missing 'last' key in name field")
            .as_str()
            .expect("'last' wasn't a string")
            .trim();

        format!("{} {}", first, last).trim().to_string()
    }

    fn parse_department(raw_department: &str) -> &str {
        match raw_department.trim() {
            "Building Maintenance/Repair request - Operations" => "Operations",
            "Exhibit Maintenance/Repair request - Exhibits" => "Exhibits",
            _ => "N/A",
        }
    }

    fn parse_priority_level(raw_priority: &str) -> &str {
        match raw_priority.trim() {
            "High - ASAP" => "High",
            "Low - as soon as possible" => "Low",
            "Medium - within 1-2 weeks" => "Medium",
            _ => "N/A",
        }
    }
}
