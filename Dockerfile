FROM rust:1.99-bookworm AS build
WORKDIR /app
COPY . .
RUN cargo build --locked --release -p rf-cli
FROM debian:bookworm-slim
COPY --from=build /app/target/release/route-foundry /usr/local/bin/route-foundry
EXPOSE 8080
ENTRYPOINT ["route-foundry"]
CMD ["serve","--addr","0.0.0.0:8080"]
