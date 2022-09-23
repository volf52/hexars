# FROM lukemathwalker/cargo-chef:0.1.41-rust-1.63-slim-buster as base
FROM clux/muslrust:stable as base

WORKDIR /app

RUN cargo install cargo-chef

# ------ Recipes -----

FROM base as migrate-recipe

COPY migrations .

RUN cargo chef prepare --recipe-path recipe.json

FROM base as hexars-recipe

COPY hexars .

RUN cargo chef prepare --recipe-path recipe.json

# ------- Builds ----------

# Not separating the steps to dep and build, as it will add an unnecessary lookup of crates.io index
FROM base as migrate

COPY --from=migrate-recipe /app/recipe.json .

RUN cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY migrations .

RUN cargo build --target x86_64-unknown-linux-musl && /app/target/x86_64-unknown-linux-musl/debug/migrate


FROM base as hexars

COPY --from=migrate /app/db.sqlite .
COPY --from=hexars-recipe /app/recipe.json .

RUN cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY .env .
COPY hexars .

RUN cargo b -r --target x86_64-unknown-linux-musl

# ----------- Production --------------

# FROM gcr.io/distroless/static:nonroot AS runtime
FROM scratch as runtime

WORKDIR /app

# COPY --from=migrate --chown=nonroot:nonroot /app/db.sqlite .
COPY --from=migrate /app/db.sqlite .

# COPY --from=hexars --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/server .
COPY --from=hexars /app/target/x86_64-unknown-linux-musl/release/server .

# COPY --chown=nonroot:nonroot .env .
COPY  .env .


ENTRYPOINT ["/app/server"]
