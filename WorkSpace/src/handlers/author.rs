use crate::models::author::{CreateAuthor, UpdateAuthor};
use crate::state::AppState;
use actix_web::{HttpResponse, web};
use crate::db_access::author::{self, *};
use crate::error::SelfDefinedError;


pub  async fn get_all_author(app_state: web::Data<AppState>) -> Result<HttpResponse, SelfDefinedError> {
    get_all_author_db(&app_state.db)
    .await
    .map(|author| HttpResponse::Ok().json(author))
}

pub async fn get_author_detail(    
    app_state: web::Data<AppState>,
    params: web::Path<i32>,)
    -> Result<HttpResponse, SelfDefinedError> {
        let author_id = params.into_inner();
        get_author_detail_db(&app_state.db, author_id)
        .await
        .map(|author| HttpResponse::Ok().json(author))
}

pub async fn post_new_author(
    new_author: web::Json<CreateAuthor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, SelfDefinedError> {
    post_new_author_db(&app_state.db, CreateAuthor::from(new_author))
        .await
        .map(|author| HttpResponse::Ok().json(author))
}

pub async fn update_author_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_author: web::Json<UpdateAuthor>,
) -> Result<HttpResponse, SelfDefinedError> {
    let author_id = params.into_inner();
    update_author_details_db(
        &app_state.db,
        author_id,
        UpdateAuthor::from(update_author),
    )
        .await
        .map(|author| HttpResponse::Ok().json(author))
}

pub async fn delete_author(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, SelfDefinedError> {
    let author_id = params.into_inner();
    delete_author_db(&app_state.db, author_id)
        .await
        .map(|author| HttpResponse::Ok().json(author))
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;
    use crate::state::AppState;
    #[actix_rt::test]
    async fn get_all_author_success_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let resp = get_all_author(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_author_details_success_teat() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let params: web::Path<i32> = web::Path::from(2);
    let resp = get_author_detail(app_state, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK)
}

#[actix_rt::test]
async fn post_author_success_test() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let new_author = CreateAuthor {
        name: "Third Author".into(),
        picture_url: "https://phy.xyz".into(),
        profile: "This is a test profile".into(),
    };
    let author_params = web::Json(new_author);
    let resp = post_new_author(author_params, app_state).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK)
}

#[actix_rt::test]
async fn delete_author_success_test() {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    let app_state: web::Data<AppState> = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: db_pool,
    });

    let params: web::Path<i32> = web::Path::from(1);
    let resp = delete_author(app_state, params).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK)

}
}