## NOTE: The SurrealQL file must be mounted at /migrations.surql.

FROM ubuntu:latest

RUN apt-get update -y && \
    apt-get install curl -y

## Install Surrealdb CLI tool.
RUN curl -sSf https://install.surrealdb.com | sh

## Run Surrealdb migrations.
CMD cat /migrations.surql | \
    /usr/local/bin/surreal sql --namespace root --multi -e ${ENDPOINT}