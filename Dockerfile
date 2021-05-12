FROM rustlang/rust:nightly

RUN apt update \
    && apt install -y postgresql \
    && cargo install sqlx-cli --no-default-features --features postgres

WORKDIR /api
COPY . /api

EXPOSE 8080

ENTRYPOINT ["/api/docker_entrypoint.sh"]