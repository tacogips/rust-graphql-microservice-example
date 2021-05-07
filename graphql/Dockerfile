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

WORKDIR /app/graphql
RUN cargo build --bin graphql-server ${BUILD_FLAG}


# --- bin -----------------------------------------------
From rust:${RUST_VER} as runtime
ARG BUILD_TARGET=release
ARG ENV=dev

WORKDIR app
ARG ARTICLE_SERVICE_ENDPOINT
ARG USER_SERVICE_ENDPOINT
ARG COMMENT_SERVICE_ENDPOINT

COPY --from=builder /app/target/${BUILD_TARGET}/graphql-server graphql-server


ENV ARTICLE_SERVICE_ENDPOINT = ${ARTICLE_SERVICE_ENDPOINT}
ENV USER_SERVICE_ENDPOINT    = ${USER_SERVICE_ENDPOINT}
ENV COMMENT_SERVICE_ENDPOINT = ${COMMENT_SERVICE_ENDPOINT}

EXPOSE 5000
CMD ./graphql-server -e .env
