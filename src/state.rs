use crate::models::book::Book;
use std::sync::Mutex;
use sqlx::postgres::PgPool;

pub struct AppState {
    pub health_check_response: String,
    pub visit_count: Mutex<u32>,
    //pub book: Mutex<Vec<Book>>,
    pub db: PgPool,
}