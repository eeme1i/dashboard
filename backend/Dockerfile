FROM rust:trixie

WORKDIR /usr/src/app
COPY . .
RUN cargo install --path .

# expose port 2001
EXPOSE 2001
CMD ["home-backend"]

# build: docker build -t home-backend .
# run with: docker run -p 2001:2001 home-backend