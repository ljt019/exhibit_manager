// src/routes/mod.rs

mod bug_report_routes;
mod exhibit_routes;
mod part_routes;

pub use bug_report_routes::bug_report_routes;
pub use exhibit_routes::exhibit_routes;
pub use part_routes::part_routes;
