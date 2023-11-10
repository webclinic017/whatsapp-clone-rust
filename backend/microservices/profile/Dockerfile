# syntax=docker/dockerfile:1

ARG APP_NAME=profile-microservice
ARG APP_NAME

ARG PORT=4000
ARG PORT

### STAGE 1 - Building the application ###

FROM rust:1.70.0-slim-bullseye AS build
WORKDIR /app

RUN apt update -y && \
    apt install -y protobuf-compiler

## Build the application.
## Leverage a cache mount to /usr/local/cargo/registry/ for downloaded dependencies and a cache mount
## to /app/target/ for compiled dependencies which will speed up subsequent builds. Once built, copy
## the executable to an output directory before the cache mounted at /app/target is unmounted.
RUN --mount=type=bind,source=backend/microservices/profile,target=backend/microservices/profile \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=bind,source=protos,target=protos \
    --mount=type=cache,target=target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
set -e
cargo build --bin profile-microservice --locked --release
cp ./target/release/profile-microservice /bin/server
EOF

### STAGE 2 - Running the application ###

FROM scratch AS final

## Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

## Expose the port that the application listens on.
EXPOSE ${PORT}

## What the container should run when it is started.
CMD ["/bin/server"]