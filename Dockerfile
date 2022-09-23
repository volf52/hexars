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

# ---- Minimize size ----
FROM volf52/upx-minimal:1.0 as upx-source

# Need this to utilize RUN, as it isn't present in scratch images like upx-minimal
FROM base as compress

COPY --from=upx-source /bin/upx .
COPY --from=hexars /app/target/x86_64-unknown-linux-musl/release/server .

RUN ./upx --best --lzma -o server_minimal server

# ----------- Production --------------
FROM volf52/tini:1.1 as tini

# FROM gcr.io/distroless/static:nonroot AS runtime
FROM scratch as runtime

WORKDIR /app
EXPOSE 3000

# COPY --from=tini --chown=nonroot:nonroot /bin/tini /bin/tini
COPY --from=tini /bin/tini /bin/tini

# COPY --from=migrate --chown=nonroot:nonroot /app/db.sqlite .
COPY --from=migrate /app/db.sqlite .

# COPY --from=compress --chown=nonroot:nonroot /app/server_minimal ./server
COPY --from=compress /app/server_minimal ./server

# COPY --chown=nonroot:nonroot .env .
COPY  .env .

ENTRYPOINT ["/bin/tini", "--","/app/server"]
