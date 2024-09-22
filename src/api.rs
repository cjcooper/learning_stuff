use actix_web::{web, get, post, delete, put, HttpResponse};
use crate::{models::{NewBook, Book}, database::Database};

#[post("/books")]
pub async fn create_book(db: web::Data<Database>, new_book: web::Json<NewBook>) -> HttpResponse {
    let todo = db.create_book(new_book.into_inner());
    match todo {
        Ok(todo) => HttpResponse::Ok().json(todo),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

#[get("/books/{id}")]
pub async fn get_book_by_id(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let todo = db.get_book_by_id(id.into_inner());

    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[get("/books")]
pub async fn get_books(db: web::Data<Database>) -> HttpResponse {
    let todos = db.get_books();
    HttpResponse::Ok().json(todos)
}

#[delete("/books/{id}")]
pub async fn delete_book_by_id(db: web::Data<Database>, id: web::Path<i32>) -> HttpResponse {
    let todo = db.delete_book_by_id(id.into_inner());
    match todo {
        Some(_) => HttpResponse::Ok().finish(),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

#[put("/books/{id}")]
pub async fn update_book_by_id(db: web::Data<Database>, id: web::Path<i32>, updated_book: web::Json<Book>) -> HttpResponse {
    let todo = db.update_book_by_id(id.into_inner(), updated_book.into_inner());
    match todo {
        Some(todo) => HttpResponse::Ok().json(todo),
        None => HttpResponse::NotFound().body("Todo not found"),
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(create_book)
            .service(get_book_by_id)
            .service(get_books)
            .service(delete_book_by_id)
            .service(update_book_by_id)
    );
}