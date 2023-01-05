#!/bin/bash
export PGPASSWORD=postgres
set -e

diesel migration run

cargo run