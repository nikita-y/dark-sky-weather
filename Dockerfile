FROM rust:1.46

WORKDIR /usr/src/app/
COPY . .
RUN cargo build --release

ENTRYPOINT [ "/usr/src/app/entrypoint.sh" ]
