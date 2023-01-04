use std::collections::HashMap;

use actix_web::{delete, get, post, put, web, HttpResponse};
use serde_json::json;

use crate::books::{Book, Books};
use crate::error_handler::CustomError;
use crate::utils::check;
use crate::utils::response;

#[utoipa::path(
    get,
    path = "/books",
    responses(
        (status = 200, description = "Get all books", body = inline(response::BooksResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/books")]
async fn find_all() -> Result<HttpResponse, CustomError> {
    let books = web::block(Books::find_all).await.unwrap();
    Ok(HttpResponse::Ok().json(books))
}

#[utoipa::path(
    get,
    path = "/books/filter",
    responses(
        (status = 200, description = "Get books filtered with url params", body = inline(response::BooksResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    ),
    params(
        ("id" = Option<i32>, Query, description = "Book database id"),
        ("ids" = Option<String>, Query, description = "Books database comma separated ids"),
        ("title" = Option<String>, Query,  description = "Book Title"),
        ("isbn" = Option<String>, Query,  description = "Book isbn"),
        ("copies_available" = Option<i32>, Query,  description = "Num of copies available"),
        ("copies" = Option<i32>, Query, description = "Num of total copies"),
    )
)]
#[get("/books/filter")]
async fn filter(_param: web::Query<HashMap<String, String>>) -> Result<HttpResponse, CustomError> {
    let params = _param.into_inner();

    let books = if params.is_empty() {
        web::block(Books::find_all).await.unwrap()
    } else {
        match check::validate_book_params(&params) {
            Ok(..) => web::block(move || Books::get(params)).await.unwrap(),
            Err(err) => return Err(err),
        }
    };

    Ok(HttpResponse::Ok().json(books))
}

#[utoipa::path(
    get,
    path = "/books/{id}",
    responses(
        (status = 200, description = "Get a book identifies with id", body = inline(Books)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[get("/books/{id}")]
async fn find(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let book = Books::find(id.into_inner())?;
    Ok(HttpResponse::Ok().json(book))
}

#[utoipa::path(
    post,
    path = "/books",
    responses(
        (status = 200, description = "Create a new book", body = inline(response::BookResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[post("/books")]
async fn create(book: web::Json<Book>) -> Result<HttpResponse, CustomError> {
    let book = Books::create(book.into_inner())?;
    Ok(HttpResponse::Ok().json(book))
}

#[utoipa::path(
    put,
    path = "/books{id}",
    responses(
    (status = 200, description = "Modify a new book", body = inline(response::BookResponse)),
    (status = 400, description = "Error", body = inline(response::ErrorResponse)),
    (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[put("/books/{id}")]
async fn update(id: web::Path<i32>, book: web::Json<Book>) -> Result<HttpResponse, CustomError> {
    let book = Books::update(id.into_inner(), book.into_inner())?;
    Ok(HttpResponse::Ok().json(book))
}

#[utoipa::path(
    delete,
    path = "/books{id}",
    responses(
        (status = 200, description = "Delete a new book", body = inline(response::DeleteResponse)),
        (status = 400, description = "Error", body = inline(response::ErrorResponse)),
        (status = 404, description = "Error", body = inline(response::ErrorResponse))
    )
)]
#[delete("/books/{id}")]
async fn delete(id: web::Path<i32>) -> Result<HttpResponse, CustomError> {
    let deleted_book = Books::delete(id.into_inner())?;
    Ok(HttpResponse::Ok().json(json!({ "deleted": deleted_book })))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(find_all);
    config.service(filter);
    config.service(find);
    config.service(create);
    config.service(update);
    config.service(delete);
    config.service(delete);
}
