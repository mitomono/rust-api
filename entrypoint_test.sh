#!/bin/bash
export PGPASSWORD=postgres
set -e

diesel migration run

psql -v ON_ERROR_STOP=1 -U postgres -h db_test --dbname "tests" <<-EOSQL
insert into members (id, first_name, last_name, email, address, age) values (1,'username', 'user last_name','user@gg.com', 'elm street', 38);
insert into books (id, title, isbn, copies_available, copies) values (1,'title_1', '1234',4, 4);
EOSQL

cargo test