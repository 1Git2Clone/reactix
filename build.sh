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

# It's just faster to build with yarn.
if [ ! command -v yarn >/dev/null 2>&1 ]; then npm install yarn --global; fi

if [ ! command -v vite >/dev/null 2>&1 ]; then yarn global add vite; fi
if [ ! command -v nodemon >/dev/null 2>&1 ]; then yarn global add nodemon; fi

yarn install

yarn build
yarn serve
