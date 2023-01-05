# build stage
FROM rustlang/rust:nightly

USER root

WORKDIR /opt/project

RUN apt update -y && apt install postgresql postgresql-contrib -y

COPY . .

RUN cargo install diesel_cli

EXPOSE 8000

#CMD ["./target/release/api_rust"]
CMD ["cargo", "run"]