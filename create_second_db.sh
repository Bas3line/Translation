#!/bin/bash

set -e

DB_NAME="translation_bot_2"
CONTAINER_NAME="megachinese_db"

echo "Creating database: $DB_NAME"

docker exec -it $CONTAINER_NAME psql -U postgres -c "CREATE DATABASE $DB_NAME;"

echo "Database $DB_NAME created successfully"
echo "Use this connection string for your second bot:"
echo "DATABASE_URL=postgresql://postgres:postgres@localhost:5433/$DB_NAME"
