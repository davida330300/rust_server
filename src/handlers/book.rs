use crate::state::AppState;
use actix_web::web::Json;
use actix_web::{web, HttpResponse};
use crate::db_access::book::{self, *};
use crate::error::SelfDefinedError;
use crate::models::book::{CreateBook, UpdateBook};
use chrono::Utc;

// Test
// curl -X POST localhost:3000/book/ -H "Content-Type: application/json" -d '{"author_id":1, "name":"First Book"}'
// curl -X POST localhost:3000/book/ -H "Content-Type: application/json" -d '{"author_id":2, "name":"Second Book"}'
pub async fn post_new_book(new_book: web::Json<CreateBook>, app_state: web::Data<AppState>) -> Result<HttpResponse, SelfDefinedError>{
    println!("new book");
    // let book_count = app_state
    //     .book
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|book| book.author_id == new_book.author_id)
    //     .collect::<Vec<Book>>()
    //     .len();

    // let new_book = Book {
    //     author_id: new_book.author_id,
    //     id: Some(book_count+1),
    //     name: new_book.name.clone(),
    //     time: Some(Utc::now().naive_utc()),
    // };

    // app_state.book.lock().unwrap().push(new_book);
    post_new_book_db(&app_state.db, new_book.try_into()?)
            .await
            .map(|book| HttpResponse::Ok().json(book))
    //HttpResponse::Ok().json(book)
}

pub async fn get_book_for_author(app_state: web::Data<AppState>, params: web::Path<i32>,) -> Result<HttpResponse, SelfDefinedError> {
    // let author_id: usize = params.into_inner();
    // let filtered_book = app_state
    //     .book
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .filter(|book| book.author_id == author_id)
    //     .collect::<Vec<Book>>();

    // if filtered_book.len() > 0 {
    //     HttpResponse::Ok().json(filtered_book)
    // } else {
    //     HttpResponse::Ok().json("No book found for author".to_string())
    // }
    let author_id = i32::try_from(params.into_inner()).unwrap();
    get_book_for_author_db(&app_state.db, author_id)
        .await
        .map(|book| HttpResponse::Ok().json(book))
    // HttpResponse::Ok().json("Success")
}

pub async fn get_book_detail(app_state: web::Data<AppState>,params: web::Path<(i32, i32)>) -> Result<HttpResponse, SelfDefinedError> {
    // let (author_id, book_id) = params.into_inner();
    // let selected_book = app_state
    //     .book
    //     .lock()
    //     .unwrap()
    //     .clone()
    //     .into_iter()
    //     .find(|x| x.author_id == author_id && x.id == Some(book_id))
    //     .ok_or("book not found");
    // if let Ok(book) = selected_book {
    //     HttpResponse::Ok().json(book)
    // } else {
    //     HttpResponse::Ok().json("book not found".to_string())
    // }
    let (author_id, book_id) = params.into_inner();
    //let  = i32::try_from(params.1).unwrap();
    get_book_details_db(&app_state.db, author_id, book_id)
        .await
        .map(|book| HttpResponse::Ok().json(book))
    //HttpResponse::Ok().json(book)
}

pub async fn delete_book(app_state: web::Data<AppState>, params: web::Path<(i32, i32)>,) -> Result<HttpResponse, SelfDefinedError> {
    let (author_id, id) = params.into_inner();
    delete_book_db(&app_state.db, author_id, id)
        .await
        .map(|book| HttpResponse::Ok().json(book))
}

pub async fn update_book_details(app_state: web::Data<AppState>, update_book: web::Json<UpdateBook>, params: web::Path<(i32, i32)>,) -> Result<HttpResponse, SelfDefinedError> {
    let (author_id, id) = params.into_inner();
    update_book_details_db(&app_state.db, author_id, id, update_book.into())
        .await
        .map(|book| HttpResponse::Ok().json(book)) 
}

#[cfg(test)]
mod test {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;
    use chrono::NaiveDateTime;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;

    #[ignore]
    #[actix_rt::test]
    async fn post_book_test() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("not database url in .env");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let book = web::Json(CreateBook {
            author_id: 1,
            name: "Test book".into(),
            description: None,
            price: None,
            language: None,
        });

        let resp = post_new_book(book, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_books_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("not database url in .env");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let author_id: web::Path<i32> = web::Path::from(1);
        let resp = get_book_for_author(app_state, author_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_book_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("not database url in .env");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let params: web::Path<(i32, i32)> = web::Path::from((1, 1));
        let resp = get_book_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_book_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("not database url in .env");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 100));
        let resp = get_book_detail(app_state, params).await;
        use actix_web::ResponseError;
        match resp {
            Ok(_) => println!("Something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }        
    }

    #[actix_rt::test]
    async fn delete_book_success() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("not database url in .env");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<(i32, i32)> = web::Path::from((1, 3));
        let resp = delete_book(app_state, params)
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
    
    #[actix_rt::test]
    async fn delete_book_failure() {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("not database url in .env");
        let db_pool = PgPoolOptions::new()
            .connect(&db_url)
            .await
            .unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });      

        let params: web::Path<(i32, i32)> = web::Path::from((1, 301));
        let resp = delete_book(app_state, params).await;
        use actix_web::ResponseError;
        match resp {
            Ok(_) => println!("Something went wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND),
        }
    }
    
}