use sqlx::postgres::PgPool;
use crate::error::SelfDefinedError;
use crate::models::author::{self, Author, CreateAuthor, UpdateAuthor};

// pub async fn get_all_author_db(pool: &PgPool) -> Result<Vec<Author>, SelfDefinedError> {
//     let row = sqlx::query!("SELECT id, name, picture_url, profile FROM author")
//             .fetch_all(pool)
//             .await?;

//     let author: Vec<Author> = row.iter().
// }

pub async fn get_all_author_db(pool: &PgPool) -> Result<Vec<Author>, SelfDefinedError> {
    let rows = sqlx::query!("SELECT id, name, picture_url, profile FROM author")
        .fetch_all(pool)
        .await?;

    let authors: Vec<Author> = rows
        .iter()
        .map(|r| Author {
            id: r.id,
            name: r.name.clone(),
            picture_url: r.picture_url.clone(),
            profile: r.profile.clone(),
        })
        .collect();
    match authors.len() {
        0 => Err(SelfDefinedError::NotFound("No Auhtor found".into())),
        _ => Ok(authors),
    }
}

pub async fn get_author_detail_db(pool: &PgPool, author_id: i32) -> Result<Author, SelfDefinedError> {
    let row = sqlx::query!(
        "SELECT id, name, picture_url, profile FROM author WHERE id = $1",
        author_id,
    )
    .fetch_one(pool)
    .await
    .map(|r| Author {
        id: r.id,
        name: r.name,
        picture_url: r.picture_url,
        profile: r.profile,
    }) 
    .map_err(|_err| SelfDefinedError::NotFound("Author id not found".into()))?;
    Ok(row)
}


pub async fn post_new_author_db(
    pool: &PgPool,
    new_author: CreateAuthor,
) -> Result<Author, SelfDefinedError> {
    let row = sqlx::query!(
        "INSERT INTO author (name, picture_url, profile)
        VALUES ($1, $2, $3) RETURNING id, name, picture_url, profile",
        new_author.name,
        new_author.picture_url,
        new_author.profile
    ).fetch_one(pool)
        .await?;

    Ok(Author {
        id: row.id,
        name: row.name,
        picture_url: row.picture_url,
        profile: row.profile,
    })
}

pub async fn update_author_details_db(pool: &PgPool, author_id: i32, update_author: UpdateAuthor) -> Result<Author, SelfDefinedError> {
    let row = sqlx::query!(
        "SELECT id, name, picture_url, profile FROM author WHERE id = $1",
        author_id,
    ).fetch_one(pool)
        .await
        .map_err(|_err| SelfDefinedError::NotFound("Author id not found".into()))?;

    let temp = Author {
        id: row.id,
        name: if let Some(name) = Some(update_author.name) {
            name
        } else {
            row.name
        },
        picture_url: if let Some(picture_url) = Some(update_author.picture_url) {
            picture_url
        } else {
            row.picture_url
        },
        profile: if let Some(profile) = Some(update_author.profile) {
            profile
        } else {
            row.profile
        },
    };
    let updated_row = sqlx::query!(
        "UPDATE author SET name = $1, picture_url = $2, profile = $3 WHERE id = $4 \
        RETURNING id, name, picture_url, profile",
        temp.name,
        temp.picture_url,
        temp.profile,
        author_id,
    )
        .fetch_one(pool)
        .await
        .map(|r| Author {
            id: r.id,
            name: r.name,
            picture_url: r.picture_url,
            profile: r.profile,
        })
        .map_err(|_err| SelfDefinedError::NotFound("author id not found".into()))?;

    Ok(updated_row)
} 

pub async fn delete_author_db(
    pool: &PgPool,
    author_id: i32,
) -> Result<String, SelfDefinedError> {
    let row = sqlx::query!("DELETE FROM author WHERE id = $1", author_id)
        .execute(pool)
        .await
        .map_err(|_err| SelfDefinedError::NotFound("Unable to delete author".into()))?;

    Ok(format!("Deleted {:?} record", row))
}