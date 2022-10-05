FROM rust:1.64

WORKDIR /usr/src/redirector
COPY . .

RUN cargo install --path .

EXPOSE 5000

CMD ["redirector"]
