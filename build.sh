#!/bin/sh

set -e

# https://stackoverflow.com/questions/59895/how-do-i-get-the-directory-where-a-bash-script-is-located-from-within-the-script/246128#246128
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Backend

cd $SCRIPT_DIR/backend

rustup toolchain install nightly &&
	rustup default nightly

cargo run --release &

# Frontend

cd $SCRIPT_DIR/frontend

npm install yarn --global # It's just faster to build with yarn.

yarn install
yarn global add vite
yarn global add nodemon
yarn build
yarn serve
