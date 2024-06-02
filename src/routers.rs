use crate::handlers::{book::*, general::health_check_handler};
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
 