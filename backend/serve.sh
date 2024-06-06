#!/bin/sh

# https://stackoverflow.com/questions/59895/how-do-i-get-the-directory-where-a-bash-script-is-located-from-within-the-script/246128#246128
export SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )

# Not really needed but just in case.
mkdir -p $SCRIPT_DIR/cert

cd $SCRIPT_DIR/cert

if [ -f key.pem ] && [ -f cert.pem ] && [ -f nopass.pem ]; then
  echo "Already existing key and cert."
  exit 0
fi

rm -f key.pem
rm -f cert.pem
rm -f nopass.pem

# Set up a random pasword for the OpenSSL TLS.
password=$(< /dev/urandom tr -dc 'a-zA-Z0-9_!@#$%^&*' | head -c 420)

openssl req -x509 -newkey rsa:4096 -keyout key.pem -out cert.pem \
	-days 365 -sha256 -subj "/C=BG" \
  -passout pass:${password}

# WARNINr: I don't reccomend doing this in more important repositories!
openssl rsa -in key.pem -out nopass.pem -passin pass:${password}
