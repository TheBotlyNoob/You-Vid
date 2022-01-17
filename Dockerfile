FROM rust:slim

ADD . /src
WORKDIR /src

RUN apt-get update -y && apt-get install -y perl-base libfindbin-libs-perl make

RUN cargo install wasm-pack && wasm-pack build --target web frontend

RUN cargo build --release

ENTRYPOINT [ "cargo", "run", "--release" ]
