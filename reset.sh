#!/bin/bash

# Source .env to set local variables
eval $(echo "$(cat .env) $1" | tr '\n' ' ')

dropdb --if-exists ${DATABASE_NAME}
diesel setup --database-url ${DATABASE_URL}
