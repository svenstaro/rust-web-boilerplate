#!/bin/bash

dropdb --if-exists boilerplateapp
diesel setup --database-url ${BOILERPLATEAPP_DATABASE_URL}
