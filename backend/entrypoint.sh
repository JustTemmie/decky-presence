#!/bin/sh
set -e

echo "Container's IP address: `awk 'END{print $1}' /etc/hosts`"

cd /backend

cargo build --release
mkdir -p ./out
cp target/release/backend ./out/
