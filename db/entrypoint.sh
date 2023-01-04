#!/bin/bash
export PGPASSWORD=postgres
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  CREATE TABLE "members"
  (
      id SERIAL PRIMARY KEY,
      first_name VARCHAR NOT NULL,
      last_name VARCHAR NOT NULL,
      email VARCHAR NOT NULL,
      address VARCHAR NOT NULL,
      age INT NOT NULL
  );
  CREATE TABLE "books"
    (
        id SERIAL PRIMARY KEY,
        title VARCHAR NOT NULL,
        isbn VARCHAR NOT NULL,
        copies_available INT NOT NULL,
        copies INT NOT NULL
    );
EOSQL