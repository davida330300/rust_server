use chrono::NaiveDateTime;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::io;

#[derive(Debug)]
pub struct Book {
    pub id: i32,
    pub author_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("not database url in .env");
    let db_pool = PgPoolOptions::new()
        .connect(&database_url)
        .await
        .unwrap();
    let book_rows = sqlx::query!(
        r#"select id, author_id, name, time from book where id = $1"#,
        1
    )
    .fetch_all(&db_pool)
    .await
    .unwrap();

    let mut book_list = vec![];
    for row in book_rows {
        book_list.push(Book {
            id: row.id,
            author_id: row.author_id,
            name: row.name,
            time: Some(chrono::NaiveDateTime::from(row.time.unwrap())),
        })
    }
    println!("book = {:?}", book_list);

    Ok(())
}