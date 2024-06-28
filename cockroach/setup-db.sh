#!/bin/bash

./cockroach sql --host=$DB_HOST --certs-dir=$CERTS --execute="create database if not exists ${DB_NAME}; \
    create user if not exists ${DB_USER} with password '${DB_PASSWORD}'; \
    grant all on database ${DB_NAME} to ${DB_USER} with grant option; \
    grant admin to ${DB_USER};"
