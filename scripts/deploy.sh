#!/bin/bash -xe
HOST="$1"
shift
PROJECT=mysterion

CROSS_TARGET=x86_64-unknown-linux-gnu
cargo zigbuild --target=$CROSS_TARGET --package=$PROJECT --all-targets --release
ssh $HOST "mkdir -p $PROJECT/bin $PROJECT/log $PROJECT/etc"
(cd target/x86_64-unknown-linux-gnu/release/ && rsync -avizh `find . -maxdepth 1 -type f ! -name "*.*"` $HOST:$PROJECT/bin/ )
scp $PROJECT/etc/config.json $HOST:$PROJECT/etc/config.json
