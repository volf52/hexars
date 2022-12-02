FROM volf52/rust-musl-builder:1.65.0-slim as userconfig

ARG USER=hexa
ARG USERID=10001
ENV USER=${USER}
ENV UID=${USERID}

# prevent using sudo
USER root:root

#See https://stackoverflow.com/a/55757473/12429735
RUN adduser \
  --disabled-password \
  --gecos "" \
  --home "/nonexistent" \
  --shell "/sbin/nologin" \
  --no-create-home \
  --uid "${UID}" \
  "${USER}"

# reset back to user space
USER volfy:volfy

FROM volf52/rust-musl-builder:1.65.0-slim as base

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

# RUN cargo build --target x86_64-unknown-linux-musl && /app/target/x86_64-unknown-linux-musl/debug/migrate
RUN cargo build --release --target x86_64-unknown-linux-musl


FROM base as hexars

# COPY --from=migrate /app/db.sqlite .
COPY --from=hexars-recipe /app/recipe.json .

RUN cargo chef cook --target x86_64-unknown-linux-musl --recipe-path recipe.json

COPY .env.build .env
COPY hexars .

RUN cargo b -r --target x86_64-unknown-linux-musl

# ---- Minimize size ----
FROM volf52/upx-minimal:3.96 as upx-source

# Need this to utilize RUN, as it isn't present in scratch images like upx-minimal
FROM base as compress

COPY --from=upx-source /bin/upx .
COPY --from=hexars /app/target/x86_64-unknown-linux-musl/release/server .

RUN ./upx --best --lzma -o server_minimal server

# ----------- Production --------------
FROM volf52/tini:0.19 as tini

# FROM gcr.io/distroless/static:nonroot AS runtime
FROM scratch as runtime

ARG USER=hexa
USER ${USER}:${USER}

COPY --from=userconfig /etc/passwd /etc/passwd
COPY --from=userconfig /etc/group /etc/group
# COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/


# WORKDIR /app
EXPOSE 3000

# COPY --from=tini --chown=nonroot:nonroot /bin/tini /bin/tini
COPY --from=tini /bin/tini /bin/tini

# COPY --from=migrate --chown=nonroot:nonroot /app/db.sqlite .
# COPY --from=migrate /app/db.sqlite .
COPY --from=migrate /app/target/x86_64-unknown-linux-musl/release/migrate /app/migrate

# COPY --from=compress --chown=nonroot:nonroot /app/server_minimal ./server
COPY --from=compress /app/server_minimal /app/server

# COPY --chown=nonroot:nonroot .env .
# COPY  .env .

ENTRYPOINT ["/bin/tini", "--","/app/server"]
