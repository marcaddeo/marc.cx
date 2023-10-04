FROM rust:1-bookworm as rust-builder

WORKDIR /app
COPY . .
RUN cargo install --path .

FROM node:lts as node-builder

WORKDIR /app
COPY . .
RUN yarn install
RUN yarn build

FROM minidocks/imagemagick:latest as content-builder
WORKDIR /app
COPY content /app/content
RUN find content -type f -name "*jpeg" -o -name "*.jpg" -o -name "*.png" \
    | xargs -I{} sh -c 'convert -resize 730x {} - | sponge {}'

FROM debian:bookworm-slim

WORKDIR /app

COPY --from=rust-builder /usr/local/cargo/bin/marccx /app
COPY --from=node-builder /app/static /app/static
COPY --from=content-builder /app/content /app/content

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000

CMD ["./marccx"]
