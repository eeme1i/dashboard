FROM rust:trixie AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:trixie-slim
WORKDIR /app
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates libssl3 && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/home-backend /app/home-backend
# expose port 2001
EXPOSE 2001
CMD ["/app/home-backend"]

# build: docker build -t home-backend .
# run with: docker run -p 2001:2001 home-backend