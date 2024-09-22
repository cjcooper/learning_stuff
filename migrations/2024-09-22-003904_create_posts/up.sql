CREATE TABLE books (
   id SERIAL PRIMARY KEY,
   title VARCHAR(255) NOT NULL,
   primary_author VARCHAR(255) NULL
);