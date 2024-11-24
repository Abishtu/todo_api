FROM rust:latest

WORKDIR /app/todo_api

COPY ../ .

RUN cargo install --path /app/todo_api

EXPOSE 3000

CMD ["todo_api"]
