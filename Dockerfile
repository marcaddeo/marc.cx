FROM rust:1-bookworm as rust-builder

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM node:lts as node-builder

WORKDIR /app
COPY . .
RUN yarn install
RUN yarn build
RUN yarn global add imageoptim-cli
RUN cd content && imageoptim

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=rust-builder /usr/local/cargo/bin/marccx /app
COPY --from=node-builder /app/static /app/static
COPY --from=node-builder content /app/content

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["./marccx"]
