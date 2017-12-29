#!/bin/bash

dropdb --if-exists ${DATABASE_NAME}
diesel setup --database-url ${DATABASE_URL}
