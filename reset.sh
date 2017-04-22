#!/bin/bash

dropdb boilerplateapp || true
diesel setup
