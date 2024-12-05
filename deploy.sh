#!/bin/env sh

ADDRESS=192.168.1.29
RESET_DB=""

if [ "$1" -ge 1 ]; then
	echo "Server reset: activated"
fi
if [ "$2" -ge 1 ]; then
	echo "DB reset: activated"
	RESET_DB="--reset-db"
fi

rm -rf build

mkdir -p build

ROCKET_PROFILE=release cargo build --release --target=aarch64-unknown-linux-gnu
cp target/aarch64-unknown-linux-gnu/release/matrix-completion-app build/matrix-server

cp .env build/.env
cp -r static build/static
cp -r assets build/assets

cd build || exit
TO_COPY="static assets"
if [ "$1" -ge 1 ]; then
	echo "pkill matrix-server" | ssh $ADDRESS
	TO_COPY="$TO_COPY matrix-server .env"
fi
# shellcheck disable=SC2086
scp -r $TO_COPY $ADDRESS:MatrixCompletion

if [ "$1" -ge 1 ]; then
	echo "cd MatrixCompletion; ROCKET_PROFILE=release ./matrix-server $RESET_DB" | ssh $ADDRESS
fi
