use crate::handlers::{book::*, general::health_check_handler, author::*};
use actix_web::web;

pub fn general_routes(cfg: &mut web::ServiceConfig) {
    // cfg.route("/health", web::get().to(health_check_handler));
    cfg.route("/health", web::get().to(health_check_handler));
} 

pub fn book_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/book")
            .route("/", web::post().to(post_new_book))
            .route("/{author_id}", web::get().to(get_book_for_author))
            .route("/{author_id}/{book_id}", web::get().to(get_book_detail)),
    );
}
 
pub fn author_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/author")
            .route("/", web::post().to(post_new_author))
            .route("/", web::get().to(get_all_author))
            .route("/{author_id}", web::get().to(get_author_detail))
            .route("/{author_id}", web::put().to(update_author_details))
            .route("/{author_id}", web::delete().to(delete_author)),
    );
}