ARG RUST_VER=1.51

# --- prepare ------------------------------------------------
From rust:${RUST_VER} as prepare
RUN cargo install cargo-chef && rm -rf $CARGO_HOME/registry/

WORKDIR /app
COPY . .
WORKDIR /app
RUN cargo chef prepare --recipe-path recipe.json


# --- cacher -----------------------------------------------
From rust:${RUST_VER} as cacher
RUN cargo install cargo-chef && rm -rf $CARGO_HOME/registry/

WORKDIR /app
COPY --from=prepare /app/recipe.json recipe.json

RUN cargo chef cook --release --recipe-path recipe.json

# --- builder -----------------------------------------------
From rust:${RUST_VER} as builder
ARG BUILD_FLAG=--release

RUN rustup component add rustfmt
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

WORKDIR /app/graphql
RUN cargo build --bin graphql-server ${BUILD_FLAG}


# --- bin -----------------------------------------------
From rust:${RUST_VER} as runtime
ARG BUILD_TARGET=release
ARG ENV=dev

WORKDIR app
COPY --from=builder /app/target/${BUILD_TARGET}/graphql-server graphql-server

EXPOSE 5000
CMD ./graphql-server -e .env
