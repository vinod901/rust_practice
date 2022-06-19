#!/bin/zsh
LAMBDA_ARCH="linux/arm_64"
RUST_TARGET="aarch64-unknown-linux-gnu"
RUST_VERSION="latest"
PROJECT_NAME="rust_practice"

al2build(){
    docker run --platform ${LAMBDA_ARCH} \
        --rm --user "${id -u}":"${id -g}" \
        -v "$(pwd)":/usr/src/myapp -w /usr/src/myapp rust:${RUST_VERSION} \
        cargo build --target ${RUST_TARGET} --release
}

zipRustLambda(){
    cp ./target/${RUST_TARGET}/release/${PROJECT_NAME} ./bootstrap \
        && zip lamdbda.zip bootstrap \
        && rm bootstrap
}