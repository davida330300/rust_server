use crate::error::SelfDefinedError;
use crate::models::book::{Book, CreateBook, UpdateBook};
use sqlx::postgres::PgPool;

pub async fn get_book_for_author_db(pool: &PgPool, author_id: i32) -> Result<Vec<Book>, SelfDefinedError> {
    let rows = sqlx::query!(
        r#"
        SELECT 
            id, 
            author_id, 
            description, 
            time, 
            name, 
            price, 
            language 
        FROM 
            book  
        WHERE 
            author_id = $1
        "#,
        author_id
    )
    .fetch_all(pool)
    .await?;
    let books: Vec<Book> = rows.into_iter().map(|row| Book {
        id: Some(row.id),
        author_id: row.author_id,
        name: row.name,
        time: row.time,
        description: row.description,
        price: row.price,
        language: row.language,
    }).collect();

    Ok(books)

}

pub async fn get_book_details_db(
    pool: &PgPool,
    author_id: i32,
    id: i32,
) -> Result<Book, SelfDefinedError> {
    let row = sqlx::query!(
        r#"SELECT author_id, id, name, time, description, price, language FROM book
            WHERE author_id = $1 and id = $2"#,
        author_id,
        id,
    )
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        let book = Book {
            id: Some(row.id),
            author_id: row.author_id,
            name: row.name,
            time: row.time,
            description: row.description,
            price: row.price,
            language: row.language,
        };
        Ok(book)
    } else {
        Err(SelfDefinedError::NotFound("book didn't found".into()))
    }
}

pub async fn post_new_book_db(
    pool: &PgPool,
    new_book: CreateBook,
) -> Result<Book, SelfDefinedError> {
    let row = sqlx::query!(
        r#"INSERT INTO book (author_id, name, description, price, language)
        VALUES ($1, $2, $3, $4, $5)
        RETURNING id, author_id, name, time, description, price, language "#,
        new_book.author_id, new_book.name, new_book.description,
        new_book.price, new_book.language,
        )
    .fetch_one(pool)
    .await?;

    let book = Book {
        id: Some(row.id),
        author_id: row.author_id,
        name: row.name,
        time: row.time,
        description: row.description,
        price: row.price,
        language: row.language,
    };

    Ok(book)
}

pub async fn delete_book_db(pool: &PgPool, author_id: i32, id: i32) -> Result<String, SelfDefinedError> {
    let book_row = sqlx::query!(
        "DELETE FROM book where author_id = $1 and id=$2",
        author_id,
        id,
    )
    .execute(pool)
    .await?;
    Ok(format!("DeletedI{:?}record", book_row))
}

pub async fn update_book_details_db(
    pool: &PgPool,
    author_id: i32,
    id: i32,
    update_book: UpdateBook,
) -> Result<Book, SelfDefinedError> {
    let current_book_row = sqlx::query!(
        r#"SELECT author_id, id, name, time, description, price, language FROM book where author_id=$1 and id=$2"#,
        author_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_err| SelfDefinedError::NotFound("book Id not found".into()))?;

    let name: String = if let name = update_book.name {
        name
    } else {
        current_book_row.name.clone()
    };
    let description: String = if let Some(description) = update_book.description {
        description
    } else {
        current_book_row.description.clone().unwrap_or_default()
    };
    let language: String = if let Some(language) = update_book.language {
        language
    } else {
        current_book_row.language.clone().unwrap_or_default()
    };
    let price: i32 = if let Some(price) = update_book.price {
        price
    } else {
        current_book_row.price.unwrap_or_default()
    };

    let updated_book_row = sqlx::query!(
        r#"UPDATE book SET name = $1, description = $2, price = $3,
            language = $4 WHERE author_id = $5 and id = $6
            RETURNING id, author_id, name, time,
            description, price, language"#,
        name,
        description,
        price,
        language,
        author_id,
        id
    )
    .fetch_one(pool)
    .await
    .map_err(|_| SelfDefinedError::NotFound("Book id not found".into()))?;

    let updated_book = Book {
        id: Some(updated_book_row.id),
        author_id: updated_book_row.author_id,
        name: updated_book_row.name,
        time: updated_book_row.time,
        description: updated_book_row.description,
        price: updated_book_row.price,
        language: updated_book_row.language,
    };

    Ok(updated_book)
}