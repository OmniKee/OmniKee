FROM docker.io/library/rust:slim as builder-rust

RUN cargo install wasm-pack

WORKDIR /code/lib
COPY ./lib .

ENV RUSTFLAGS='--cfg getrandom_backend="wasm_js"'
RUN wasm-pack build -t web

FROM docker.io/library/node:lts-slim as builder-node

WORKDIR /code/www
COPY ./www .
COPY --from=builder-rust /code/lib/pkg /code/lib/pkg

ENV PUBLIC_PATH=/
RUN npm install && npm run build

FROM docker.io/library/busybox:stable

USER www-data

COPY --from=builder-node --chown=www-data:www-data /code/www/dist/spa /var/www/html/

ENTRYPOINT ["/bin/httpd"]
CMD ["-f", "-p", "8080", "-h", "/var/www/html"]
