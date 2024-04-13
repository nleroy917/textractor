FROM rust:1.76.0

WORKDIR /app
COPY . /app

RUN cargo install --path /app

CMD ["textractor"]