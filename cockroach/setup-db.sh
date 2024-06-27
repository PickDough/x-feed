#!/bin/bash

./cockroach sql --insecure --host=$DB_HOST --execute="create database if not exists ${DB_NAME};"
#  \
#     create user if not exists ${DB_USER} with password '${DB_PASSWORD}'; \
#     grant all on database ${DB_NAME} to ${DB_USER} with grant option; \
#     grant admin to ${DB_USER};"
