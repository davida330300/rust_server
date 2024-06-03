use std::fmt::Debug;

use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use crate::error::SelfDefinedError;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Book {
    pub author_id: i32,
    pub id: Option<i32>,
    pub name: String,
    pub time: Option<NaiveDateTime>,

    pub description: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
}

#[derive(Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct CreateBook {
    pub author_id: i32,
    pub name: String,

    pub description: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateBook {
    pub name: String,

    pub description: Option<String>,
    pub price: Option<i32>,
    pub language: Option<String>,
}

impl From<web::Json<Book>> for Book {
    fn from(book: web::Json<Book>) -> Self {
        Book {
            author_id: book.author_id,
            id: book.id,
            name: book.name.clone(),
            time: book.time,

            description: book.description.clone(),
            price: book.price,
            language: book.language.clone(),
        }
    }
}

impl TryFrom<web::Json<CreateBook>> for CreateBook {
    type Error =  SelfDefinedError;

    fn try_from(book: web::Json<CreateBook>) -> Result<Self, Self::Error> {
        Ok(CreateBook {
            author_id: book.author_id,
            name: book.name.clone(),
            description: book.description.clone(),
            price: book.price,
            language: book.language.clone(),
        })
    }
}

impl From<web::Json<UpdateBook>> for UpdateBook {
    fn from(book: web::Json<UpdateBook>) -> Self {
        UpdateBook {
            name: book.name.clone(),
            description: book.description.clone(),
            price: book.price,
            language: book.language.clone(),
        }
    }
}