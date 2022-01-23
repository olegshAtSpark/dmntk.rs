#!/usr/bin/env bash

# set variables
NAME=dmntk
VERSION=0.0.46-dev

# clean before proceeding
cargo clean
docker stop $NAME
docker rm $NAME
docker rmi "$(docker images | grep $VERSION | awk '{print $3}')"

# build musl target
cargo build --target x86_64-unknown-linux-musl --release

# build the docker image
docker build -t $NAME:$VERSION .

# start the container
docker run --name $NAME -d -p 22022:22022 $NAME:$VERSION

# run test
curl localhost:22022/system/info
echo ""