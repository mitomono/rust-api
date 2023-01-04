use std::collections::{HashMap, HashSet};

use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::db;
use crate::error_handler::CustomError;
use crate::schema::books;
use crate::utils::check;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[diesel(table_name = books)]
pub struct Book {
    pub title: String,
    pub isbn: String,
    pub copies_available: i32,
    pub copies: i32,
}

#[derive(Serialize, Deserialize, Queryable, Insertable, ToSchema)]
#[diesel(table_name = books)]
pub struct Books {
    pub id: i32,
    pub title: String,
    pub isbn: String,
    pub copies_available: i32,
    pub copies: i32,
}

impl Books {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let mut conn = db::connection()?;
        let books = books::table.load::<Books>(&mut conn)?;
        Ok(books)
    }

    pub fn get(params: HashMap<String, String>) -> Result<Vec<Self>, CustomError> {
        let mut query = books::table.into_boxed();

        if params.contains_key("id") {
            match check::validate_int(params.get("id").unwrap()) {
                Ok(id) => query = query.filter(books::id.eq(id)),
                Err(error) => return Err(error),
            }
        }
        if params.contains_key("ids") {
            match check::parse_ids(params.get("ids").unwrap()) {
                Ok(ids) => {
                    let ids_clean: HashSet<i32> = ids.into_iter().collect();
                    query = query.filter(books::id.eq_any(ids_clean));
                }
                Err(error) => return Err(error),
            }
        }
        if let Some(title) = params.get("title") {
            query = query.filter(books::title.eq(title))
        }
        if let Some(isbn) = params.get("isbn") {
            query = query.filter(books::isbn.eq(isbn))
        }

        let mut conn = db::connection()?;
        let books = match query.get_results(&mut conn) {
            Ok(books) => books,
            Err(e) => return Err(CustomError::from(e)),
        };

        Ok(books)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let book = books::table.filter(books::id.eq(id)).first(&mut conn)?;
        Ok(book)
    }

    pub fn create(book: Book) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let book = Book::from(book);
        let book = diesel::insert_into(books::table)
            .values(book)
            .get_result(&mut conn)?;
        Ok(book)
    }

    pub fn update(id: i32, book: Book) -> Result<Self, CustomError> {
        let mut conn = db::connection()?;
        let book = diesel::update(books::table)
            .filter(books::id.eq(id))
            .set(book)
            .get_result(&mut conn)?;
        Ok(book)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let mut conn = db::connection()?;
        let res = diesel::delete(books::table.filter(books::id.eq(id))).execute(&mut conn)?;
        Ok(res)
    }
}

impl Book {
    fn from(book: Book) -> Book {
        Book {
            title: book.title,
            isbn: book.isbn,
            copies_available: book.copies_available,
            copies: book.copies,
        }
    }
}
