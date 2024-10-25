pub mod db_connection;

pub type Db = std::sync::Arc<tokio::sync::Mutex<db_connection::DbConnection>>;
