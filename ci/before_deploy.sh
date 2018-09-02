#!/usr/bin/env bash
# Building and packaging for release

set -ex

build() {
    cargo build --target "$TARGET" --release --verbose
}

pack() {
    local tempdir
    local out_dir
    local package_name

    tempdir=$(mktemp -d 2>/dev/null || mktemp -d -t tmp)
    out_dir=$(pwd)
    package_name="$PROJECT_NAME-$TRAVIS_TAG-$TARGET"

    # create a "staging" directory
    mkdir "$tempdir/$package_name"

    # copying the main binary
    ls -al "target/$TARGET/release"
    cp "target/$TARGET/release/$PROJECT_NAME" "$tempdir/$package_name/"
    strip "$tempdir/$package_name/$PROJECT_NAME"

    # readme and license
    cp README.md "$tempdir/$package_name"
    cp LICENSE-MIT "$tempdir/$package_name"
    cp LICENSE-APACHE "$tempdir/$package_name"

    # archiving
    pushd "$tempdir"
    tar czf "$out_dir/$package_name.tar.gz" "$package_name"/*
    popd
    rm -r "$tempdir"
}


main() {
    pack
}

main
