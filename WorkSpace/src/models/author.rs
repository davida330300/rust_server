use std::fmt::Debug;

use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use crate::error::SelfDefinedError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub picture_url: String,
    pub profile: String, 
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateAuthor {
    pub name: String,
    pub picture_url: String,
    pub profile: String, 
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateAuthor {
    pub name: String,
    pub picture_url: String,
    pub profile: String, 
}

impl From<web::Json<CreateAuthor>> for CreateAuthor {
    fn from(new_author: web::Json<CreateAuthor>) -> Self {
        CreateAuthor {
            name: new_author.name.clone(),
            picture_url: new_author.picture_url.clone(),
            profile: new_author.profile.clone(),
        }
    }
}

impl From<web::Json<UpdateAuthor>> for UpdateAuthor {
    fn from(update_author: web::Json<UpdateAuthor>) -> Self {
        UpdateAuthor {
            name: update_author.name.clone(),
            picture_url: update_author.picture_url.clone(),
            profile: update_author.profile.clone(),
        }
    }
}