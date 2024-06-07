#!/bin/sh

# Process args

FORCE=false

for arg in "$@"
do
  if [ "$arg" == "--help" ] || [ "$arg" == "-h" ]; then
    echo "SSL Certificate parser."

    echo
    echo \
      "------------------------------------------------------------------------"
    echo "Flags:"
    echo \
      " -h  --help         | Prints out this screen."
    echo \
      " -f  --force        | Skips the check if there are already existing"
    echo \
      "                    | certificates."
    echo \
      "------------------------------------------------------------------------"
    echo

    exit 0
  fi
  if [ "$arg" == "--force" ] || [ "$arg" == "-f" ]; then
    FORCE=true
  fi
done

# https://stackoverflow.com/questions/59895/how-do-i-get-the-directory-where-a-bash-script-is-located-from-within-the-script/246128#246128
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Not really needed but just in case.
mkdir -p $SCRIPT_DIR/cert

# Verify if there's already a certificate.

cd $SCRIPT_DIR/cert

if [ -f key.pem ] && [ -f cert.pem ] && [ "$FORCE" = false ]; then
  echo "Already existing key and cert."
  exit 0
fi

rm -f key.pem
rm -f cert.pem

# Read and export all the .env file keys.

ENV_FILE="$SCRIPT_DIR/.env"

if [ ! -f "$ENV_FILE" ]; then
  echo "No env file found in '$ENV_FILE'. Making one..."
  touch $ENV_FILE
fi

# https://stackoverflow.com/questions/43267413/how-to-set-environment-variables-from-env-file/43267603#43267603
while IFS='=' read -r key value; do
    printf -v "$key" %s "$value" && export "$key"
done < "$ENV_FILE"

# Generate SSL certificate and key.
# https://actix.rs/docs/server#tls--https

openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
	-days 365 -sha256 -subj "/C=BG" \
  -passout pass:$SSL_PASSWORD
