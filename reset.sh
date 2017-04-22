#!/bin/bash

dropdb --if-exists boilerplateapp
diesel setup
