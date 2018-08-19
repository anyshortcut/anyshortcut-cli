#!/bin/bash
set -ex

BUILD_DIR="/work"

docker run \
        -w ${BUILD_DIR} \
        -v `pwd`:${BUILD_DIR}:ro \
        -v `pwd`/target:${BUILD_DIR}/target \
        -v $HOME/.cargo/registry:/root/.cargo/registry \
        -it messense/rust-musl-cross:${DOCKER_IMAGE_TAG} \
        cargo test --target ${TARGET} --verbose

# Fix permissions for shared directories
USER_ID=$(id -u)
GROUP_ID=$(id -g)
sudo chown -R ${USER_ID}:${GROUP_ID} target/ $HOME/.cargo
