FROM rust:1.23.0

WORKDIR /usr/src/project-ironclad

COPY . .

RUN cargo install

CMD ["ironclad"]
