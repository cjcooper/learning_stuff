use std::fmt::Error;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use dotenvy::dotenv;
use crate::models::{NewBook, Book};
use crate::schema::{books::dsl::*};

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub struct Database {
    pool: DBPool,
}

impl Database {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool: DBPool = r2d2::Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Database { pool }
    }

    pub fn create_book(&self, new_book: NewBook) -> Result<Book, Error> {
        let book = diesel::insert_into(books)
            .values(&new_book)
            .returning(Book::as_returning())
            .get_result(&mut self.pool.get().unwrap())
            .expect("Error saving new post");
        Ok(book)
    }

    pub fn get_books(&self) -> Vec<Book> {
        books
            .load::<Book>(&mut self.pool.get().unwrap())
            .expect("Error loading all books")
    }

    pub fn get_book_by_id(&self, book_id: i32) -> Option<Book> {
        let book = books
            .find(book_id)
            .get_result::<Book>(&mut self.pool.get().unwrap())
            .expect("Error loading book by id");
        Some(book)
    }

    pub fn delete_book_by_id(&self, book_id: i32) -> Option<usize> {
        let count = diesel::delete(books.find(book_id))
            .execute(&mut self.pool.get().unwrap())
            .expect("Error deleting book by id");
        Some(count)
    }

    pub fn update_book_by_id(&self, book_id: i32, book: Book) -> Option<Book> {
        let book = diesel::update(books.find(book_id))
            .set(&book)
            .get_result::<Book>(&mut self.pool.get().unwrap())
            .expect("Error updating book by id");
        Some(book)
    }
}