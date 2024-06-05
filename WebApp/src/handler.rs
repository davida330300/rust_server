use crate::error::WebAppError;
use crate::model::{AuthorRegisterForm, AuthorResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde_json::json;

pub async fn get_all_author(tmpl: web::Data<tera::Tera>)
                              -> Result<HttpResponse, Error> {
    let awc_client = awc::Client::default();

    let res = awc_client
        .get("http://localhost:3000/author/")
        .send()
        .await
        .unwrap()
        .json::<Vec<AuthorResponse>>()
        .await
        .unwrap();

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("author", &res);

    let s = tmpl
        .render("author.html", &ctx).unwrap();
        //.map_err(|_| WebAppError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_picture_url", "");
    ctx.insert("current_profile", "");
    let s = tmpl
        .render("register.html", &ctx).unwrap();
        //.map_err(|_| WebAppError::TeraError("template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<AuthorRegisterForm>,
) -> Result<HttpResponse, Error> {
    let mut ctx = tera::Context::new();
    let s;
    if params.name == "Dave" {
        ctx.insert("error", "Dave is already exists!");
        ctx.insert("current_name", &params.name);
        ctx.insert("current_picture_url", &params.picture_url);
        ctx.insert("current_profile", &params.profile);
        s = tmpl
            .render("register.html", &ctx)
            .map_err(|err| WebAppError::TeraError(err.to_string()))?;
    } else {
        let new_teacher = json!({
            "name": &params.name,
            "picture_url": &params.picture_url,
            "profile": &params.profile,
        });
        let awc_client = awc::Client::default();
        let res = awc_client
            .post("http://localhost:3000/author/")
            .send_json(&new_teacher)
            .await
            .unwrap()
            .body()
            .await?;
        let author_response: AuthorResponse =
            serde_json::from_str(&std::str::from_utf8(&res)?)?;
        s = format!("Congratulations! Your Id is: {}", author_response.id);
    }

    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}