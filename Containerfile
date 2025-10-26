FROM node:20 AS tailwind
WORKDIR /opt
COPY . /opt
RUN npm i -g @tailwindcss/cli && npm i tailwindcss && npx tailwindcss -i ./input.css -o ./assets/tailwind.css


FROM rust:1.90 AS builder
WORKDIR /opt
COPY --from=tailwind /opt /opt

RUN cargo install cargo-binstall
RUN cargo binstall dioxus-cli --version 0.6.3 -y

RUN dx bundle --platform web

FROM debian:bookworm 
WORKDIR /app
COPY --from=builder /opt/target/dx/auth/release/web /app

ENV PORT=8080
ENV IP=0.0.0.0
EXPOSE 8080

#WORKDIR /opt/target/dx/dummy-build-repro/release/web
ENTRYPOINT ["./server"]


