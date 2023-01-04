#!/bin/bash
export PGPASSWORD=postgres
set -e

psql -v ON_ERROR_STOP=1 --username "$POSTGRES_USER" --dbname "$POSTGRES_DB" <<-EOSQL
  CREATE TABLE "employees"
  (
      id SERIAL PRIMARY KEY,
      first_name VARCHAR NOT NULL,
      last_name VARCHAR NOT NULL,
      department VARCHAR NOT NULL,
      salary FLOAT NOT NULL,
      age INT NOT NULL
  );
EOSQL