#!/bin/sh

set -e

# https://stackoverflow.com/questions/59895/how-do-i-get-the-directory-where-a-bash-script-is-located-from-within-the-script/246128#246128
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Backend

cd $SCRIPT_DIR/backend

if [ ! -f ".env" ]; then
  echo "No env file found in 'backend/.env'. Making one..."
  touch .env
fi

# https://stackoverflow.com/questions/43267413/how-to-set-environment-variables-from-env-file/43267603#43267603
while IFS='=' read -r key value; do
  printf -v "$key" %s "$value" && export "$key"
done < ".env"

if [ ! $SSL_PASSWORD ]; then
  SSL_PASSWORD=$(< /dev/urandom tr -dc 'a-zA-Z0-9_!@#$%^&*' | head -c 1000)
  echo "\nSSL_PASSWORD=${SSL_PASSWORD}" > .env
fi

if [ ! -f "cert/key.pem" ] || [ ! -f "cert/cert.pem" ]; then
  rm -rf cert/key.pem
  rm -rf cert/cert.pem

  # NOTE: You need to change the /CN to your hosted netlify domain.
  # If you want to deploy your own netlify app with it.
  openssl req -x509 -newkey rsa:4096 -keyout cert/key.pem -out \
    cert/cert.pem -days 365 -sha256 -subj \
    "/C=BG/CN=1k2s-reactix.netlify.app" -passout \
    pass:$SSL_PASSWORD
fi

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
# yarn serve
