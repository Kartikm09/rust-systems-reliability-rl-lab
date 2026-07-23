FROM rust:1.97.1-slim AS build
WORKDIR /workspace
COPY . .
RUN cargo build --locked --release -p cli

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl ca-certificates && rm -rf /var/lib/apt/lists/* && useradd --create-home --uid 10001 appuser
COPY --from=build /workspace/target/release/event-lab /usr/local/bin/event-lab
USER appuser
ENV APP_PORT=8082
EXPOSE 8082
CMD ["event-lab", "serve"]
