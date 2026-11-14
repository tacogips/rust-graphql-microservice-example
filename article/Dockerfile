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
ARG BUILD_FLAG=--release
RUN cargo install cargo-chef && rm -rf $CARGO_HOME/registry/

WORKDIR /app
COPY --from=prepare /app/recipe.json recipe.json

RUN cargo chef cook ${BUILD_FLAG} --recipe-path recipe.json

# --- builder -----------------------------------------------
From rust:${RUST_VER} as builder
ARG BUILD_FLAG=--release

RUN rustup component add rustfmt
WORKDIR /app
COPY . .
COPY --from=cacher /app/target target
COPY --from=cacher $CARGO_HOME $CARGO_HOME

ARG DATABASE_URL

ENV SQLX_OFFLINE=true
WORKDIR /app/article
RUN cargo build --bin article-server ${BUILD_FLAG}



# --- bin -----------------------------------------------
From rust:${RUST_VER} as runtime
ARG BUILD_TARGET=release
ARG ENV=dev

WORKDIR app
ARG DATABASE_URL
COPY --from=builder /app/target/${BUILD_TARGET}/article-server article-server

ENV DATABASE_URL=${DATABASE_URL}

EXPOSE 5000
CMD ./article-server
