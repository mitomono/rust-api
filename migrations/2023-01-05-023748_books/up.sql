CREATE TABLE IF NOT EXISTS books
(
    id SERIAL PRIMARY KEY,
    title VARCHAR NOT NULL,
    isbn VARCHAR NOT NULL,
    copies_available INT NOT NULL,
    copies INT NOT NULL
);